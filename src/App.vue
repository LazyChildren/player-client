<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { ElMessage } from "element-plus";
import { computed, onMounted, onUnmounted, ref, watch } from "vue";
import PlayerFooter from "./components/PlayerFooter.vue";
import PlaylistView from "./components/PlaylistView.vue";
import SettingsDialog from "./components/SettingsDialog.vue";
import SidebarMenu from "./components/SidebarMenu.vue";
import UploadSongDialog, { type UploadSongDraft } from "./components/UploadSongDialog.vue";
import WindowControls from "./components/WindowControls.vue";
import { parseAudioMetadata } from "./utils/audioMetadata.ts";
import type { AppSettings, Track } from "./types/music";

const DEFAULT_SETTINGS: AppSettings = {
  serverIp: "127.0.0.1",
  serverPort: "3000",
  volume: 80,
  playbackOrder: "loop",
};

const tracks = ref<Track[]>([]);
const currentTrackIndex = ref(0);
const activeTrackIndex = ref(0);
const currentPlaylistName = ref("默认歌单");
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
const isSettingsOpen = ref(false);
const savedSettings = ref<AppSettings>({ ...DEFAULT_SETTINGS });
const isLoadingTracks = ref(false);
const trackLoadError = ref("");
const deletingTrackIds = ref<number[]>([]);
const uploadFileInputRef = ref<HTMLInputElement | null>(null);
const isUploadDialogOpen = ref(false);
const selectedUploadFile = ref<File | null>(null);
const isUploadingSong = ref(false);
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
  isSettingsOpen.value = true;
}

