<script setup lang="ts">
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { ElMessage, ElMessageBox } from "element-plus";
import { computed, onMounted, onUnmounted, ref, watch } from "vue";
import PlayerFooter from "./components/PlayerFooter.vue";
import PlaylistView from "./components/PlaylistView.vue";
import SettingsView from "./views/SettingsView.vue";
import SidebarMenu from "./components/SidebarMenu.vue";
import UploadSongDialog, { type UploadSongDraft } from "./components/UploadSongDialog.vue";
import WindowControls from "./components/WindowControls.vue";
import BatchUploadDialog, { type BatchUploadItem } from "./components/BatchUploadDialog.vue";
import { parseAudioMetadata } from "./utils/audioMetadata.ts";
import type { AppSettings, Collection, Track } from "./types/music";

const DEFAULT_SETTINGS: AppSettings = {
  serverIp: "127.0.0.1",
  serverPort: "3000",
  volume: 80,
  playbackOrder: "loop",
  closeWindowBehavior: "minimize",
};

const tracks = ref<Track[]>([]);
const currentTrackIndex = ref(0);
const activeTrackIndex = ref(0);
const collections = ref<Collection[]>([]);
const selectedCollectionId = ref<number | null>(null);
const currentPlaylistName = computed(() => collections.value.find(
  (collection) => collection.id === selectedCollectionId.value,
)?.collectionName ?? "收藏夹");
const isPlaying = ref(false);
const currentTime = ref(0);
const duration = ref(0);
const volume = ref(80);
const previousVolume = ref(80);
const audioRef = ref<HTMLAudioElement | null>(null);
const playbackOrder = ref<'loop' | 'one' | 'shuffle'>('loop');

// shuffle queue state: permutation of indices when in shuffle mode
const shuffleQueue = ref<number[]>([]);
const shufflePos = ref(0);
const currentPage = ref(window.location.hash === "#/settings" ? "settings" : "playlist");
const isSavingSettings = ref(false);
function syncPage() {
  currentPage.value = window.location.hash === "#/settings" ? "settings" : "playlist";
  if (currentPage.value === "playlist") {
    void flushSettings().then(() => {
      if (playlistNeedsRefresh) {
        playlistNeedsRefresh = false;
        void fetchCollections();
      }
    });
  }
}
const savedSettings = ref<AppSettings>({ ...DEFAULT_SETTINGS });
const isLoadingTracks = ref(false);
const trackLoadError = ref("");
const deletingTrackIds = ref<number[]>([]);
const downloadingTrackIds = ref<number[]>([]);
const uploadFileInputRef = ref<HTMLInputElement | null>(null);
const isUploadDialogOpen = ref(false);
const selectedUploadFile = ref<File | null>(null);
const isUploadingSong = ref(false);
const isBatchDialogOpen = ref(false);
const batchProgress = ref({ running: false, completed: 0, total: 0, failed: 0, current: "", errors: [] as string[] });
const isUploadProgressOpen = ref(false);

function openBatchUpload() {
  if (isUploadingSong.value || batchProgress.value.running) return;
  closeUploadDialog();
  isBatchDialogOpen.value = true;
}

async function uploadBatch(items: BatchUploadItem[]) {
  const collectionId = selectedCollectionId.value;
  if (collectionId === null || batchProgress.value.running || !items.length) return;
  isBatchDialogOpen.value = false;
  isUploadProgressOpen.value = false;
  batchProgress.value = { running: true, completed: 0, total: items.length, failed: 0, current: "", errors: [] };
  for (const item of items) {
    batchProgress.value.current = item.title;
    try {
      const coverUrl = await invoke<string>("fetch_cover_url", { musicName: item.title, artist: item.artist });
      const fileBytes = Array.from(new Uint8Array(await item.file.arrayBuffer()));
      await invoke("upload_music", { collectionId, fileName: item.file.name, fileBytes,
        musicName: item.title, artist: item.artist, coverUrl });
    } catch (error) {
      batchProgress.value.failed++;
      batchProgress.value.errors.push(`${item.title}：${String(error)}`);
    } finally {
      batchProgress.value.completed++;
    }
  }
  batchProgress.value.running = false;
  batchProgress.value.current = "";
  ElMessage({ type: batchProgress.value.failed ? "warning" : "success",
    message: `批量上传完成：成功 ${items.length - batchProgress.value.failed} 首，失败 ${batchProgress.value.failed} 首` });
  if (selectedCollectionId.value === collectionId) await fetchTracks();
}
const uploadSongDraft = ref<UploadSongDraft>({
  fileName: "",
  filePath: "",
  title: "",
  artist: "",
  coverUrl: "",
});

