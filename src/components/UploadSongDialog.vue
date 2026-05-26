<script setup lang="ts">
import { reactive, watch } from "vue";

export type UploadSongDraft = {
  fileName: string;
  filePath: string;
  title: string;
  artist: string;
  coverUrl: string;
};

const props = defineProps<{
  open: boolean;
  draft: UploadSongDraft;
}>();

const emit = defineEmits<{
  (event: "update:draft", draft: UploadSongDraft): void;
  (event: "cancel"): void;
}>();

const localDraft = reactive<UploadSongDraft>({
  fileName: "",
  filePath: "",
  title: "",
  artist: "",
  coverUrl: "",
});

function syncDraft(source: UploadSongDraft) {
  localDraft.fileName = source.fileName;
  localDraft.filePath = source.filePath;
  localDraft.title = source.title;
  localDraft.artist = source.artist;
  localDraft.coverUrl = source.coverUrl;
}

watch(
  () => props.draft,
  (draft) => {
    syncDraft(draft);
  },
  { immediate: true, deep: true }
);

watch(
  localDraft,
  () => {
    emit("update:draft", {
      fileName: localDraft.fileName,
      filePath: localDraft.filePath,
      title: localDraft.title,
      artist: localDraft.artist,
      coverUrl: localDraft.coverUrl,
    });
  },
  { deep: true }
);

function handleCancel() {
  emit("cancel");
}

function handleFetchCoverLink() {
  // Reserved for future cover lookup behavior.
}
</script>

<template>
  <Teleport to="body">
    <Transition name="upload-fade">
      <div v-if="open" class="upload-overlay" @click.self="handleCancel">
        <section class="upload-dialog" role="dialog" aria-modal="true" aria-labelledby="upload-song-title">
          <header class="upload-header">
            <div>
              <p class="upload-eyebrow">本地音乐上传</p>
              <h2 id="upload-song-title">上传歌曲</h2>
              <p class="upload-file-name">{{ localDraft.fileName }}</p>
            </div>

            <button type="button" class="dialog-close-button" aria-label="关闭上传窗口" @click="handleCancel">
              <svg width="14" height="14" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round">
                <path d="M2 2l8 8M10 2l-8 8" />
              </svg>
            </button>
          </header>

          <div class="upload-body">
            <div class="cover-panel">
              <div class="cover-frame">
                <span>封面预留</span>
              </div>
              <p class="cover-hint">暂不支持上传歌曲封面</p>
            </div>

            <div class="upload-form">
              <label class="upload-field">
                <span>歌曲名</span>
                <input v-model="localDraft.title" type="text" placeholder="请输入歌曲名" />
              </label>

              <label class="upload-field">
                <span>艺术家</span>
                <input v-model="localDraft.artist" type="text" placeholder="请输入艺术家" />
              </label>

              <label class="upload-field">
                <span>歌曲封面</span>
                <div class="cover-link-row">
                  <input v-model="localDraft.coverUrl" type="text" placeholder="暂不支持直接输入" disabled />
                  <button type="button" class="cover-link-button" aria-label="获取封面链接" title="获取封面链接" @click="handleFetchCoverLink">
                    <svg
                      class="cover-link-icon"
                      viewBox="0 0 1024 1024"
                      version="1.1"
                      xmlns="http://www.w3.org/2000/svg"
                      aria-hidden="true"
                    >
                      <path
                        d="M910.496 213.536C804.16 82.208 611.488 61.952 480.128 168.32l-100.768 81.6 50.336 62.176 100.768-81.6a225.984 225.984 0 1 1 284.448 351.264l-107.968 87.424 50.336 62.176 107.968-87.424a305.984 305.984 0 0 0 45.248-430.4zM516.352 823.552a225.984 225.984 0 1 1-284.448-351.264l110.976-89.856-50.336-62.176-110.976 89.856C50.24 516.448 29.984 709.152 136.32 840.48c106.336 131.328 299.04 151.584 430.368 45.248l105.12-85.12-50.336-62.176-105.12 85.12z"
                        fill="currentColor"
                      />
                      <path d="M676.16 353.28l51.232 61.44-343.552 286.304-51.2-61.44z" fill="currentColor" />
                    </svg>
                  </button>
                </div>
              </label>
            </div>
          </div>
        </section>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.upload-overlay {
  position: fixed;
  inset: 0;
  z-index: 10000;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 1.25rem;
  box-sizing: border-box;
  background: rgba(15, 23, 42, 0.34);
  -webkit-app-region: no-drag;
}

