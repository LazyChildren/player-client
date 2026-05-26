<script setup lang="ts">
import type { Track } from "../types/music";

const props = defineProps<{
  currentTrack: Track | undefined;
  isPlaying: boolean;
  currentTime: number;
  duration: number;
  progress: number;
  volume: number;
}>();

const emit = defineEmits<{
  (event: "prev"): void;
  (event: "playPause"): void;
  (event: "next"): void;
  (event: "seek", value: number): void;
  (event: "set-volume", value: number): void;
  (event: "toggle-mute"): void;
}>();

function onSeek(event: Event) {
  const value = Number((event.target as HTMLInputElement).value);
  emit("seek", value);
}

function onVolumeInput(event: Event) {
  const value = Number((event.target as HTMLInputElement).value);
  emit("set-volume", value);
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
          <h2>{{ props.currentTrack?.title ?? "No track selected" }}</h2>
          <p>{{ props.currentTrack?.artist ?? "Nothing is playing right now" }}</p>
        </div>
      </div>

      <div class="player-controls">
        <button @click="emit('prev')" class="control-button" title="Previous track" type="button">
          <svg class="icon" viewBox="0 0 1024 1024" width="20" height="20" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path d="M803.584 134.592c-18.176-10.048-39.936-10.112-58.112 0L192 443.008l0-282.88c0-17.664-14.336-32-32-32S128 142.464 128 160.128l0 344.896C127.744 507.2 127.104 509.248 127.104 511.488S127.744 515.84 128 518.016l0 346.112c0 17.664 14.336 32 32 32s32-14.336 32-32L192 579.968l553.408 308.352C754.56 893.44 764.48 896 774.528 896s19.968-2.56 29.056-7.616c18.176-10.112 28.992-28.224 28.992-48.576L832.576 183.104C832.576 162.88 821.76 144.768 803.584 134.592zM768.192 829.248 198.016 511.488 771.328 192 768.192 829.248z" />
          </svg>
        </button>

        <button @click="emit('playPause')" class="control-button primary" :title="props.isPlaying ? 'Pause' : 'Play'" type="button">
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

        <button @click="emit('next')" class="control-button" title="Next track" type="button">
          <svg class="icon" viewBox="0 0 1024 1024" width="20" height="20" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
            <path d="M832 160.128c0-17.664-14.336-32-32-32S768 142.464 768 160.128l0 282.88L214.592 134.592c-18.176-10.112-39.936-10.048-58.112 0C138.24 144.768 127.424 162.88 127.424 183.168l0 656.64c0 20.224 10.816 38.464 29.056 48.576C165.568 893.44 175.488 896 185.472 896s19.968-2.56 29.056-7.616L768 579.968l0 284.096c0 17.664 14.336 32 32 32s32-14.336 32-32L832 160.128zM191.808 829.248 188.672 192l573.312 319.488L191.808 829.248z" />
          </svg>
        </button>
      </div>

      <div class="footer-right">
        <div class="volume-controls" aria-label="Volume control">
          <button
            @click="emit('toggle-mute')"
            class="volume-button"
            :title="props.volume === 0 ? 'Unmute' : 'Mute'"
            type="button"
          >
            <svg
              v-if="props.volume === 0"
              class="icon"
              viewBox="0 0 1024 1024"
              width="20"
              height="20"
              fill="currentColor"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path d="M448 938.666667a21.333333 21.333333 0 0 1-15.093333-6.246667L225.833333 725.333333H53.333333a53.393333 53.393333 0 0 1-53.333333-53.333333V352a53.393333 53.393333 0 0 1 53.333333-53.333333h172.5l207.08-207.086667A21.333333 21.333333 0 0 1 469.333333 106.666667v810.666666a21.333333 21.333333 0 0 1-21.333333 21.333334zM53.333333 341.333333a10.666667 10.666667 0 0 0-10.666666 10.666667v320a10.666667 10.666667 0 0 0 10.666666 10.666667h181.333334a21.333333 21.333333 0 0 1 15.086666 6.246666L426.666667 865.833333V158.166667L249.753333 335.086667A21.333333 21.333333 0 0 1 234.666667 341.333333z m964.42 377.753334a21.333333 21.333333 0 0 0 0-30.173334L840.833333 512l176.92-176.913333a21.333333 21.333333 0 1 0-30.173333-30.173334L810.666667 481.833333 633.753333 304.913333a21.333333 21.333333 0 0 0-30.173333 30.173334L780.5 512l-176.92 176.913333a21.333333 21.333333 0 0 0 30.173333 30.173334L810.666667 542.166667l176.913333 176.92a21.333333 21.333333 0 0 0 30.173333 0z" />
            </svg>
            <svg
              v-else
              class="icon"
              viewBox="0 0 1024 1024"
              width="20"
              height="20"
              fill="currentColor"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path d="M509.226667 167.722667l-222.442667 184.341333c-16.704 13.866667-43.178667 23.402667-64.896 23.402667H170.730667A42.666667 42.666667 0 0 0 128 418.090667v187.434666c0 23.466667 19.178667 42.624 42.730667 42.624H221.866667c21.802667 0 48.170667 9.536 64.896 23.402667l222.421333 184.32V167.744zM259.541333 704.426667c-9.045333-7.509333-25.770667-13.589333-37.674666-13.589334H170.730667A85.418667 85.418667 0 0 1 85.333333 605.525333v-187.434666A85.333333 85.333333 0 0 1 170.730667 332.8H221.866667c11.776 0 28.629333-6.08 37.674666-13.589333L519.125333 104.106667c18.090667-14.997333 32.746667-8.170667 32.746667 15.402666v784.64c0 23.488-14.677333 30.378667-32.746667 15.402667L259.562667 704.405333z m436.117334-4.821334a21.333333 21.333333 0 1 1-26.026667-33.834666A196.608 196.608 0 0 0 746.666667 509.504c0-57.408-29.269333-112.32-77.482667-151.637333a21.333333 21.333333 0 1 1 26.965333-33.066667C753.792 371.797333 789.333333 438.485333 789.333333 509.504c0 75.370667-35.050667 144.981333-93.653333 190.08z m64 170.666667a21.333333 21.333333 0 1 1-26.026667-33.834667A414.506667 414.506667 0 0 0 896 507.029333c0-121.642667-61.696-237.354667-162.816-319.829333a21.333333 21.333333 0 1 1 26.965333-33.066667C870.698667 244.309333 938.666667 371.776 938.666667 507.029333a457.173333 457.173333 0 0 1-178.986667 363.221334z" />
            </svg>
          </button>

          <div class="volume-popover">
            <div class="volume-slider-wrap">
              <input
                class="volume-slider"
                type="range"
                min="0"
                max="100"
                step="1"
                :value="props.volume"
                @input="onVolumeInput"
              />
            </div>
            <span class="volume-value">{{ props.volume }}%</span>
          </div>
        </div>

        <div class="time-labels" aria-label="Playback time">
          <span>{{ formatTime(props.currentTime) }}</span>
          <span class="time-separator">/</span>
          <span>{{ props.duration ? formatTime(props.duration) : "0:00" }}</span>
        </div>

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

.footer-right {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 0rem;
  min-width: 0;
  justify-self: end;
}

.time-labels {
  display: flex;
  align-items: center;
  gap: 0.35rem;
  color: #6b7280;
  font-size: 0.85rem;
  min-width: 5rem;
  justify-content: flex-end;

  font-variant-numeric: tabular-nums;
}

.time-separator {
  opacity: 0.7;
}

.volume-controls {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: flex-end;
}

.volume-button {
  all: unset;
  cursor: pointer;
  width: 2rem;
  height: 2rem;
  display: grid;
  place-items: center;
  border-radius: 50%;
  color: #6b7280;
  background: rgba(79, 70, 229, 0.08);
  transition: background 0.2s ease, color 0.2s ease, transform 0.2s ease;
}

.volume-button:hover {
  background: rgba(79, 70, 229, 0.14);
  color: #4338ca;
  transform: translateY(-1px);
}

.volume-button .icon {
  width: 18px;
  height: 18px;
  display: block;
}

.volume-popover {
  position: absolute;
  left: 50%;
  bottom: calc(100% + 0.7rem);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0rem;
  width: 1.5rem;
  padding: 0.75rem 0.45rem 0.6rem;
  border: 1px solid rgba(79, 70, 229, 0.14);
  border-radius: 1rem;
  background: rgba(255, 255, 255, 0.98);
  box-shadow: 0 12px 30px rgba(15, 23, 42, 0.14);
  opacity: 0;
  visibility: hidden;
  transform: translateX(-50%) translateY(6px);
  pointer-events: none;
  transition:
    opacity 0.2s ease 0.18s,
    transform 0.2s ease 0.18s,
    visibility 0s linear 0.18s;
}

.volume-popover::after {
  content: "";
  position: absolute;
  left: 50%;
  top: 100%;
  width: 0.7rem;
  height: 0.7rem;
  background: rgba(255, 255, 255, 0.98);
  border-right: 1px solid rgba(79, 70, 229, 0.14);
  border-bottom: 1px solid rgba(79, 70, 229, 0.14);
  transform: translate(-50%, -50%) rotate(45deg);
}

.volume-popover::before {
  content: "";
  position: absolute;
  left: 0;
  right: 0;
  top: 100%;
  height: 1rem;
}

.volume-controls:hover .volume-popover,
.volume-controls:focus-within .volume-popover {
  opacity: 1;
  visibility: visible;
  transform: translateX(-50%) translateY(0);
  pointer-events: auto;
  transition-delay: 0s;
}

.volume-value {
  min-width: 0;
  color: #4b5563;
  font-size: 0.78rem;
  font-variant-numeric: tabular-nums;
  line-height: 1;
  text-align: center;
}

.volume-slider-wrap {
  width: 0.5rem;
  height: 7.5rem;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: visible;
}

.volume-slider {
  width: 7.25rem;
  height: 0.3rem;
  accent-color: #787878;
  transform: rotate(-90deg);
  transform-origin: center;
  margin: 0;
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
  .footer-right {
    justify-self: stretch;
  }

  .player-controls {
    justify-self: center;
  }

  .footer-right {
    flex-wrap: wrap;
    justify-content: flex-end;
  }

  .volume-popover {
    left: 50%;
  }
}
</style>
