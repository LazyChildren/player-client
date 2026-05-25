<script setup lang="ts">
import PlayingIndicator from "./PlayingIndicator.vue";
import type { Track } from "../types/music";

const props = defineProps<{
  tracks: Track[];
  activeTrackIndex: number;
  currentTrackIndex: number;
  isPlaying: boolean;
  playlistName: string;
  loading: boolean;
  errorMessage: string;
}>();

const emit = defineEmits<{
  (event: "select", index: number): void;
  (event: "play", index: number): void;
}>();

function selectTrack(index: number) {
  emit("select", index);
}

function playTrack(index: number) {
  emit("play", index);
}
</script>

<template>
  <section class="playlist-panel">
    <div class="playlist-header">
      <div>
        <p class="playlist-title">{{ props.playlistName }}</p>
      </div>
      <span class="playlist-count">{{ props.tracks.length }} 首</span>
    </div>

    <div class="track-table-header">
      <span class="col-index">序号</span>
      <span class="col-title">标题</span>
      <span class="col-artist">艺术家</span>
      <span class="col-duration">时长</span>
    </div>

    <div v-if="props.loading" class="playlist-state">正在加载音乐列表...</div>
    <div v-else-if="props.errorMessage" class="playlist-state error">{{ props.errorMessage }}</div>
    <div v-else-if="!props.tracks.length" class="playlist-state">暂无可播放的音乐数据</div>

    <ul v-else class="track-list">
      <li
        v-for="(track, index) in props.tracks"
        :key="track.id"
        :class="{ active: index === props.activeTrackIndex }"
        @click="selectTrack(index)"
        @dblclick="playTrack(index)"
      >
        <span class="row-index">
          <PlayingIndicator v-if="props.isPlaying && index === props.currentTrackIndex" :autoplay="true" />
          <template v-else>{{ index + 1 }}</template>
        </span>
        <div class="track-item">
          <img class="cover-preview" :src="track.coverUrl" :alt="track.title" />
          <div class="track-meta">
            <p class="track-title">{{ track.title }}</p>
          </div>
        </div>
        <span class="row-artist">{{ track.artist }}</span>
        <span class="duration">{{ track.duration ?? "--:--" }}</span>
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

.playlist-title {
  margin: 0;
  font-size: 1.55rem;
  font-weight: 700;
  color: #374151;
}

.playlist-count {
  color: #6b7280;
  font-weight: 700;
}

.track-table-header {
  display: grid;
  grid-template-columns: 64px minmax(0, 1.5fr) minmax(0, 1fr) 90px;
  align-items: center;
  gap: 0.75rem;
  padding: 0 0.9rem 0.35rem;
  border-bottom: 1px solid rgba(15, 23, 42, 0.08);
  color: #8e95a1;
  font-size: 0.82rem;
  font-weight: 400;
}

.col-index {
  text-align: center;
}

.col-title {
  padding-left: 4.85rem;
}

.playlist-state {
  margin: 0 0.9rem;
  padding: 1rem 1.1rem;
  border-radius: 0.9rem;
  background: rgba(99, 102, 241, 0.08);
  color: #4b5563;
  font-size: 0.92rem;
}

.playlist-state.error {
  background: rgba(239, 68, 68, 0.1);
  color: #b91c1c;
}

.track-list {
  list-style: none;
  margin: 0;
  padding: 0 0.9rem;
  display: grid;
  gap: 0.2rem;
}

.track-list li {
  display: grid;
  grid-template-columns: 64px minmax(0, 1.5fr) minmax(0, 1fr) 90px;
  align-items: center;
  gap: 0.75rem;
  padding: 0.3rem 0;
  border-radius: 0.9rem;
  cursor: default;
  transition: background 0.2s ease, transform 0.15s ease;
}

.track-list li:hover {
  background: rgba(99, 102, 241, 0.08);
  transform: translateY(-1px);
}

.track-list li.active {
  background: rgba(79, 70, 229, 0.12);
}

.row-index {
  color: #6b7280;
  font-weight: 300;
  text-align: center;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-height: 16px;
}

.track-item {
  display: flex;
  align-items: center;
  gap: 0.95rem;
  border-radius: 0.9rem;
  padding: 0.65rem 0.9rem;
  min-width: 0;
  font-size: 0.92rem;
}

.cover-preview {
  width: 3rem;
  height: 3rem;
  border-radius: 0;
  object-fit: cover;
  background: rgba(226, 232, 240, 0.9);
}

.track-meta {
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.track-title {
  margin: 0;
  font-weight: 350;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.row-artist {
  color: #7b8391;
  font-size: 0.86rem;
  font-weight: 350;
  min-width: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.duration {
  color: #8a8a8a;
  font-size: 0.86rem;
  font-weight: 400;
  text-align: left;
}
</style>
