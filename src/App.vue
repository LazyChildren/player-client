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
}

function nextTrack() {
  currentTrackIndex.value = (currentTrackIndex.value + 1) % tracks.value.length;
}

function selectTrack(index: number) {
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
        <div></div>
        <WindowControls />
      </div>

      <div class="content-grid">
        <SidebarMenu />
        <main class="main-panel">
          <PlaylistView
            :tracks="tracks"
            :currentTrackIndex="currentTrackIndex"
            @select="selectTrack"
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
  box-shadow: 0 20px 60px rgba(15, 23, 42, 0.12);
}

.app-content {
  position: relative;
  width: 100%;
  height: 100%;
  padding: 1.2rem 1.2rem 8rem;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  gap: 1.2rem;
  overflow: hidden;
}

.titlebar {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  padding: 0.65rem 1rem;
  background: transparent;
  border: none;
  -webkit-app-region: drag;
}

.titlebar > div:first-child {
  flex: 1;
  -webkit-app-region: drag;
}

.content-grid {
  display: grid;
  grid-template-columns: 280px minmax(0, 1fr);
  gap: 1.2rem;
}

.main-panel {
  display: flex;
  flex-direction: column;
  gap: 1.2rem;
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