const currentTrack = computed(() => tracks.value[currentTrackIndex.value]);
const progress = computed(() => (duration.value ? (currentTime.value / duration.value) * 100 : 0));
const searchQuery = ref("");
const filteredTracks = computed(() => {
  const query = searchQuery.value.trim().toLowerCase();
  const rows = tracks.value.map((track, index) => ({ track, originalIndex: index }));
  if (!query) return rows;
  return rows.filter(({ track }) => {
    return (
      track.title.toLowerCase().includes(query) ||
      track.artist.toLowerCase().includes(query)
    );
  });
});

function openSettings() {
  window.location.hash = "/settings";
}

function closeSettings() {
  void flushSettings();
  window.location.hash = "/";
}

function resetUploadSongDraft() {
  uploadSongDraft.value = {
    fileName: "",
    filePath: "",
    title: "",
    artist: "",
    coverUrl: "",
  };
  selectedUploadFile.value = null;
}

let settingsTimer: ReturnType<typeof setTimeout> | undefined;
let pendingSettings: AppSettings | null = null;
let settingsSaveChain: Promise<void> = Promise.resolve();
let playlistNeedsRefresh = false;

function saveSettings(settings: AppSettings) {
  pendingSettings = { ...settings };
  clearTimeout(settingsTimer);
  settingsTimer = setTimeout(() => { void flushSettings(); }, 400);
}

function flushSettings(): Promise<void> {
  clearTimeout(settingsTimer);
  settingsTimer = undefined;
  const pending = pendingSettings;
  pendingSettings = null;
  if (!pending) return settingsSaveChain;
  // Serialize writes so an older request can never overwrite the latest edit.
  settingsSaveChain = settingsSaveChain.then(async () => {
    isSavingSettings.value = true;
    const nextSettings: AppSettings = {
      ...pending,
      volume: volume.value,
      playbackOrder: playbackOrder.value,
      closeWindowBehavior: pending.closeWindowBehavior || "minimize",
    };
    try {
      await invoke("save_settings", { settings: nextSettings });
      playlistNeedsRefresh ||= nextSettings.serverIp !== savedSettings.value.serverIp
        || nextSettings.serverPort !== savedSettings.value.serverPort
        || nextSettings.downloadDirectory !== savedSettings.value.downloadDirectory;
      savedSettings.value = nextSettings;
    } catch (error) {
      ElMessage.error(typeof error === "string" ? error : "自动保存失败，请重新修改后重试");
    } finally {
      isSavingSettings.value = false;
    }
  });
  return settingsSaveChain;
}

let playbackRequest = 0;
async function applyTrack(track: Track | undefined) {
  if (!audioRef.value || !track) return;

  const request = ++playbackRequest;
  track.localPath = await invoke<string | null>("resolve_local_music", { id: track.id, src: track.src, title: track.title, artist: track.artist }).catch(() => null);
  if (request !== playbackRequest || !audioRef.value) return;
  const source = track.localPath ? convertFileSrc(track.localPath) : track.src;
  if (audioRef.value.getAttribute("src") === source) return;
  audioRef.value.src = source;
  audioRef.value.load();
  currentTime.value = 0;
  duration.value = 0;
}

function loadTrack() {
  return applyTrack(currentTrack.value);
}

async function playPause() {
  if (!audioRef.value || !tracks.value.length) return;

  if (isPlaying.value) {
    audioRef.value.pause();
  } else {
    await applyTrack(currentTrack.value);
    void audioRef.value.play();
  }
}

function prevTrack() {
  if (!tracks.value.length) return;
  if (playbackOrder.value === 'shuffle') {
    if (!shuffleQueue.value.length) ensureShuffleQueue();
    shufflePos.value = (shufflePos.value - 1 + shuffleQueue.value.length) % shuffleQueue.value.length;
    const idx = shuffleQueue.value[shufflePos.value];
    currentTrackIndex.value = idx;
    activeTrackIndex.value = idx;
    return;
  }

  currentTrackIndex.value = (currentTrackIndex.value - 1 + tracks.value.length) % tracks.value.length;
  activeTrackIndex.value = currentTrackIndex.value;
}