function closeSettings() {
  isSettingsOpen.value = false;
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

async function saveSettings(settings: AppSettings) {
  savedSettings.value = {
    serverIp: settings.serverIp,
    serverPort: settings.serverPort,
    volume: volume.value,
    playbackOrder: playbackOrder.value,
  };

  try {
    await invoke("save_settings", {
      settings: {
        server_ip: savedSettings.value.serverIp,
        server_port: savedSettings.value.serverPort,
        volume: savedSettings.value.volume,
        playback_order: savedSettings.value.playbackOrder,
      },
    });
    await fetchTracks();
  } catch (error) {
    console.error("Failed to save settings", error);
  }

  closeSettings();
}

function applyTrack(track: Track | undefined) {
  if (!audioRef.value || !track) return;

  audioRef.value.src = track.src;
  audioRef.value.load();
  currentTime.value = 0;
  duration.value = 0;
}

function loadTrack() {
  applyTrack(currentTrack.value);
}

function playPause() {
  if (!audioRef.value || !tracks.value.length) return;

  if (isPlaying.value) {
    audioRef.value.pause();
  } else {
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
  activeTrackIndex.value = index;
  currentTrackIndex.value = index;
  isPlaying.value = true;
  if (playbackOrder.value === 'shuffle') {
    if (!shuffleQueue.value.length) ensureShuffleQueue();
    const pos = shuffleQueue.value.indexOf(index);
    shufflePos.value = pos >= 0 ? pos : 0;
  }
}

async function deleteTrack(index: number) {
  if (index < 0 || index >= tracks.value.length) return;

  const track = tracks.value[index];
  const isCurrentTrackDeleted = index === currentTrackIndex.value;
  const wasCurrentPlaying = isCurrentTrackDeleted && isPlaying.value;
  const deletingId = track.id;

  if (deletingTrackIds.value.includes(deletingId)) return;
  deletingTrackIds.value.push(deletingId);

  try {
    if (wasCurrentPlaying && audioRef.value) {
      audioRef.value.pause();
    }

    await invoke("delete_music", { id: track.id });

    if (isCurrentTrackDeleted) {
      isPlaying.value = false;
    }

    await fetchTracks();

    ElMessage({
      message: "歌曲已删除",
      type: "success",
      duration: 2200,
      showClose: true,
      grouping: false,
      offset: 24,
      zIndex: 20050,
    });
  } catch (error) {
    if (wasCurrentPlaying && audioRef.value) {
      audioRef.value.play().catch(() => {});
    }
    console.error("Failed to delete music", error);
    ElMessage({
      message: error instanceof Error && error.message ? error.message : "删除失败",
      type: "error",
      duration: 3000,
      showClose: true,
      grouping: false,
      offset: 24,
      zIndex: 20050,
    });
  } finally {
    deletingTrackIds.value = deletingTrackIds.value.filter((id) => id !== deletingId);
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
  uploadFileInputRef.value?.click();
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
  const baseName = fileName.replace(/\.mp3$/i, "").trim();
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
  if (!selectedUploadFile.value) {
    showUploadAlert("请选择要上传的音乐文件");
    return;
  }

  isUploadingSong.value = true;

  try {
    const fileBytes = Array.from(new Uint8Array(await selectedUploadFile.value.arrayBuffer()));

    await invoke("upload_music", {
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

async function fetchTracks() {
  isLoadingTracks.value = true;
  trackLoadError.value = "";
  const previousTrackId = currentTrack.value?.id;

  try {
    const loadedTracks = await invoke<Track[]>("fetch_music_list");
    tracks.value = loadedTracks;

    if (playbackOrder.value === 'shuffle') ensureShuffleQueue();

    if (!loadedTracks.length) {
      currentTrackIndex.value = 0;
      activeTrackIndex.value = 0;
      isPlaying.value = false;
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
      applyTrack(nextTrack);
    }

    if (isPlaying.value) {
      await audioRef.value?.play();
    }
  } catch (error) {
    console.error("Failed to load music list", error);
    tracks.value = [];
    currentTrackIndex.value = 0;
    activeTrackIndex.value = 0;
    isPlaying.value = false;
    trackLoadError.value = "音乐列表加载失败，请检查服务器设置。";
  } finally {
    isLoadingTracks.value = false;
  }
}

watch(currentTrackIndex, () => {
  loadTrack();
  if (isPlaying.value) {
    void audioRef.value?.play();
  }
});

onMounted(() => {
  void (async () => {
    try {
      const loaded = await invoke<AppSettings>("load_settings");
      savedSettings.value = {
        serverIp: loaded.serverIp || DEFAULT_SETTINGS.serverIp,
        serverPort: loaded.serverPort || DEFAULT_SETTINGS.serverPort,
        volume: loaded.volume ?? DEFAULT_SETTINGS.volume,
        playbackOrder: loaded.playbackOrder || DEFAULT_SETTINGS.playbackOrder,
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

    await fetchTracks();
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

    // for shuffle and loop, advance and play
    nextTrack();
    // ensure state indicates playing so watch handlers also trigger play
    isPlaying.value = true;
    void audioRef.value?.play();
  });
});

onUnmounted(() => {
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
          <SidebarMenu />
          <main class="main-panel">
            <PlaylistView
              :tracks="filteredTracks"
              :activeTrackIndex="activeTrackIndex"
              :currentTrackIndex="currentTrackIndex"
              :isPlaying="isPlaying"
              :playlistName="currentPlaylistName"
              :loading="isLoadingTracks"
              :errorMessage="trackLoadError"
              :deletingTrackIds="deletingTrackIds"
              @select="selectTrack"
              @play="playTrack"
              @delete="deleteTrack"
              @refresh="fetchTracks"
              @upload="openUploadPicker"
            />
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

    <SettingsDialog
      :open="isSettingsOpen"
      :initialSettings="savedSettings"
      @save="saveSettings"
      @cancel="closeSettings"
    />

    <UploadSongDialog
      :open="isUploadDialogOpen"
      :draft="uploadSongDraft"
      :isUploading="isUploadingSong"
      @update:draft="updateUploadSongDraft"
      @cancel="closeUploadDialog"
      @fetch-cover="handleFetchCover"
      @upload="handleUploadSong"
      @reselect-file="reselectUploadFile"
    />

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
  gap: 0.5rem;
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
  width: 6.3rem;
  flex: 0 0 6.3rem;
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
