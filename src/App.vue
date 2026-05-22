<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from "vue";
import SidebarMenu from "./components/SidebarMenu.vue";
import PlaylistView from "./components/PlaylistView.vue";
import PlayerFooter from "./components/PlayerFooter.vue";
import WindowControls from "./components/WindowControls.vue";

type Track = {
  id: number;
  title: string;
  artist: string;
  album: string;
  duration: string;
  src: string;
  color: string;
};

const tracks = ref<Track[]>([
  {
    id: 1,
    title: "Moonlit Drive",
    artist: "Nova Echo",
    album: "Midnight Waves",
    duration: "3:42",
    src: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-1.mp3",
    color: "#6d83ff",
  },
  {
    id: 2,
    title: "Sunrise Beat",
    artist: "Aurora Pulse",
    album: "Daybreak",
    duration: "4:08",
    src: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-2.mp3",
    color: "#ff8a6f",
  },
  {
    id: 3,
    title: "Starlight Reverie",
    artist: "Luna Harbor",
    album: "Dreamscape",
    duration: "2:56",
    src: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-3.mp3",
    color: "#3fd2c7",
  },
]);

const currentTrackIndex = ref(0);
const activeTrackIndex = ref(0);
const currentPlaylistName = ref("默认收藏夹");
const isPlaying = ref(false);
const currentTime = ref(0);
const duration = ref(0);
const audioRef = ref<HTMLAudioElement | null>(null);

const currentTrack = computed(() => tracks.value[currentTrackIndex.value]);
const progress = computed(() => (duration.value ? (currentTime.value / duration.value) * 100 : 0));

function formatTime(seconds: number) {
  const minutes = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${minutes}:${secs.toString().padStart(2, "0")}`;
}
void formatTime;

function loadTrack() {
  if (!audioRef.value) return;
  audioRef.value.src = currentTrack.value.src;
  audioRef.value.load();
  currentTime.value = 0;
  duration.value = 0;
}

function playPause() {
  if (!audioRef.value) return;
  if (isPlaying.value) {
    audioRef.value.pause();
  } else {
    audioRef.value.play();
  }
}

function prevTrack() {
  currentTrackIndex.value = (currentTrackIndex.value - 1 + tracks.value.length) % tracks.value.length;
  activeTrackIndex.value = currentTrackIndex.value;
}

function nextTrack() {
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

watch(currentTrackIndex, () => {
  loadTrack();
  if (isPlaying.value) {
    audioRef.value?.play();
  }
});

onMounted(() => {
  loadTrack();
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
          <WindowControls />
        </div>
      </div>

      <div class="content-grid">
        <SidebarMenu />
        <main class="main-panel">
          <PlaylistView
            :tracks="tracks"
            :activeTrackIndex="activeTrackIndex"
            :playlistName="currentPlaylistName"
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
  background: radial-gradient(circle at top, rgba(211, 172, 73, 0.00), transparent 0%),
    radial-gradient(circle at bottom right, rgba(16, 185, 129, 0.00), transparent 0%),
    #f5f6fb;
  background-clip: padding-box;
}

.app-content {
  position: relative;
  width: 100%;
  height: 100%;
  padding: 0.5rem 0rem;
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

*{
	margin:0;
	padding:0;
	border:0
}

.app-content .player-footer {
  position: absolute;
  bottom: 0;
  width: auto;
}
</style>