function nextTrack() {
  if (!tracks.value.length) return;
  if (playbackOrder.value === 'shuffle') {
    if (!shuffleQueue.value.length) ensureShuffleQueue();
    shufflePos.value = (shufflePos.value + 1) % shuffleQueue.value.length;
    const idx = shuffleQueue.value[shufflePos.value];
    currentTrackIndex.value = idx;
    activeTrackIndex.value = idx;
    return;
  }

  currentTrackIndex.value = (currentTrackIndex.value + 1) % tracks.value.length;
  activeTrackIndex.value = currentTrackIndex.value;
}

function selectTrack(index: number) {
  activeTrackIndex.value = index;
}

function playTrack(index: number) {
  if (index === currentTrackIndex.value) {
    void applyTrack(currentTrack.value).then(() => audioRef.value?.play());
  }
  activeTrackIndex.value = index;
  currentTrackIndex.value = index;
  isPlaying.value = true;
  if (playbackOrder.value === 'shuffle') {
    if (!shuffleQueue.value.length) ensureShuffleQueue();
    const pos = shuffleQueue.value.indexOf(index);
    shufflePos.value = pos >= 0 ? pos : 0;
  }
}

async function downloadTrack(index: number) {
  const track = tracks.value[index];
  if (!track || downloadingTrackIds.value.includes(track.id)) return;
  if (deleteConfirmationOpen || deletingTrackIds.value.includes(track.id)) return;
  downloadingTrackIds.value.push(track.id);
  try {
    const localPath = await invoke<string>("download_music", { id: track.id, src: track.src, title: track.title, artist: track.artist });
    const latest = tracks.value.find(item => item.id === track.id && item.src === track.src);
    if (latest) latest.localPath = localPath;
    ElMessage.success("歌曲已下载");
  } catch (error) {
    ElMessage.error(typeof error === "string" ? error : "下载失败，请稍后重试");
  } finally {
    downloadingTrackIds.value = downloadingTrackIds.value.filter(id => id !== track.id);
  }
}

const isBatchDownloading = ref(false);
const batchDownloadProgress = ref({ completed: 0, total: 0 });
async function downloadTracks(ids: number[]) {
  if (isBatchDownloading.value || deleteConfirmationOpen || isDeletingTracks.value) return;
  const selected = tracks.value.filter(track => ids.includes(track.id)
    && !downloadingTrackIds.value.includes(track.id) && !deletingTrackIds.value.includes(track.id));
  if (!selected.length) return;
  isBatchDownloading.value = true;
  batchDownloadProgress.value = { completed: 0, total: selected.length };
  downloadingTrackIds.value.push(...selected.map(track => track.id));
  let succeeded = 0;
  const errors: string[] = [];
  try {
    for (const track of selected) {
      try {
        const localPath = await invoke<string>("download_music", {
          id: track.id, src: track.src, title: track.title, artist: track.artist,
        });
        const latest = tracks.value.find(item => item.id === track.id && item.src === track.src);
        if (latest) latest.localPath = localPath;
        succeeded++;
      } catch (error) {
        errors.push(`${track.title}：${String(error)}`);
      } finally {
        batchDownloadProgress.value.completed++;
      }
    }
    ElMessage({ message: [`批量下载完成：成功 ${succeeded} 首，失败 ${errors.length} 首`, ...errors].join("；"),
      type: errors.length ? "warning" : "success", duration: errors.length ? 6000 : 2500, showClose: true });
  } finally {
    const selectedIds = new Set(selected.map(track => track.id));
    downloadingTrackIds.value = downloadingTrackIds.value.filter(id => !selectedIds.has(id));
    isBatchDownloading.value = false;
  }
}

let deleteConfirmationOpen = false;
const isDeletingTracks = ref(false);
async function deleteTrack(index: number) {
  const track = tracks.value[index];
  if (track) await deleteTracks([track.id]);
}

