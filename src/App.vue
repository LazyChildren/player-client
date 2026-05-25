<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { computed, onMounted, onUnmounted, ref, watch } from "vue";
import PlayerFooter from "./components/PlayerFooter.vue";
import PlaylistView from "./components/PlaylistView.vue";
import SettingsDialog from "./components/SettingsDialog.vue";
import SidebarMenu from "./components/SidebarMenu.vue";
import WindowControls from "./components/WindowControls.vue";
import type { AppSettings, Track } from "./types/music";

const DEFAULT_SETTINGS: AppSettings = {
  serverIp: "127.0.0.1",
  serverPort: "3000",
};

const tracks = ref<Track[]>([]);
const currentTrackIndex = ref(0);
const activeTrackIndex = ref(0);
const currentPlaylistName = ref("默认收藏夹");
const isPlaying = ref(false);
const currentTime = ref(0);
const duration = ref(0);
const audioRef = ref<HTMLAudioElement | null>(null);
const isSettingsOpen = ref(false);
const savedSettings = ref<AppSettings>({ ...DEFAULT_SETTINGS });
const isLoadingTracks = ref(false);
const trackLoadError = ref("");

const currentTrack = computed(() => tracks.value[currentTrackIndex.value]);
const progress = computed(() => (duration.value ? (currentTime.value / duration.value) * 100 : 0));

function openSettings() {
  isSettingsOpen.value = true;
}

function closeSettings() {
  isSettingsOpen.value = false;
}

async function saveSettings(settings: AppSettings) {
  savedSettings.value = {
    serverIp: settings.serverIp,
    serverPort: settings.serverPort,
  };

  try {
    await invoke("save_settings", {
      settings: {
        server_ip: savedSettings.value.serverIp,
        server_port: savedSettings.value.serverPort,
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
  currentTrackIndex.value = (currentTrackIndex.value - 1 + tracks.value.length) % tracks.value.length;
  activeTrackIndex.value = currentTrackIndex.value;
}

function nextTrack() {
  if (!tracks.value.length) return;
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
}

function seek(value: number) {
  if (!audioRef.value || !duration.value) return;
  audioRef.value.currentTime = (value / 100) * duration.value;
}

async function fetchTracks() {
  isLoadingTracks.value = true;
  trackLoadError.value = "";

  try {
    const loadedTracks = await invoke<Track[]>("fetch_music_list");
    tracks.value = loadedTracks;

    if (!loadedTracks.length) {
      currentTrackIndex.value = 0;
      activeTrackIndex.value = 0;
      isPlaying.value = false;
      return;
    }

    if (currentTrackIndex.value >= loadedTracks.length) {
      currentTrackIndex.value = 0;
    }

    activeTrackIndex.value = currentTrackIndex.value;
    applyTrack(loadedTracks[currentTrackIndex.value]);

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
      };
    } catch (error) {
      console.error("Failed to load settings", error);
      savedSettings.value = { ...DEFAULT_SETTINGS };
    }

    await fetchTracks();
  })();

  if (!audioRef.value) return;

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
  audioRef.value.addEventListener("ended", nextTrack);
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
          <input type="search" placeholder="查找音乐..." class="titlebar-search-input" />
        </div>
        <div class="titlebar-right">
          <WindowControls @open-settings="openSettings" />
        </div>
      </div>

      <div class="content-grid">
        <SidebarMenu />
        <main class="main-panel">
          <PlaylistView
            :tracks="tracks"
            :activeTrackIndex="activeTrackIndex"
            :currentTrackIndex="currentTrackIndex"
            :isPlaying="isPlaying"
            :playlistName="currentPlaylistName"
            :loading="isLoadingTracks"
            :errorMessage="trackLoadError"
            @select="selectTrack"
            @play="playTrack"
          />
          <PlayerFooter
            :currentTrack="currentTrack"
            :isPlaying="isPlaying"
            :currentTime="currentTime"
            :duration="duration"
            :progress="progress"
            @prev="prevTrack"
            @playPause="playPause"
            @next="nextTrack"
            @seek="seek"
          />
        </main>
      </div>
    </div>

    <SettingsDialog
      :open="isSettingsOpen"
      :initialSettings="savedSettings"
      @save="saveSettings"
      @cancel="closeSettings"
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
  z-index: 9999;
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
  padding: 0.5rem 0;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  overflow: hidden;
}

.content-grid {
  display: grid;
  grid-template-columns: 260px minmax(0, 1fr);
  gap: 0;
  flex: 1;
  min-height: 0;
  align-items: stretch;
}

.main-panel {
  display: flex;
  flex-direction: column;
  gap: 0;
  min-height: 0;
  padding-bottom: 0;
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
}

@media (max-width: 720px) {
  .titlebar {
    padding: 0.2rem 0.6rem 0.35rem;
    gap: 0.5rem;
  }

  .titlebar-center {
    justify-content: flex-start;
  }

  .titlebar-search-input {
    width: 100%;
  }

  .titlebar-right {
    margin-left: auto;
  }
}

@media (max-width: 560px) {
  .titlebar-center {
    min-width: 0;
  }

  .titlebar-search-input {
    min-width: 0;
    padding: 0.42rem 0.78rem;
  }

  .titlebar-right {
    width: auto;
  }
}

@media (max-width: 430px) {
  .titlebar {
    flex-wrap: wrap;
  }

  .titlebar-center,
  .titlebar-right {
    width: 100%;
    flex: 1 1 100%;
  }

  .titlebar-right {
    justify-content: flex-end;
  }

  .titlebar-center {
    order: 1;
  }

  .titlebar-right {
    order: 2;
  }

  .titlebar-search-input {
    width: 100%;
  }
}

@media (max-width: 1024px) {
  .content-grid {
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

.app-content .player-footer {
  position: absolute;
  bottom: 0;
  width: auto;
}
</style>