.upload-dialog {
  width: min(100%, 48rem);
  border-radius: 1.15rem;
  background: rgba(255, 255, 255, 0.97);
  box-shadow:
    0 0 0 1px rgba(79, 70, 229, 0.18),
    0 28px 78px rgba(15, 23, 42, 0.28);
  border: 1px solid rgba(79, 70, 229, 0.22);
  overflow: hidden;
  -webkit-app-region: no-drag;
  animation: upload-pop 0.18s ease-out;
}

.upload-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 1rem;
  padding: 1.15rem 1.25rem 0.95rem;
  background: linear-gradient(135deg, rgba(79, 70, 229, 0.08), rgba(14, 165, 233, 0.05));
}

.upload-eyebrow {
  margin: 0 0 0.35rem;
  font-size: 0.74rem;
  letter-spacing: 0.18em;
  text-transform: uppercase;
  color: #64748b;
}

.upload-header h2 {
  margin: 0;
  font-size: 1.18rem;
  color: #0f172a;
}

.upload-file-name {
  margin: 0.45rem 0 0;
  color: #64748b;
  font-size: 0.84rem;
}

.dialog-close-button {
  all: unset;
  width: 2rem;
  height: 2rem;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 9999px;
  cursor: pointer;
  color: #64748b;
  transition: background 0.18s ease, color 0.18s ease, transform 0.18s ease;
}

.dialog-close-button:hover {
  background: rgba(148, 163, 184, 0.15);
  color: #0f172a;
  transform: translateY(-1px);
}

.upload-body {
  display: grid;
  grid-template-columns: 14rem minmax(0, 1fr);
  gap: 1.4rem;
  padding: 1.2rem 1.25rem 1.35rem;
}

.cover-panel {
  display: grid;
  gap: 0.75rem;
  align-content: start;
}

.cover-frame {
  aspect-ratio: 1 / 1;
  width: 100%;
  border-radius: 1rem;
  background:
    linear-gradient(135deg, rgba(79, 70, 229, 0.08), rgba(14, 165, 233, 0.05)),
    rgba(248, 250, 252, 0.96);
  border: 1px dashed rgba(99, 102, 241, 0.28);
  display: grid;
  place-items: center;
  color: #7c8aa0;
  font-size: 0.92rem;
  font-weight: 600;
}

.cover-hint {
  margin: 0;
  color: #64748b;
  font-size: 0.82rem;
  line-height: 1.45;
}

.upload-form {
  display: grid;
  gap: 1rem;
  align-content: start;
}

.upload-field {
  display: grid;
  gap: 0.45rem;
  color: #334155;
  font-size: 0.9rem;
}

.upload-field span {
  font-weight: 600;
}

.upload-field input {
  width: 100%;
  box-sizing: border-box;
  border: 1px solid rgba(148, 163, 184, 0.45);
  border-radius: 0.85rem;
  padding: 0.78rem 0.9rem;
  background: rgba(248, 250, 252, 0.96);
  color: #0f172a;
  font: inherit;
  transition: border-color 0.18s ease, box-shadow 0.18s ease, background 0.18s ease;
}

.upload-field input:focus {
  outline: none;
  border-color: rgba(79, 70, 229, 0.55);
  background: #fff;
  box-shadow: 0 0 0 4px rgba(79, 70, 229, 0.12);
}

.upload-field input:disabled {
  color: #94a3b8;
  background: rgba(241, 245, 249, 0.9);
  cursor: not-allowed;
}

.cover-link-row {
  display: grid;
  grid-template-columns: minmax(0, 1fr) 2.9rem;
  gap: 0.6rem;
  align-items: center;
}

.cover-link-button {
  all: unset;
  width: 2.9rem;
  height: 2.9rem;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 0.85rem;
  background: rgba(79, 70, 229, 0.08);
  color: #4f46e5;
  cursor: pointer;
  transition: background 0.18s ease, color 0.18s ease, transform 0.18s ease;
}

.cover-link-button:hover {
  background: rgba(79, 70, 229, 0.14);
  color: #3730a3;
  transform: translateY(-1px);
}

.cover-link-button:focus-visible {
  outline: none;
  box-shadow: 0 0 0 4px rgba(79, 70, 229, 0.12);
}

.cover-link-icon {
  width: 1rem;
  height: 1rem;
  display: block;
}

.upload-fade-enter-active,
.upload-fade-leave-active {
  transition: opacity 0.18s ease;
}

.upload-fade-enter-from,
.upload-fade-leave-to {
  opacity: 0;
}

@keyframes upload-pop {
  from {
    transform: translateY(8px) scale(0.98);
  }

  to {
    transform: translateY(0) scale(1);
  }
}

@media (max-width: 760px) {
  .upload-body {
    grid-template-columns: minmax(0, 1fr);
  }

  .cover-panel {
    max-width: 14rem;
  }
}
</style>