async function deleteTracks(ids: number[]) {
  if (deleteConfirmationOpen || isDeletingTracks.value) return;
  const selected = tracks.value.filter(track => ids.includes(track.id));
  if (!selected.length) return;
  if (selected.some(track => downloadingTrackIds.value.includes(track.id))) {
    ElMessage.warning("所选歌曲正在下载，请下载完成后再删除");
    return;
  }
  if (selected.some(track => deletingTrackIds.value.includes(track.id))) return;
  deleteConfirmationOpen = true;
  try {
    const description = selected.length === 1 ? `歌曲「${selected[0].title}」` : `所选的 ${selected.length} 首歌曲`;
    await ElMessageBox.confirm(`确定删除${description}吗？已下载的本地文件也会一并删除。`, "删除歌曲", {
      confirmButtonText: "确认删除", cancelButtonText: "取消", type: "warning", closeOnClickModal: false,
    });
  } catch {
    return;
  } finally {
    deleteConfirmationOpen = false;
  }
  isDeletingTracks.value = true;
  deletingTrackIds.value.push(...selected.map(track => track.id));
  let succeeded = 0;
  const errors: string[] = [];
  const warnings: string[] = [];
  try {
    for (const track of selected) {
      const isCurrent = currentTrack.value?.id === track.id;
      const wasPlaying = isCurrent && isPlaying.value;
      if (isCurrent && audioRef.value) {
        ++playbackRequest;
        audioRef.value.pause();
        audioRef.value.removeAttribute("src");
        audioRef.value.load();
      }
      try {
        const warning = await invoke<string | null>("delete_music", { id: track.id, src: track.src });
        succeeded++;
        if (warning) warnings.push(`${track.title}：${warning}`);
        if (isCurrent) isPlaying.value = false;
      } catch (error) {
        errors.push(`${track.title}：${String(error)}`);
        if (wasPlaying && currentTrack.value?.id === track.id) {
          await applyTrack(track);
          void audioRef.value?.play().catch(() => {});
        }
      }
    }
    await fetchTracks();
    const result = selected.length === 1
      ? (succeeded ? "歌曲已删除" : "删除失败")
      : `批量删除完成：成功 ${succeeded} 首，失败 ${errors.length} 首`;
    ElMessage({ message: [result, ...errors, ...warnings].join("；"),
      type: errors.length || warnings.length ? "warning" : "success",
      duration: errors.length || warnings.length ? 6000 : 2500, showClose: true });
  } finally {
    const selectedIds = new Set(selected.map(track => track.id));
    deletingTrackIds.value = deletingTrackIds.value.filter(id => !selectedIds.has(id));
    isDeletingTracks.value = false;
  }
}
function setPlaybackOrder(order: 'loop' | 'one' | 'shuffle') {
  playbackOrder.value = order;
  if (order === 'shuffle') {
    ensureShuffleQueue();
    const pos = shuffleQueue.value.indexOf(currentTrackIndex.value);
    shufflePos.value = pos >= 0 ? pos : 0;
  }
}

function ensureShuffleQueue() {
  const n = tracks.value.length;
  if (n === 0) {
    shuffleQueue.value = [];
    shufflePos.value = 0;
    return;
  }
  const arr = Array.from({ length: n }, (_, i) => i);
  for (let i = n - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    const tmp = arr[i];
    arr[i] = arr[j];
    arr[j] = tmp;
  }
  shuffleQueue.value = arr;
  const pos = shuffleQueue.value.indexOf(currentTrackIndex.value);
  shufflePos.value = pos >= 0 ? pos : 0;
}

function seek(value: number) {
  if (!audioRef.value || !duration.value) return;
  audioRef.value.currentTime = (value / 100) * duration.value;
}

function setVolume(value: number) {
  const nextVolume = Math.min(100, Math.max(0, value));
  volume.value = nextVolume;

  if (nextVolume > 0) {
    previousVolume.value = nextVolume;
  }

  if (audioRef.value) {
    audioRef.value.volume = nextVolume / 100;
  }
}

function toggleMute() {
  if (volume.value === 0) {
    setVolume(previousVolume.value || 80);
    return;
  }

  previousVolume.value = volume.value;
  setVolume(0);
}

function openUploadPicker() {
  if (batchProgress.value.running) {
    showUploadAlert("批量上传正在后台执行，请等待当前任务完成");
    return;
  }
  if (selectedCollectionId.value === null) {
    showUploadAlert("请先选择收藏夹");
    return;
  }
  isUploadDialogOpen.value = true;
}

function reselectUploadFile() {
  if (isUploadingSong.value) return;
  uploadFileInputRef.value?.click();
}

function closeUploadDialog(force = false) {
  if (isUploadingSong.value && !force) return;

  isUploadDialogOpen.value = false;
  resetUploadSongDraft();

  if (uploadFileInputRef.value) {
    uploadFileInputRef.value.value = "";
  }
}

