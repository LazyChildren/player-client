<script setup lang="ts">
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
  (event: "refresh"): void;
  (event: "upload"): void;
}>();

function selectTrack(index: number) {
  emit("select", index);
}

function playTrack(index: number) {
  emit("play", index);
}

function refreshPlaylist() {
  if (props.loading) return;
  emit("refresh");
}

function openUploadDialog() {
  emit("upload");
}
</script>

<template>
  <section class="playlist-panel">
    <div class="playlist-header">
      <div class="playlist-header-main">
        <p class="playlist-title">{{ props.playlistName }}</p>
        <button
          type="button"
          class="playlist-refresh-button"
          :disabled="props.loading"
          :aria-busy="props.loading"
          aria-label="刷新音乐列表"
          title="刷新音乐列表"
          @click="refreshPlaylist"
        >
          <svg
            class="playlist-refresh-icon"
            viewBox="0 0 1024 1024"
            version="1.1"
            xmlns="http://www.w3.org/2000/svg"
            aria-hidden="true"
          >
            <path
              d="M989.311588 512.085547a36.053318 36.053318 0 0 0-38.613317 33.194652 438.570484 438.570484 0 0 1-138.794609 288.63988A438.186484 438.186484 0 0 1 511.999787 951.978697c-87.039964 0-171.093262-25.258656-243.199899-73.258636a439.63715 439.63715 0 0 1-148.778605-166.698598h99.967959a35.967985 35.967985 0 1 0 0-72.021303H36.010652a35.967985 35.967985 0 0 0-36.010652 36.010652v183.97859a35.967985 35.967985 0 1 0 72.021303 0v-85.973298A513.066453 513.066453 0 0 0 228.863905 938.666702C312.917203 994.517346 410.666496 1024 511.999787 1024c130.005279 0 253.994561-48.810646 349.013188-137.386609a509.653121 509.653121 0 0 0 161.493266-335.914527 35.967985 35.967985 0 0 0-33.194653-38.613317zM988.031588 128.000373a35.967985 35.967985 0 0 0-36.053318 36.010652v85.973298A512.298453 512.298453 0 0 0 795.007669 85.333724 509.439788 509.439788 0 0 0 511.999787 0.000427a510.122454 510.122454 0 0 0-349.013188 137.386609 510.207787 510.207787 0 0 0-161.5786 335.914527 36.053318 36.053318 0 0 0 33.194653 38.613317 36.053318 36.053318 0 0 0 38.613317-33.194653 438.570484 438.570484 0 0 1 138.794609-288.63988A438.613151 438.613151 0 0 1 511.999787 71.979063c87.039964 0 171.093262 25.301323 243.199898 73.301303a439.63715 439.63715 0 0 1 148.821272 166.741264h-100.010625a35.967985 35.967985 0 1 0 0 71.978637h183.97859A35.967985 35.967985 0 0 0 1023.999573 347.989615V164.011025A35.967985 35.967985 0 0 0 987.988922 128.000373z"
              fill="currentColor"
              opacity="0.72"
            />
          </svg>
        </button>
      </div>

      <div class="playlist-header-actions">
        <span class="playlist-count">{{ props.tracks.length }} 首</span>
        <button type="button" class="playlist-upload-button" aria-label="上传歌曲" title="上传歌曲" @click="openUploadDialog">
          <svg
            class="playlist-upload-icon"
            viewBox="0 0 1024 1024"
            version="1.1"
            xmlns="http://www.w3.org/2000/svg"
            aria-hidden="true"
          >
            <path
              d="M341.333333 337.066667l115.2-115.2c8.533333-8.533333 17.066667-12.8 29.866667-17.066667v473.6c0 17.066667 12.8 29.866667 25.6 29.866667s25.6-12.8 25.6-29.866667V204.8c8.533333 4.266667 21.333333 8.533333 29.866667 17.066667L682.666667 337.066667c4.266667 4.266667 12.8 8.533333 21.333333 8.533333s12.8-4.266667 21.333333-8.533333c12.8-12.8 12.8-29.866667 0-38.4l-115.2-115.2C554.666667 132.266667 469.333333 132.266667 418.133333 183.466667L302.933333 298.666667c-12.8 12.8-12.8 29.866667 0 38.4s25.6 12.8 38.4 0z"
              fill="currentColor"
            />
            <path
              d="M844.8 546.133333c-17.066667 0-29.866667 12.8-29.866667 25.6v170.666667c0 42.666667-34.133333 76.8-76.8 76.8H285.866667c-42.666667 0-76.8-34.133333-76.8-76.8v-170.666667c0-17.066667-12.8-25.6-25.6-25.6s-25.6 12.8-25.6 25.6v170.666667c0 72.533333 59.733333 132.266667 132.266666 132.266667h456.533334c72.533333 0 132.266667-59.733333 132.266666-132.266667v-170.666667c-4.266667-12.8-17.066667-25.6-34.133333-25.6z"
              fill="currentColor"
            />
          </svg>
          <span>上传歌曲</span>
        </button>
      </div>
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
          <span v-if="props.isPlaying && index === props.currentTrackIndex" class="playing-bars" aria-hidden="true">
            <svg
              t="1779702244479"
              class="icon playing-bars-icon"
              viewBox="0 0 1024 1024"
              version="1.1"
              xmlns="http://www.w3.org/2000/svg"
              p-id="21004"
            >
              <rect class="playing-bar bar-1" x="682.666667" y="298.666667" width="42.666666" height="469.333333" />
              <rect class="playing-bar bar-2" x="341.333333" y="298.666667" width="42.666667" height="469.333333" />
              <rect class="playing-bar bar-3" x="170.666667" y="426.666667" width="42.666666" height="213.333333" />
              <rect class="playing-bar bar-4" x="853.333333" y="426.666667" width="42.666667" height="213.333333" />
              <rect class="playing-bar bar-5" x="512" y="170.666667" width="42.666667" height="725.333333" />
            </svg>
          </span>
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

