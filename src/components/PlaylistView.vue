<script setup lang="ts">
import { computed, ref, watch } from "vue";
import type { Track } from "../types/music";

type TrackRow = {
  track: Track;
  originalIndex: number;
};

const props = defineProps<{
  tracks: TrackRow[];
  activeTrackIndex: number;
  currentTrackIndex: number;
  isPlaying: boolean;
  playlistName: string;
  loading: boolean;
  errorMessage: string;
  deletingTrackIds: number[];
  downloadingTrackIds: number[];
  batchUploading: boolean;
  hasUploadProgress: boolean;
  uploadProgressOpen: boolean;
  deleting: boolean;
  batchDownloading: boolean;
  downloadProgress: string;
  collectionId: number | null;
}>();

const emit = defineEmits<{
  (event: "select", index: number): void;
  (event: "play", index: number): void;
  (event: "delete", index: number): void;
  (event: "download", index: number): void;
  (event: "refresh"): void;
  (event: "upload"): void;
  (event: "toggle-upload-progress"): void;
  (event: "batch-delete", ids: number[]): void;
  (event: "batch-download", ids: number[]): void;
}>();

const batchMode = ref(false);
const selectedIds = ref<number[]>([]);
const selectableIds = computed(() => props.tracks.filter(({ track }) =>
  !props.downloadingTrackIds.includes(track.id) && !props.deletingTrackIds.includes(track.id)
).map(({ track }) => track.id));
const allSelected = computed(() => selectableIds.value.length > 0 && selectableIds.value.every(id => selectedIds.value.includes(id)));
watch(() => props.collectionId, () => { batchMode.value = false; selectedIds.value = []; });
watch(selectableIds, ids => { selectedIds.value = selectedIds.value.filter(id => ids.includes(id)); });
function toggleBatchMode() {
  batchMode.value = !batchMode.value;
  selectedIds.value = [];
}
function toggleAll() {
  selectedIds.value = allSelected.value ? [] : [...selectableIds.value];
}

function selectTrack(index: number) {
  emit("select", index);
}

function playTrack(index: number) {
  emit("play", index);
}

function deleteTrack(index: number) {
  emit("delete", index);
}

function refreshPlaylist() {
  if (props.loading) return;
  emit("refresh");
}