function updateUploadSongDraft(draft: UploadSongDraft) {
  uploadSongDraft.value = draft;
}

function showUploadAlert(message: string) {
  ElMessage({
    message,
    type: "warning",
    duration: 2600,
    showClose: true,
    grouping: false,
    offset: 24,
    zIndex: 20050,
  });
}

function parseUploadFileName(fileName: string) {
  const baseName = fileName.replace(/\.(mp3|ogg)$/i, "").trim();
  const parts = baseName.split("-");

  if (parts.length < 2) {
    return {
      title: baseName,
      artist: "",
    };
  }

  const title = parts[0]?.trim() ?? "";
  const artist = parts.slice(1).join("-").trim();

  return {
    title: title || baseName,
    artist,
  };
}

async function handleUploadFileChange(event: Event) {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0];

  if (!file) return;

  if (!file.name.toLowerCase().endsWith(".mp3") && !file.name.toLowerCase().endsWith(".ogg")) {
    input.value = "";
    showUploadAlert("请选择 MP3 或 OGG 格式的音乐文件");
    return;
  }

  const fallbackName = file.name.replace(/\.(mp3|ogg)$/i, "");
  const metadata = await parseAudioMetadata(file).catch((error: unknown) => {
    console.error("Failed to parse audio metadata", error);
    return { title: "", artist: "" };
  });
  const fallback = parseUploadFileName(file.name);
  const hasMetadata = Boolean(metadata.title || metadata.artist);

  uploadSongDraft.value = {
    fileName: file.name,
    filePath: file.name,
    title: metadata.title || fallback.title || fallbackName,
    artist: metadata.artist || (hasMetadata ? "" : fallback.artist),
    coverUrl: "",
  };
  selectedUploadFile.value = file;
  isUploadDialogOpen.value = true;
}

async function handleFetchCover() {
  const title = uploadSongDraft.value.title.trim();
  const artist = uploadSongDraft.value.artist.trim();

  try {
    const coverUrl = await invoke<string>("fetch_cover_url", {
      musicName: title,
      artist,
    });

    if (coverUrl) {
      uploadSongDraft.value = {
        ...uploadSongDraft.value,
        coverUrl,
      };
    }
  } catch (error) {
    console.error("Failed to fetch cover url", error);
    showUploadAlert(error instanceof Error ? error.message : "出错请稍后再试");
  }
}

async function handleUploadSong() {
  const collectionId = selectedCollectionId.value;
  if (collectionId === null) {
    showUploadAlert("请先选择收藏夹");
    return;
  }
  if (!selectedUploadFile.value) {
    showUploadAlert("请选择要上传的音乐文件");
    return;
  }

  isUploadingSong.value = true;

  try {
    const fileBytes = Array.from(new Uint8Array(await selectedUploadFile.value.arrayBuffer()));

    await invoke("upload_music", {
      collectionId,
      fileName: selectedUploadFile.value.name,
      fileBytes,
      musicName: uploadSongDraft.value.title.trim(),
      artist: uploadSongDraft.value.artist.trim(),
      coverUrl: uploadSongDraft.value.coverUrl.trim(),
    });

    isUploadingSong.value = false;
    closeUploadDialog(true);

    ElMessage({
      message: "上传成功",
      type: "success",
      duration: 2200,
      showClose: true,
      grouping: false,
      offset: 24,
      zIndex: 20050,
    });
    await fetchTracks();
  } catch (error) {
    console.error("Failed to upload music", error);
    isUploadingSong.value = false;
    ElMessage({
      message: error instanceof Error && error.message ? error.message : "上传失败",
      type: "error",
      duration: 3000,
      showClose: true,
      grouping: false,
      offset: 24,
      zIndex: 20050,
    });
  }
}

let trackListRequest = 0;

function clearTracks() {
  ++playbackRequest;
  audioRef.value?.pause();
  audioRef.value?.removeAttribute("src");
  audioRef.value?.load();
  tracks.value = [];
  currentTrackIndex.value = 0;
  activeTrackIndex.value = 0;
  currentTime.value = 0;
  duration.value = 0;
  isPlaying.value = false;
  shuffleQueue.value = [];
}

