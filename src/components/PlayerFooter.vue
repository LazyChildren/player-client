<script setup lang="ts">
import type { Track } from "../types/music";

const props = defineProps<{
  currentTrack: Track | undefined;
  isPlaying: boolean;
  currentTime: number;
  duration: number;
  progress: number;
}>();

const emit = defineEmits<{
  (event: "prev"): void;
  (event: "playPause"): void;
  (event: "next"): void;
  (event: "seek", value: number): void;
}>();

function onSeek(event: Event) {
  const value = Number((event.target as HTMLInputElement).value);
  emit("seek", value);
}

function formatTime(seconds: number) {
  const minutes = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${minutes}:${secs.toString().padStart(2, "0")}`;
}
</script>

<template>
  <section class="player-footer">
    <div class="player-footer-inner">
      <div class="player-info-card">
        <img v-if="props.currentTrack" class="player-cover" :src="props.currentTrack.coverUrl" :alt="props.currentTrack.title" />
        <div v-else class="player-cover placeholder"></div>
        <div class="player-meta">
          <h2>{{ props.currentTrack?.title ?? "未选择歌曲" }}</h2>
          <p>{{ props.currentTrack?.artist ?? "暂无播放信息" }}</p>
        </div>
      </div>

      <div class="player-controls">
        <button @click="emit('prev')" class="control-button" title="上一首">
          <svg class="icon" viewBox="0 0 1024 1024" width="20" height="20" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path d="M803.584 134.592c-18.176-10.048-39.936-10.112-58.112 0L192 443.008l0-282.88c0-17.664-14.336-32-32-32S128 142.464 128 160.128l0 344.896C127.744 507.2 127.104 509.248 127.104 511.488S127.744 515.84 128 518.016l0 346.112c0 17.664 14.336 32 32 32s32-14.336 32-32L192 579.968l553.408 308.352C754.56 893.44 764.48 896 774.528 896s19.968-2.56 29.056-7.616c18.176-10.112 28.992-28.224 28.992-48.576L832.576 183.104C832.576 162.88 821.76 144.768 803.584 134.592zM768.192 829.248 198.016 511.488 771.328 192 768.192 829.248z" />
          </svg>
        </button>

        <button @click="emit('playPause')" class="control-button primary" :title="props.isPlaying ? '暂停' : '播放'">
          <template v-if="props.isPlaying">
            <svg class="icon" viewBox="0 0 1024 1024" width="20" height="20" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
              <path d="M288 128C270.336 128 256 142.336 256 160l0 768C256 945.664 270.336 960 288 960S320 945.664 320 928l0-768C320 142.336 305.664 128 288 128zM672 128C654.336 128 640 142.336 640 160l0 768c0 17.664 14.336 32 32 32s32-14.336 32-32l0-768C704 142.336 689.664 128 672 128z" />
            </svg>
          </template>
          <template v-else>
            <svg class="icon play-icon" viewBox="0 0 1024 1024" width="20" height="20" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
              <path d="M803.84 492.032 213.056 135.04C195.392 124.288 174.208 124.288 156.352 134.976 138.624 145.728 128 164.928 128 186.368l0 714.176c0 21.44 10.624 40.704 28.352 51.392C165.248 957.312 174.976 960 184.704 960c9.728 0 19.52-2.688 28.352-8.064l590.72-356.992c17.728-10.752 28.288-30.016 28.288-51.456C832.128 521.984 821.568 502.72 803.84 492.032zM189.568 891.712 187.136 193.792l578.624 349.696L189.568 891.712z" />
            </svg>
          </template>
        </button>

        <button @click="emit('next')" class="control-button" title="下一首">
          <svg class="icon" viewBox="0 0 1024 1024" width="20" height="20" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path d="M832 160.128c0-17.664-14.336-32-32-32S768 142.464 768 160.128l0 282.88L214.592 134.592c-18.176-10.112-39.936-10.048-58.112 0C138.24 144.768 127.424 162.88 127.424 183.168l0 656.64c0 20.224 10.816 38.464 29.056 48.576C165.568 893.44 175.488 896 185.472 896s19.968-2.56 29.056-7.616L768 579.968l0 284.096c0 17.664 14.336 32 32 32s32-14.336 32-32L832 160.128zM191.808 829.248 188.672 192l573.312 319.488L191.808 829.248z" />
          </svg>
        </button>

      </div>

      <div class="time-labels" aria-label="播放时间">
        <span>{{ formatTime(props.currentTime) }}</span>
        <span class="time-separator">/</span>
        <span>{{ props.duration ? formatTime(props.duration) : "0:00" }}</span>
      </div>
    </div>

    <div class="progress-block">
      <input
        class="progress-slider"
        type="range"
        min="0"
        max="100"
        step="0.1"
        :value="props.progress"
        @input="onSeek"
      />
    </div>
  </section>
</template>

<style scoped>
.player-footer {
  position: absolute;
  left: 0;
  right: 0;
  bottom: 0;
  width: 100%;
  box-sizing: border-box;
  border-radius: 0;
  background: rgba(255, 255, 255, 0.95);
  border: 1px solid rgba(128, 128, 128, 0.08);
  padding: 0.5rem 1rem 0.75rem;
  display: grid;
  gap: 0.5rem;
  z-index: 50;
}

.player-footer-inner {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto minmax(0, 1fr);
  align-items: center;
  column-gap: 1rem;
  row-gap: 0.5rem;
}

.player-info-card {
  display: flex;
  align-items: center;
  gap: 0.7rem;
  padding: 0.65rem 0.75rem;
  border-radius: 0;
  color: #111827;
  min-width: 0;
  width: 100%;
  justify-self: start;
}

.player-cover {
  width: 2.5rem;
  height: 2.5rem;
  border-radius: 0;
  object-fit: cover;
  background: rgba(255, 255, 255, 0.45);
}

.player-cover.placeholder {
  background: rgba(226, 232, 240, 0.85);
}

.player-meta h2 {
  margin: 0.2rem 0 0.1rem;
  font-size: 0.85rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.player-meta p {
  margin: 0;
  color: #4b5563;
  font-size: 0.75rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.player-controls {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  justify-self: center;
  min-width: max-content;
}

.control-button {
  all: unset;
  cursor: pointer;
  width: 2.4rem;
  height: 2.4rem;
  display: grid;
  place-items: center;
  border-radius: 50%;
  overflow: hidden;
  background: #eef2ff;
  color: #4338ca;
  line-height: 0;
  transition: transform 0.2s ease, background 0.2s ease;
}

.control-button:hover {
  transform: translateY(-2px);
  background: #e0e7ff;
}

.control-button.primary {
  width: 2.4rem;
  height: 2.4rem;
  background: #eef2ff;
  color: #4338ca;
}

.control-button .icon {
  width: 18px;
  height: 18px;
  display: block;
}

.control-button .play-icon {
  transform: translateX(1px);
}

.time-labels {
  display: flex;
  align-items: center;
  gap: 0.35rem;
  color: #6b7280;
  font-size: 0.85rem;
  min-width: 9rem;
  justify-content: flex-end;
  justify-self: end;
  font-variant-numeric: tabular-nums;
  margin: 0 0.5rem 0;
}

.time-separator {
  opacity: 0.7;
}

.progress-block {
  width: 100%;
}

.progress-slider {
  width: 100%;
  accent-color: #4f46e5;
}

@media (max-width: 960px) {
  .player-footer-inner {
    grid-template-columns: minmax(0, 1fr);
    justify-items: stretch;
  }

  .player-info-card,
  .player-controls,
  .time-labels {
    justify-self: stretch;
  }

  .player-controls {
    justify-self: center;
  }

  .time-labels {
    justify-content: flex-end;
  }
}
</style>