.playlist-header-main {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  min-width: 0;
}

.playlist-header-actions {
  display: inline-flex;
  align-items: center;
  justify-content: flex-end;
  gap: 0.75rem;
  min-width: 0;
}

.playlist-title {
  margin: 0;
  font-size: 1.55rem;
  font-weight: 700;
  color: #374151;
  line-height: 1.2;
}

.playlist-refresh-button {
  width: 2rem;
  height: 2rem;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 9999px;
  background: transparent;
  color: #9ca3af;
  cursor: pointer;
  transition: background 0.2s ease, color 0.2s ease, transform 0.2s ease;
  flex: 0 0 auto;
}

.playlist-refresh-button:hover:not(:disabled) {
  background: rgba(99, 102, 241, 0.1);
  color: #4f46e5;
  transform: rotate(20deg);
}

.playlist-refresh-button:focus-visible {
  outline: none;
  box-shadow: 0 0 0 4px rgba(99, 102, 241, 0.15);
}

.playlist-refresh-button:disabled {
  cursor: progress;
  opacity: 0.55;
}

.playlist-refresh-button:disabled .playlist-refresh-icon {
  animation: playlist-refresh-spin 0.9s linear infinite;
}

.playlist-refresh-icon {
  width: 1rem;
  height: 1rem;
  display: block;
}

.playlist-upload-button {
  display: inline-flex;
  align-items: center;
  gap: 0.45rem;
  height: 2.2rem;
  padding: 0 0.85rem;
  border-radius: 9999px;
  background: rgba(79, 70, 229, 0.1);
  color: #4338ca;
  cursor: pointer;
  font-size: 0.88rem;
  font-weight: 600;
  transition: background 0.2s ease, color 0.2s ease, transform 0.2s ease;
  flex: 0 0 auto;
}

.playlist-upload-button:hover {
  background: rgba(79, 70, 229, 0.16);
  color: #3730a3;
  transform: translateY(-1px);
}

.playlist-upload-button:focus-visible {
  outline: none;
  box-shadow: 0 0 0 4px rgba(99, 102, 241, 0.15);
}

.playlist-upload-icon {
  width: 0.95rem;
  height: 0.95rem;
  display: block;
  flex: 0 0 auto;
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

.playing-bars {
  width: 16px;
  height: 16px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  color: #6366f1;
}

.playing-bars-icon {
  width: 100%;
  height: 100%;
  display: block;
}

.playing-bar {
  fill: currentColor;
  transform-box: fill-box;
  transform-origin: center bottom;
  animation: playing-bars-wave 1s ease-in-out infinite;
}

.bar-1 {
  animation-delay: -0.1s;
}

.bar-2 {
  animation-delay: -0.35s;
}

.bar-3 {
  animation-delay: -0.6s;
}

.bar-4 {
  animation-delay: -0.2s;
}

.bar-5 {
  animation-delay: -0.45s;
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
  object-fit: cover;
  background: rgba(226, 232, 240, 0.9);
  border-radius: 0.2rem;
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

@keyframes playlist-refresh-spin {
  to {
    transform: rotate(360deg);
  }
}

@keyframes playing-bars-wave {
  0%,
  100% {
    opacity: 0.45;
    transform: scaleY(0.45);
  }

  35% {
    opacity: 1;
    transform: scaleY(1);
  }

  70% {
    opacity: 0.72;
    transform: scaleY(0.62);
  }
}
</style>