async function fetchCollections() {
  const request = ++trackListRequest;
  isLoadingTracks.value = true;
  trackLoadError.value = "";
  clearTracks();
  collections.value = [];
  selectedCollectionId.value = null;
  try {
    const loaded = await invoke<Collection[]>("fetch_collections");
    if (request !== trackListRequest) return;
    collections.value = loaded;
    selectedCollectionId.value = loaded[0]?.id ?? null;
    await fetchTracks();
  } catch (error) {
    if (request !== trackListRequest) return;
    console.error("Failed to load collections", error);
    trackLoadError.value = "收藏夹加载失败，请检查服务器设置后刷新重试。";
  } finally {
    if (request === trackListRequest) isLoadingTracks.value = false;
  }
}

async function openCollection(collectionId: number) {
  await flushSettings();
  const needsRefresh = playlistNeedsRefresh;
  playlistNeedsRefresh = false;
  closeSettings();
  if (needsRefresh) {
    await fetchCollections();
    return;
  }
  if (selectedCollectionId.value === collectionId) return;
  selectedCollectionId.value = collectionId;
  clearTracks();
  await fetchTracks();
}

async function refreshPlaylist() {
  if (!collections.value.length) await fetchCollections();
  else await fetchTracks();
}

async function fetchTracks() {
  const request = ++trackListRequest;
  const collectionId = selectedCollectionId.value;
  if (collectionId === null) {
    clearTracks();
    isLoadingTracks.value = false;
    return;
  }
  isLoadingTracks.value = true;
  trackLoadError.value = "";
  const previousTrackId = currentTrack.value?.id;

  try {
    const loadedTracks = await invoke<Track[]>("fetch_music_list", { collectionId });
    if (request !== trackListRequest) return;
    tracks.value = loadedTracks;

    if (playbackOrder.value === 'shuffle') ensureShuffleQueue();

    if (!loadedTracks.length) {
      clearTracks();
      return;
    }

    if (previousTrackId != null) {
      const preservedIndex = loadedTracks.findIndex((track) => track.id === previousTrackId);
      if (preservedIndex !== -1) {
        currentTrackIndex.value = preservedIndex;
      } else if (currentTrackIndex.value >= loadedTracks.length) {
        currentTrackIndex.value = 0;
      }
    } else if (currentTrackIndex.value >= loadedTracks.length) {
      currentTrackIndex.value = 0;
    }

    activeTrackIndex.value = currentTrackIndex.value;
    const nextTrack = loadedTracks[currentTrackIndex.value];
    const isSameTrack = previousTrackId != null && nextTrack.id === previousTrackId;

    if (!isSameTrack) {
      await applyTrack(nextTrack);
    }

    if (isPlaying.value) {
      await audioRef.value?.play();
    }
  } catch (error) {
    if (request !== trackListRequest) return;
    console.error("Failed to load music list", error);
    clearTracks();
    trackLoadError.value = "音乐列表加载失败，请检查服务器设置。";
  } finally {
    if (request === trackListRequest) isLoadingTracks.value = false;
  }
}

watch(currentTrackIndex, async () => {
  const shouldPlay = isPlaying.value;
  await loadTrack();
  if (shouldPlay) {
    void audioRef.value?.play();
  }
});

onMounted(() => {
  window.addEventListener("hashchange", syncPage);
  void (async () => {
    try {
      const loaded = await invoke<AppSettings>("load_settings");
      savedSettings.value = {
        downloadDirectory: loaded.downloadDirectory || "",
        serverIp: loaded.serverIp || DEFAULT_SETTINGS.serverIp,
        serverPort: loaded.serverPort || DEFAULT_SETTINGS.serverPort,
        volume: loaded.volume ?? DEFAULT_SETTINGS.volume,
        playbackOrder: loaded.playbackOrder || DEFAULT_SETTINGS.playbackOrder,
        closeWindowBehavior: loaded.closeWindowBehavior || DEFAULT_SETTINGS.closeWindowBehavior,
      };
      
      // 应用加载的音量和播放顺序
      volume.value = savedSettings.value.volume ?? 80;
      playbackOrder.value = savedSettings.value.playbackOrder ?? "loop";
    } catch (error) {
      console.error("Failed to load settings", error);
      savedSettings.value = { ...DEFAULT_SETTINGS };
      volume.value = DEFAULT_SETTINGS.volume ?? 80;
      playbackOrder.value = DEFAULT_SETTINGS.playbackOrder ?? "loop";
    }

    await fetchCollections();
  })();

  if (!audioRef.value) return;

  audioRef.value.volume = volume.value / 100;

  audioRef.value.addEventListener("timeupdate", () => {
    currentTime.value = audioRef.value?.currentTime ?? 0;
  });
  audioRef.value.addEventListener("loadedmetadata", () => {
    duration.value = audioRef.value?.duration ?? 0;
  });
  audioRef.value.addEventListener("play", () => {
    isPlaying.value = true;
  });
  audioRef.value.addEventListener("pause", () => {
    isPlaying.value = false;
  });
  audioRef.value.addEventListener("ended", () => {
    if (playbackOrder.value === 'one') {
      if (!audioRef.value) return;
      audioRef.value.currentTime = 0;
      isPlaying.value = true;
      void audioRef.value.play();
      return;
    }

    const previousIndex = currentTrackIndex.value;
    isPlaying.value = true;
    nextTrack();
    if (previousIndex === currentTrackIndex.value) {
      void applyTrack(currentTrack.value).then(() => audioRef.value?.play());
    }
  });
});

