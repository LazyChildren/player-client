<script setup lang="ts">

const props = defineProps<{
  tracks: Array<{
    id: number;
    title: string;
    artist: string;
    album: string;
    duration: string;
    color: string;
  }>;
  currentTrackIndex: number;
}>();

const { tracks, currentTrackIndex } = props;

const emit = defineEmits<{
  (event: "select", index: number): void;
}>();

function selectTrack(index: number) {
  emit("select", index);
}
</script>

<template>
  <section class="playlist-panel">
    <div class="playlist-header">
      <div>
        <h2>全部歌曲</h2>
      </div>
      <span class="playlist-count">{{ tracks.length }} 首</span>
    </div>

    <ul class="track-list">
      <li
        v-for="(track, index) in tracks"
        :key="track.id"
        :class="{ active: index === currentTrackIndex }"
        @click="selectTrack(index)"
      >
        <div class="track-item">
          <div class="cover-preview" :style="{ background: track.color }"></div>
          <div class="track-meta">
            <p class="track-title">{{ track.title }}</p>
            <p class="track-subtitle">{{ track.artist }} · {{ track.album }}</p>
          </div>
        </div>
        <span class="duration">{{ track.duration }}</span>
      </li>
    </ul>
  </section>
</template>

<style scoped>
.playlist-panel {
  background: rgba(255, 255, 255, 0.88);
  border-radius: 0;
  padding: 1.6rem;
  display: flex;
  flex-direction: column;
  gap: 1.4rem;
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
}

/* 自定义滚动条 - Webkit浏览器 */
.playlist-panel::-webkit-scrollbar {
  width: 10px;
}

.playlist-panel::-webkit-scrollbar-track {
  background: rgba(245, 246, 251, 0.5);
  border-radius: 0;
}

.playlist-panel::-webkit-scrollbar-thumb {
  background: rgba(99, 102, 241, 0.4);
  border-radius: 0;
  transition: background 0.2s ease;
}

.playlist-panel::-webkit-scrollbar-thumb:hover {
  background: rgba(99, 102, 241, 0.6);
}

.playlist-panel::-webkit-scrollbar-button {
  display: none;
}

/* Firefox滚动条样式 */
.playlist-panel {
  scrollbar-color: rgba(99, 102, 241, 0.4) rgba(245, 246, 251, 0.5);
  scrollbar-width: thin;
}
.playlist-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
}
.playlist-header h2 {
  margin: 0;
  font-size: 1.55rem;
}
.playlist-count {
  color: #6b7280;
  font-weight: 700;
}
.track-list {
  list-style: none;
  margin: 0;
  padding: 0;
  display: grid;
  gap: 0.85rem;
}
.track-list li {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.95rem 1rem;
  border-radius: 0;
  cursor: pointer;
  transition: background 0.2s ease, transform 0.15s ease;
}
.track-list li:hover {
  background: rgba(99, 102, 241, 0.08);
  transform: translateY(-1px);
}
.track-list li.active {
  background: rgba(79, 70, 229, 0.12);
}
.track-item {
  display: flex;
  align-items: center;
  gap: 0.95rem;
}
.cover-preview {
  width: 3rem;
  height: 3rem;
  border-radius: 0;
}
.track-meta {
  display: flex;
  flex-direction: column;
}
.track-title {
  margin: 0;
  font-weight: 700;
}
.track-subtitle {
  margin: 0.25rem 0 0;
  color: #6b7280;
  font-size: 0.95rem;
}
.duration {
  color: #4b5563;
  font-weight: 700;
}
</style>
