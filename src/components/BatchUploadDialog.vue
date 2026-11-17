<script setup lang="ts">
import { ref } from "vue";
import { parseAudioMetadata } from "../utils/audioMetadata";

export type BatchUploadItem = { file: File; title: string; artist: string };
defineProps<{ open: boolean }>();
const emit = defineEmits<{
  (event: "cancel"): void;
  (event: "single"): void;
  (event: "upload", items: BatchUploadItem[]): void;
}>();
const rows = ref<BatchUploadItem[]>([]);
const parsing = ref(false);
const error = ref("");
async function selectFiles(event: Event) {
  const input = event.target as HTMLInputElement;
  const files = Array.from(input.files ?? []);
  input.value = "";
  if (!files.length) return;
  if (files.some(file => !/\.(mp3|ogg)$/i.test(file.name))) {
    error.value = "请选择 MP3 或 OGG 格式的音乐文件";
    return;
  }
  parsing.value = true;
  error.value = "";
  try {
    const selected: BatchUploadItem[] = [];
    for (const file of files) {
      const metadata = await parseAudioMetadata(file).catch(() => ({ title: "", artist: "" }));
      const parts = file.name.replace(/\.(mp3|ogg)$/i, "").trim().split("-");
      selected.push({ file, title: metadata.title || parts[0].trim(),
        artist: metadata.artist || (metadata.title ? "" : parts.slice(1).join("-").trim()) });
    }
    rows.value = selected;
  } finally {
    parsing.value = false;
  }
}
function confirm() {
  if (parsing.value || !rows.value.length) return;
  if (rows.value.some(row => !row.title.trim())) {
    error.value = "请填写每首歌曲的歌曲名";
    return;
  }
  emit("upload", rows.value.map(row => ({ ...row, title: row.title.trim(), artist: row.artist.trim() })));
  rows.value = [];
}
</script>

<template>
  <Teleport to="body">
    <div v-if="open" class="batch-overlay" @click.self="!parsing && emit('cancel')">
      <section class="batch-dialog upload-dialog" role="dialog" aria-modal="true" aria-labelledby="batch-title">
        <header>
          <h2 id="batch-title">批量上传歌曲</h2>
          <button class="single-mode-link" :disabled="parsing" @click="emit('single')">单曲上传</button>
        </header>
        <label class="choose-files">选择本地文件
          <input type="file" multiple accept=".mp3,.ogg,audio/mpeg,audio/ogg" :disabled="parsing" @change="selectFiles" />
        </label>
        <p v-if="parsing" role="status">正在读取歌曲信息…</p>
        <p v-if="error" role="alert">{{ error }}</p>
        <div v-if="rows.length" class="batch-table">
          <table>
            <thead><tr><th>序号</th><th>歌曲名</th><th>艺术家</th></tr></thead>
            <tbody><tr v-for="(row, index) in rows" :key="index">
              <td>{{ index + 1 }}</td>
              <td class="upload-field"><input v-model="row.title" :aria-label="`第 ${index + 1} 首歌曲名`" :title="row.file.name" :disabled="parsing" /></td>
              <td class="upload-field"><input v-model="row.artist" :aria-label="`第 ${index + 1} 首艺术家`" :disabled="parsing" /></td>
            </tr></tbody>
          </table>
        </div>
        <p v-if="rows.length">确认后在后台依次上传，可继续浏览和播放音乐。</p>
        <footer>
          <button :disabled="parsing" @click="emit('cancel')">取消</button>
          <button :disabled="parsing || !rows.length" @click="confirm">确认上传（{{ rows.length }} 首）</button>
        </footer>
      </section>
    </div>
  </Teleport>
</template>

<style scoped>
.batch-overlay { position: fixed; inset: 0; z-index: 10000; background: #0006; display: flex; align-items: center; justify-content: center; padding: 1.25rem; -webkit-app-region: no-drag; }
.batch-dialog { width: min(100%, 48rem); max-height: 85vh; overflow-y: auto; padding: 1.5rem; border-radius: 1rem; background: #fff; color: #334155; box-sizing: border-box; }
header, footer { display: flex; align-items: center; justify-content: space-between; gap: 1rem; }
h2 { font-size: 1.1rem; }
button { padding: .6rem .9rem; border: 1px solid #818cf8; border-radius: .5rem; background: transparent; color: inherit; cursor: pointer; }
button:disabled { opacity: .5; cursor: default; }
.single-mode-link { padding: 0; border: none; border-radius: 0; background: transparent; color: #2563eb; font: inherit; font-size: .84rem; }
.single-mode-link:hover:not(:disabled) { text-decoration: underline; }
.single-mode-link:focus-visible { outline: 2px solid #818cf8; outline-offset: 4px; }
:global(:root[data-theme="dark"]) .single-mode-link { color: #60a5fa; }
.choose-files { display: grid; gap: .75rem; margin: 1rem 0; }
.batch-table { max-height: 40vh; overflow: auto; }
table { width: 100%; border-collapse: collapse; }
th, td { padding: .5rem; text-align: left; }
td.upload-field { display: table-cell; }
input:not([type="file"]) { width: 100%; min-width: 0; box-sizing: border-box; padding: .6rem; border: 1px solid #94a3b8; border-radius: .4rem; font: inherit; }
p { font-size: .85rem; }
</style>