onUnmounted(() => {
  window.removeEventListener("hashchange", syncPage);
  if (!audioRef.value) return;
  audioRef.value.pause();
  audioRef.value.src = "";
});
</script>

<template>
  <div class="app-shell">
    <div class="app-content">
      <div class="titlebar">
        <div class="titlebar-left"></div>
        <div class="titlebar-center">
          <input
            type="search"
            placeholder="查找音乐..."
            class="titlebar-search-input"
            v-model="searchQuery"
          />
        </div>
        <div class="titlebar-right">
          <WindowControls @open-settings="openSettings" />
        </div>
      </div>

      <div class="content-grid">
        <div class="content-row">
          <SidebarMenu :active="currentPage === 'playlist'" :collections="collections"
            :selectedCollectionId="selectedCollectionId" @open-playlist="openCollection" />
          <main class="main-panel">
            <SettingsView v-if="currentPage === 'settings'" :initialSettings="savedSettings" :saving="isSavingSettings" @save="saveSettings" @cancel="closeSettings" />
            <PlaylistView v-else
              :tracks="filteredTracks"
              :activeTrackIndex="activeTrackIndex"
              :currentTrackIndex="currentTrackIndex"
              :isPlaying="isPlaying"
              :playlistName="currentPlaylistName"
              :loading="isLoadingTracks"
              :errorMessage="trackLoadError"
              :deletingTrackIds="deletingTrackIds"
              :deleting="isDeletingTracks"
              :batchDownloading="isBatchDownloading"
              :downloadProgress="`${batchDownloadProgress.completed} / ${batchDownloadProgress.total}`"
              :collectionId="selectedCollectionId"
              @batch-delete="deleteTracks"
              @batch-download="downloadTracks"
              :downloadingTrackIds="downloadingTrackIds"
              :batchUploading="batchProgress.running"
              :hasUploadProgress="batchProgress.total > 0"
              :uploadProgressOpen="isUploadProgressOpen"
              @toggle-upload-progress="isUploadProgressOpen = !isUploadProgressOpen"
              @download="downloadTrack"
              @select="selectTrack"
              @play="playTrack"
              @delete="deleteTrack"
              @refresh="refreshPlaylist"
              @upload="openUploadPicker"
            >
              <template #upload-progress>
    <aside v-if="batchProgress.total && isUploadProgressOpen" id="batch-upload-progress" class="batch-progress" role="status" aria-live="polite">
      <div>{{ batchProgress.running ? '正在上传' : '上传完成' }} {{ batchProgress.completed }} / {{ batchProgress.total }}
        <span v-if="batchProgress.failed">（失败 {{ batchProgress.failed }} 首）</span>
      </div>
      <progress :value="batchProgress.completed" :max="batchProgress.total"></progress>
      <div v-if="batchProgress.current">{{ batchProgress.current }}</div>
      <details v-if="batchProgress.errors.length"><summary>查看失败原因</summary>
        <p v-for="(error, index) in batchProgress.errors" :key="index">{{ error }}</p>
      </details>
    </aside>
              </template>
            </PlaylistView>
          </main>
        </div>
        <PlayerFooter
          :currentTrack="currentTrack"
          :isPlaying="isPlaying"
          :currentTime="currentTime"
          :duration="duration"
          :progress="progress"
          :volume="volume"
          :playbackOrder="playbackOrder"
          @prev="prevTrack"
          @playPause="playPause"
          @next="nextTrack"
          @seek="seek"
          @set-volume="setVolume"
          @toggle-mute="toggleMute"
          @set-playback-order="setPlaybackOrder"
        />
      </div>
    </div>

    <UploadSongDialog
      :open="isUploadDialogOpen"
      :draft="uploadSongDraft"
      :isUploading="isUploadingSong"
      @update:draft="updateUploadSongDraft"
      @cancel="closeUploadDialog"
      @fetch-cover="handleFetchCover"
      @upload="handleUploadSong"
      @reselect-file="reselectUploadFile"
      @batch="openBatchUpload"
    />

    <BatchUploadDialog :open="isBatchDialogOpen" @cancel="isBatchDialogOpen = false"
      @single="isBatchDialogOpen = false; isUploadDialogOpen = true" @upload="uploadBatch" />


    <input
      ref="uploadFileInputRef"
      type="file"
      accept=".mp3,.ogg,audio/mpeg,audio/ogg"
      class="hidden-upload-input"
      @change="handleUploadFileChange"
    />

    <audio ref="audioRef" preload="metadata"></audio>
  </div>