function openUploadDialog() {
  if (props.batchUploading) return;
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
        <span class="playlist-count">{{ props.tracks.length }} 首</span>
      </div>

      <div class="playlist-header-actions">
        <button type="button" class="batch-delete-link" :disabled="props.deleting || props.batchDownloading || props.loading" @click="toggleBatchMode">{{ batchMode ? '取消多选' : '多选' }}</button>
        <template v-if="batchMode">
          <button type="button" class="batch-delete-link danger" :disabled="props.deleting || props.batchDownloading || !selectedIds.length" @click="emit('batch-delete', [...selectedIds])">{{ props.deleting ? '删除中…' : '删除' }}</button>
          <button type="button" class="batch-delete-link" :disabled="props.deleting || props.batchDownloading || !selectedIds.length" @click="emit('batch-download', [...selectedIds])">{{ props.batchDownloading ? `下载中 ${props.downloadProgress}` : '下载' }}</button>
        </template>
        <button type="button" class="playlist-upload-button" :disabled="props.batchUploading" aria-label="上传歌曲" title="上传歌曲" @click="openUploadDialog">
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
        <el-popover v-if="props.hasUploadProgress" :visible="props.uploadProgressOpen"
          placement="bottom-end" :width="320" :show-arrow="true" :teleported="true">
          <template #reference>
        <button type="button" class="upload-progress-button"
          :aria-expanded="props.uploadProgressOpen" aria-controls="batch-upload-progress"
          aria-label="查看上传进度" title="查看上传进度" @click="emit('toggle-upload-progress')">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <circle v-if="props.batchUploading" class="upload-progress-ring" cx="12" cy="12" r="10" stroke-dasharray="45 18" />
            <circle v-else cx="12" cy="12" r="10" opacity="0.4" />
            <path d="M12 17V7m-4 4 4-4 4 4" />
          </svg>
        </button>
          </template>
          <slot name="upload-progress" />
        </el-popover>
      </div>
    </div>

    <div class="playlist-body">
      <div class="track-table-header">
        <span class="col-index">
          <input v-if="batchMode" type="checkbox" aria-label="全选当前列表可操作的歌曲" :checked="allSelected" :indeterminate="selectedIds.length > 0 && !allSelected" :disabled="props.deleting || props.batchDownloading || !selectableIds.length" @change="toggleAll" />
          <template v-else>序号</template>
        </span>
        <span class="col-title">标题</span>
        <span class="col-artist">艺术家</span>
        <span class="col-actions">操作</span>
      </div>
      <div v-if="props.loading" class="playlist-state">正在加载音乐列表...</div>
      <div v-else-if="props.errorMessage" class="playlist-state error">{{ props.errorMessage }}</div>
      <div v-else-if="!props.tracks.length" class="playlist-state">暂无可播放的音乐数据</div>

      <ul v-else class="track-list">
        <li
          v-for="(item, index) in props.tracks"
          :key="item.track.id"
          :class="{ active: item.originalIndex === props.activeTrackIndex }"
          @click="selectTrack(item.originalIndex)"
          @dblclick="playTrack(item.originalIndex)"
        >
          <span class="row-index">
            <input v-if="batchMode" v-model="selectedIds" type="checkbox" :value="item.track.id" :aria-label="`选择歌曲 ${item.track.title}`" :disabled="props.deleting || props.batchDownloading || !selectableIds.includes(item.track.id)" @click.stop @dblclick.stop />
            <span v-else-if="props.isPlaying && item.originalIndex === props.currentTrackIndex" class="playing-bars" aria-hidden="true">
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
            <span class="cover-container">
              <img class="cover-preview" :src="item.track.coverUrl" :alt="item.track.title" />
              <svg v-if="item.track.localPath" class="downloaded-badge" viewBox="0 0 1024 1024" role="img" aria-label="已下载" xmlns="http://www.w3.org/2000/svg">
                <path d="M459.755102 637.387755c-5.22449 0-10.44898-2.089796-14.628571-6.269388l-104.489796-104.489796c-8.359184-8.359184-8.359184-21.420408 0-29.779591 8.359184-8.359184 21.420408-8.359184 29.779592 0l104.489795 104.489796c8.359184 8.359184 8.359184 21.420408 0 29.779591-4.702041 4.179592-9.926531 6.269388-15.15102 6.269388z" fill="#cdcdcd" />
                <path d="M459.755102 637.387755c-5.22449 0-10.44898-2.089796-14.628571-6.269388-8.359184-8.359184-8.359184-21.420408 0-29.779591l208.979591-208.979592c8.359184-8.359184 21.420408-8.359184 29.779592 0 8.359184 8.359184 8.359184 21.420408 0 29.779592l-208.979592 208.979591c-4.702041 4.179592-9.926531 6.269388-15.15102 6.269388z" fill="#cdcdcd" />
                <path d="M512 929.959184c-230.4 0-417.959184-187.559184-417.959184-417.959184s187.559184-417.959184 417.959184-417.959184 417.959184 187.559184 417.959184 417.959184-187.559184 417.959184-417.959184 417.959184z m0-794.122449c-207.412245 0-376.163265 168.75102-376.163265 376.163265s168.75102 376.163265 376.163265 376.163265 376.163265-168.75102 376.163265-376.163265-168.75102-376.163265-376.163265-376.163265z" fill="#cdcdcd" />
              </svg>
            </span>
            <div class="track-meta">
              <p class="track-title">{{ item.track.title }}</p>
            </div>
          </div>
          <span class="row-artist">{{ item.track.artist }}</span>
          <span class="duration">
            <button type="button" class="track-action-button download-button" :disabled="props.downloadingTrackIds.includes(item.track.id) || !!item.track.localPath" :aria-busy="props.downloadingTrackIds.includes(item.track.id)" :title="item.track.localPath ? '已下载' : props.downloadingTrackIds.includes(item.track.id) ? '正在下载' : '下载歌曲'" aria-label="下载歌曲" @click.stop="emit('download', item.originalIndex)" @dblclick.stop>
              <span v-if="props.downloadingTrackIds.includes(item.track.id)" class="delete-spinner" aria-hidden="true"></span>
              <svg v-else-if="item.track.localPath" class="download-complete-icon" viewBox="0 0 1024 1024" width="20" height="20" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                <path d="M512 244.345V665 M325.191 495.515L514.124 665L703.058 495.515 M197.113 779.655H826.889" fill="none" stroke="#bfbfbf" stroke-width="42" stroke-linejoin="round" />
                <path d="M842.633 338.812c-86.956 0-157.443-70.487-157.443-157.443s70.487-157.443 157.443-157.443 157.443 70.487 157.443 157.443-70.487 157.443-157.443 157.443zM780.41 164.615l-13.319 14.485 59.813 54.916 87.916-84.941-13.682-14.138-74.581 72.046-46.147-42.353z" fill="#bfbfbf" />
              </svg>
              <svg v-else viewBox="0 0 1024 1024" width="20" height="20" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                <path d="M544.256 605.184l244.224-244.224a31.744 31.744 0 0 1 45.056 45.056l-295.424 295.424a36.864 36.864 0 0 1-51.2 0L190.464 406.528a31.744 31.744 0 1 1 45.056-45.056l244.224 244.224V111.104a32.256 32.256 0 1 1 64 0zM153.6 902.656a32.256 32.256 0 0 1 0-64h716.8a32.256 32.256 0 0 1 0 64z" fill="currentColor" />
              </svg>
            </button>
            <button
              type="button"
              class="track-action-button delete-button"
              :disabled="props.deletingTrackIds.includes(item.track.id)"
              @click.stop="deleteTrack(item.originalIndex)"
              :title="props.deletingTrackIds.includes(item.track.id) ? '正在删除' : '删除歌曲'"
              :aria-label="props.deletingTrackIds.includes(item.track.id) ? '正在删除' : '删除歌曲'"
            >
              <template v-if="props.deletingTrackIds.includes(item.track.id)">
                <span class="delete-spinner" aria-hidden="true"></span>
              </template>
              <template v-else>
                <svg
                  class="icon"
                  viewBox="0 0 1024 1024"
                  xmlns="http://www.w3.org/2000/svg"
                  width="20"
                  height="20"
                >
                  <path
                    d="M202.666667 256h-42.666667a32 32 0 0 1 0-64h704a32 32 0 0 1 0 64H266.666667v565.333333a53.333333 53.333333 0 0 0 53.333333 53.333334h384a53.333333 53.333333 0 0 0 53.333333-53.333334V352a32 32 0 0 1 64 0v469.333333c0 64.8-52.533333 117.333333-117.333333 117.333334H320c-64.8 0-117.333333-52.533333-117.333333-117.333334V256z m224-106.666667a32 32 0 0 1 0-64h170.666666a32 32 0 0 1 0 64H426.666667z m-32 288a32 32 0 0 1 64 0v256a32 32 0 0 1-64 0V437.333333z m170.666666 0a32 32 0 0 1 64 0v256a32 32 0 0 1-64 0V437.333333z"
                    fill="currentColor"
                  />
                </svg>
              </template>
            </button>
          </span>
        </li>
      </ul>
    </div>
  </section>
</template>

<style scoped>
.batch-delete-link { padding: .25rem 0; border: none; background: transparent; color: var(--theme-accent, #2563eb); font: inherit; font-size: .84rem; cursor: pointer; white-space: nowrap; }
.batch-delete-link:hover:not(:disabled) { text-decoration: underline; }
.batch-delete-link.danger { color: #ef4444; }
.batch-delete-link:disabled { opacity: .45; cursor: default; }
.batch-delete-link:focus-visible { outline: 2px solid #818cf8; outline-offset: 3px; }
input[type="checkbox"] {
  --checkbox-color: #334155;
  --checkbox-mark: #fff;
  appearance: none;
  display: inline-grid;
  place-content: center;
  width: 1rem;
  height: 1rem;
  margin: 0;
  box-sizing: border-box;
  border: 1.5px solid var(--checkbox-color);
  border-radius: .2rem;
  background: transparent;
  vertical-align: middle;
  cursor: pointer;
}
input[type="checkbox"]:checked,
input[type="checkbox"]:indeterminate { background: var(--checkbox-color); }
input[type="checkbox"]:checked::before {
  content: "";
  width: .25rem;
  height: .45rem;
  border: solid var(--checkbox-mark);
  border-width: 0 2px 2px 0;
  transform: translateY(-1px) rotate(45deg);
}
input[type="checkbox"]:indeterminate::before {
  content: "";
  width: .5rem;
  height: 2px;
  background: var(--checkbox-mark);
  border: none;
  transform: none;
}
input[type="checkbox"]:focus-visible { outline: 2px solid var(--checkbox-color); outline-offset: 3px; }
input[type="checkbox"]:disabled { opacity: .4; cursor: not-allowed; }
:global(:root[data-theme="dark"]) input[type="checkbox"] {
  --checkbox-color: #e2e8f0;
  --checkbox-mark: #1b202b;
}
.playlist-upload-button:disabled { opacity: .45; cursor: not-allowed; pointer-events: none; }
.upload-progress-button {
  display: inline-flex; align-items: center; justify-content: center;
  width: 2rem; height: 2rem; padding: .25rem; border: none; border-radius: .5rem;
  background: transparent; color: var(--theme-accent, #4f46e5); cursor: pointer;
}
.upload-progress-button:hover { background: var(--theme-tint, rgba(79, 70, 229, .08)); }
.upload-progress-button:focus-visible { outline: 2px solid #818cf8; outline-offset: 3px; }
.upload-progress-button svg { width: 1.4rem; height: 1.4rem; overflow: visible; }
.upload-progress-ring { transform-origin: 12px 12px; animation: upload-progress-spin 1.2s linear infinite; }
@keyframes upload-progress-spin { to { transform: rotate(360deg); } }
@media (prefers-reduced-motion: reduce) { .upload-progress-ring { animation: none; } }
.playlist-panel {
  --track-columns: 48px minmax(0, 1.1fr) minmax(0, 1fr) 120px;
  background: rgba(255, 255, 255, 0.88);
  border-radius: 0;
  padding: 0.7rem;
  display: flex;
  flex-direction: column;
  gap: 1.4rem;
  flex: 1;
  overflow: hidden;
  height: 100%;
  min-height: 0;
}

.playlist-body::-webkit-scrollbar {
  width: 10px;
}

.playlist-body::-webkit-scrollbar-track {
  background: rgba(245, 246, 251, 0.5);
  border-radius: 0;
}

.playlist-body::-webkit-scrollbar-thumb {
  background: rgba(99, 102, 241, 0.4);
  border-radius: 0;
  transition: background 0.2s ease;
}

.playlist-body::-webkit-scrollbar-thumb:hover {
  background: rgba(99, 102, 241, 0.6);
}

.playlist-body::-webkit-scrollbar-button {
  display: none;
}

.playlist-body {
  overflow-y: auto;
  scrollbar-color: rgba(99, 102, 241, 0.4) rgba(245, 246, 251, 0.5);
  scrollbar-width: thin;
  scrollbar-gutter: stable;
  box-sizing: border-box;
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
  position: sticky;
  top: 0;
  z-index: 1;
  background: #fff;
  margin-bottom: 0.6rem;
  display: grid;
  grid-template-columns: var(--track-columns);
  align-items: center;
  gap: 0.75rem;
  padding: 0 0.9rem 0.35rem;
  border-bottom: 1px solid rgba(15, 23, 42, 0.08);
  color: #8e95a1;
  font-size: 0.82rem;
  font-weight: 400;
}

.col-actions {
  padding-left: 0.375rem;
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
  grid-template-columns: var(--track-columns);
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

.cover-container { position: relative; width: 3rem; height: 3rem; flex-shrink: 0; }
.downloaded-badge { position: absolute; right: -3px; bottom: -3px; width: 17px; height: 17px; background: #374151; border-radius: 50%; }
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

.row-actions {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  justify-self: center;
  width: 100%;
}

.track-action-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 9999px;
  background: transparent;
  border: none;
  cursor: pointer;
  color: #6b7280;
  transition: transform 0.15s ease;
  width: 28px;
  height: 28px;
  padding: 0;
  flex: 0 0 28px;
}

.track-action-button svg {
  width: 16px;
  height: 16px;
}

.track-action-button .download-complete-icon {
  width: 23px;
  height: 23px;
}

.track-action-button:hover {
  transform: translateY(-1px);
}
.track-action-button:disabled {
  cursor: not-allowed;
}

.download-button:disabled {
  color: #cbd5e1;
  transform: none;
}

.delete-spinner {
  width: 12px;
  height: 12px;
  border: 2px solid currentColor;
  border-top-color: transparent;
  border-radius: 50%;
  animation: delete-spinner-spin 0.8s linear infinite;
}

@keyframes delete-spinner-spin {
  to {
    transform: rotate(360deg);
  }
}
.track-action-button:focus-visible {
  outline: none;
  box-shadow: 0 0 0 4px rgba(239, 68, 68, 0.15);
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
  display: flex;
  align-items: center;
  gap: 2px;
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
