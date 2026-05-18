<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from "vue";

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

function seek(event: Event) {
  const target = event.target as HTMLInputElement;
  const value = Number(target.value);
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
  <div class="player-shell">
    <header class="player-header">
      <div>
        <p class="eyebrow">Tauri Music Player</p>
        <h1>Listen to your favorite tracks</h1>
      </div>
    </header>

    <section class="player-grid">
      <article class="now-playing-card">
        <div class="cover" :style="{ background: `linear-gradient(135deg, ${currentTrack.color} 0%, rgba(255,255,255,0.15) 100%)` }">
          <span class="cover-badge">Now playing</span>
        </div>

        <div class="track-info">
          <div>
            <p class="track-title">{{ currentTrack.title }}</p>
            <p class="track-artist">{{ currentTrack.artist }}</p>
            <p class="track-album">{{ currentTrack.album }}</p>
          </div>
          <div class="track-length">{{ currentTrack.duration }}</div>
        </div>

        <div class="controls">
          <button class="control-button" @click="prevTrack" aria-label="Previous track">⏮</button>
          <button class="control-button primary" @click="playPause" aria-label="Play or pause">
            {{ isPlaying ? "⏸" : "▶" }}
          </button>
          <button class="control-button" @click="nextTrack" aria-label="Next track">⏭</button>
        </div>

        <div class="progress-row">
          <span>{{ formatTime(currentTime) }}</span>
          <input type="range" min="0" max="100" step="0.1" :value="progress" @input="seek" />
          <span>{{ duration ? formatTime(duration) : "0:00" }}</span>
        </div>
      </article>

      <aside class="playlist-card">
        <div class="playlist-header">
          <div>
            <p class="eyebrow">Playlist</p>
            <h2>Top tracks</h2>
          </div>
          <span class="playlist-count">{{ tracks.length }} songs</span>
        </div>

        <ul class="track-list">
          <li
            v-for="(track, index) in tracks"
            :key="track.id"
            :class="{ active: index === currentTrackIndex }"
            @click="selectTrack(index)"
          >
            <div class="track-row">
              <div class="track-marker" :style="{ backgroundColor: track.color }"></div>
              <div>
                <p class="track-name">{{ track.title }}</p>
                <p class="track-meta">{{ track.artist }} · {{ track.album }}</p>
              </div>
            </div>
            <span class="duration-label">{{ track.duration }}</span>
          </li>
        </ul>
      </aside>
    </section>

    <audio ref="audioRef"></audio>
  </div>
</template>

<style scoped>
.player-shell {
  min-height: 100vh;
  padding: 2rem;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  background: radial-gradient(circle at top, rgba(99, 102, 241, 0.08), transparent 38%),
    radial-gradient(circle at bottom right, rgba(16, 185, 129, 0.08), transparent 28%),
    #f4f5fb;
}

.player-header {
  display: flex;
  justify-content: center;
  text-align: center;
}

.eyebrow {
  text-transform: uppercase;
  letter-spacing: 0.2em;
  font-size: 0.75rem;
  color: #6366f1;
  margin-bottom: 0.75rem;
}

.player-header h1 {
  margin: 0;
  font-size: clamp(2rem, 2.5vw, 2.8rem);
  line-height: 1.05;
}

.player-grid {
  display: grid;
  grid-template-columns: minmax(0, 1.6fr) minmax(280px, 1fr);
  gap: 1.5rem;
}

.now-playing-card,
.playlist-card {
  background: rgba(255, 255, 255, 0.86);
  border: 1px solid rgba(15, 23, 42, 0.08);
  border-radius: 28px;
  box-shadow: 0 18px 50px rgba(15, 23, 42, 0.08);
  padding: 1.7rem;
}

.cover {
  height: 260px;
  border-radius: 24px;
  display: grid;
  place-items: start;
  padding: 1rem;
  position: relative;
  overflow: hidden;
  color: white;
}

.cover-badge {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  background: rgba(255, 255, 255, 0.16);
  padding: 0.55rem 0.8rem;
  border-radius: 999px;
  font-size: 0.8rem;
  backdrop-filter: blur(8px);
}

.track-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1rem;
  margin-top: 1.4rem;
}

.track-title {
  font-size: 1.7rem;
  font-weight: 700;
  margin: 0;
}

.track-artist,
.track-album {
  margin: 0.35rem 0 0;
  color: #6b7280;
}

.track-length {
  font-weight: 700;
  color: #111827;
}

.controls {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 1rem;
  margin-top: 1.7rem;
}

.control-button {
  border: none;
  background: #eef2ff;
  color: #4338ca;
  width: 3.3rem;
  height: 3.3rem;
  border-radius: 16px;
  cursor: pointer;
  font-size: 1.1rem;
  transition: transform 0.2s ease, background 0.2s ease;
}

.control-button:hover {
  transform: translateY(-2px);
  background: #e0e7ff;
}

.control-button.primary {
  width: 4.4rem;
  height: 4.4rem;
  background: #4f46e5;
  color: white;
}

.progress-row {
  display: grid;
  grid-template-columns: auto 1fr auto;
  align-items: center;
  gap: 1rem;
  margin-top: 1.8rem;
}

.progress-row input[type="range"] {
  width: 100%;
  accent-color: #4f46e5;
}

.playlist-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
  gap: 1rem;
}

.playlist-header h2 {
  margin: 0;
  font-size: 1.45rem;
}

.playlist-count {
  color: #6b7280;
  font-weight: 700;
}

.track-list {
  list-style: none;
  margin: 1.4rem 0 0;
  padding: 0;
  display: grid;
  gap: 0.85rem;
}

.track-list li {
  border-radius: 18px;
  padding: 1rem 1rem;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  cursor: pointer;
  transition: background 0.2s ease, transform 0.2s ease;
}

.track-list li:hover {
  background: rgba(99, 102, 241, 0.08);
}

.track-list li.active {
  background: rgba(79, 70, 229, 0.12);
}

.track-row {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.track-marker {
  width: 1.2rem;
  height: 1.2rem;
  border-radius: 50%;
  flex-shrink: 0;
}

.track-name {
  margin: 0;
  font-weight: 600;
}

.track-meta {
  margin: 0.25rem 0 0;
  color: #6b7280;
  font-size: 0.95rem;
}

.duration-label {
  color: #4b5563;
  font-weight: 700;
}

@media (max-width: 960px) {
  .player-grid {
    grid-template-columns: 1fr;
  }
}
</style>