</template>

<style scoped>
.batch-progress {

  max-height: 45vh; overflow: auto; padding: .25rem;
  background: var(--theme-panel, #fff); color: var(--theme-text, #334155);

  font-size: .85rem; overflow-wrap: anywhere;
  -webkit-app-region: no-drag;
}
.batch-progress progress { width: 100%; margin: .6rem 0; accent-color: #818cf8; }
.app-shell {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  overflow: hidden;
  border-radius: 1rem;
  background: radial-gradient(circle at top, rgba(211, 172, 73, 0), transparent 0%),
    radial-gradient(circle at bottom right, rgba(16, 185, 129, 0), transparent 0%),
    #f5f6fb;
  background-clip: padding-box;
}

.app-content {
  position: relative;
  width: 100%;
  height: 100%;
  padding: 0rem 0;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.content-grid {
  display: grid;
  grid-template-rows: 1fr auto;
  gap: 0;
  flex: 1;
  min-height: 0;
  height: 100%;
  overflow: hidden;
}

.content-row {
  display: grid;
  grid-template-columns: 260px minmax(0, 1fr);
  gap: 0;
  min-height: 0;
  overflow: hidden;
}

.main-panel {
  display: flex;
  flex-direction: column;
  gap: 0;
  min-height: 0;
  height: 100%;
  overflow: hidden;
}


.titlebar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.2rem 1rem 0.35rem;
  background: transparent;
  border: none;
  -webkit-app-region: drag;
  gap: 0;
}

.titlebar-left {
  width: 260px;
  flex: 0 0 260px;
  -webkit-app-region: drag;
}

.titlebar-center {
  flex: 1;
  display: flex;
  justify-content: flex-start;
  -webkit-app-region: drag;
}

.titlebar-right {
  width: 10.5rem;
  flex: 0 0 10.5rem;
  display: flex;
  justify-content: flex-end;
  -webkit-app-region: no-drag;
}

.titlebar-search-input {
  width: min(460px, 100%);
  max-width: 100%;
  border: 1px solid rgba(15, 23, 42, 0.12);
  border-radius: 9999px;
  padding: 0.48rem 0.9rem;
  background: rgba(247, 250, 252, 0.95);
  color: #374151;
  -webkit-app-region: no-drag;
  transition: border-color 0.2s ease, box-shadow 0.2s ease, background 0.2s ease;
}

.titlebar-search-input:focus {
  outline: none;
  border-color: rgba(99, 102, 241, 0.35);
  box-shadow: 0 0 0 4px rgba(99, 102, 241, 0.08);
  background: rgba(255, 255, 255, 1);
}

.hidden-upload-input {
  display: none;
}

@media (max-width: 1024px) {
  .titlebar-left {
    display: none;
  }

  .titlebar-right {
    width: auto;
    flex: 0 0 auto;
  }

  .titlebar-search-input {
    width: min(420px, 100%);
  }

  .content-row {
    grid-template-columns: 1fr;
  }
}
</style>

<style>
html,
body,
#app {
  width: 100%;
  height: 100%;
  margin: 0;
  padding: 0;
  background: transparent !important;
  background-color: transparent !important;
  overflow: hidden;
}

body,
#app {
  min-height: 100vh;
}

#app {
  overflow: hidden;
}

.app-shell {
  background-clip: padding-box;
}

* {
  margin: 0;
  padding: 0;
  border: 0;
}

</style>
