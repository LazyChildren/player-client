<script setup lang="ts">
import { reactive, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { ElMessage } from "element-plus";
import type { AppSettings } from "../types/music";

const props = defineProps<{ initialSettings: AppSettings; saving: boolean }>();
const emit = defineEmits<{
  (event: "save", settings: AppSettings): void;
  (event: "cancel"): void;
}>();
const draft = reactive<AppSettings>({ ...props.initialSettings });
const browsing = ref(false);
let edited = false;
watch(() => props.initialSettings, settings => {
  if (!edited) Object.assign(draft, settings);
});
watch(draft, () => {
  edited = true;
  handleSave();
}, { deep: true, flush: "sync" });

async function browseDirectory() {
  if (browsing.value) return;
  browsing.value = true;
  try {
    const selected = await invoke<string | null>("choose_download_directory", {
      initialPath: draft.downloadDirectory || "",
    });
    if (selected !== null) draft.downloadDirectory = selected;
  } catch (error) {
    ElMessage.error(typeof error === "string" ? error : "无法打开文件夹选择器");
  } finally {
    browsing.value = false;
  }
}

function handleSave() {
  if (!draft.serverIp.trim() || !/^\d+$/.test(draft.serverPort.trim()) || Number(draft.serverPort) < 1 || Number(draft.serverPort) > 65535) return;
  emit("save", { ...draft, serverIp: draft.serverIp.trim(), serverPort: draft.serverPort.trim(),
    downloadDirectory: draft.downloadDirectory?.trim() || "" });
}
</script>

<template>
  <section class="settings-page" aria-labelledby="settings-title">
    <form class="settings-content" @submit.prevent>
      <header class="settings-header">
        <div>
          <p class="settings-eyebrow">系统设置</p>
          <h2 id="settings-title">设置</h2>
        </div>
        <button type="button" class="back-button" @click="emit('cancel')">
          <svg viewBox="0 0 1024 1024" width="16" height="16" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
            <path d="M100.608 341.632l196.096-196.16 60.352 60.352L264.064 298.88h365.312a288 288 0 1 1 0 576H362.688v-85.312h266.688a202.688 202.688 0 0 0 0-405.376h-365.44L357.056 477.44l-60.352 60.352-196.096-196.096z" fill="#bfbfbf" />
          </svg>
          <span>返回歌单</span>
        </button>
      </header>
      <fieldset class="settings-body">
        <p class="settings-hint" role="status">{{ saving ? '正在保存…' : '修改后自动保存' }}</p>
        <label class="setting-field">
          <span>服务器 IP</span>
          <input v-model="draft.serverIp" required type="text" placeholder="例如 127.0.0.1" />
        </label>
        <label class="setting-field">
          <span>端口</span>
          <input v-model="draft.serverPort" required type="text" inputmode="numeric" placeholder="例如 3000" />
        </label>
        <div class="setting-field">
          <label for="download-directory">歌曲下载目录</label>
          <div class="directory-picker">
            <input id="download-directory" v-model="draft.downloadDirectory" type="text" placeholder="留空使用系统下载目录 / Simple Music" aria-describedby="directory-hint" />
            <button type="button" class="action-button secondary browse-button" :disabled="browsing" :aria-busy="browsing" @click="browseDirectory">{{ browsing ? '正在打开…' : '浏览' }}</button>
          </div>
          <p id="directory-hint" class="settings-hint">选择歌曲下载保存的文件夹，也可以手动填写完整路径。</p>
        </div>
        <fieldset class="setting-field">
          <legend>关闭窗口行为</legend>
          <label class="radio-option"><input v-model="draft.closeWindowBehavior" type="radio" value="minimize" name="closeWindowBehavior" /><span>最小化到托盘</span></label>
          <label class="radio-option"><input v-model="draft.closeWindowBehavior" type="radio" value="exit" name="closeWindowBehavior" /><span>直接关闭应用程序</span></label>
        </fieldset>
      </fieldset>

    </form>
  </section>
</template>

<style scoped>


.settings-page { flex: 1; min-height: 0; overflow-y: auto; background: rgba(255, 255, 255, 0.88); padding: 1.5rem 2rem; box-sizing: border-box; }
.settings-content { width: 100%; margin: 0; }
.settings-header { border-bottom: 1px solid rgba(148, 163, 184, 0.2); }
.directory-picker { display: flex; align-items: center; gap: 0.6rem; }
.directory-picker input { min-width: 0; flex: 1; }
.browse-button { flex-shrink: 0; }
.settings-hint { margin: 0; font-size: 0.8rem; color: #64748b; }
.action-button:disabled { opacity: 0.55; cursor: progress; }
.action-button:focus-visible { outline: 2px solid #818cf8; outline-offset: 3px; }

.back-button {
  display: inline-flex;
  align-items: center;
  gap: 0.35rem;
  padding: 0.4rem 0;
  border: none;
  background: transparent;
  color: #2563eb;
  font: inherit;
  font-size: 0.82rem;
  font-weight: 400;
  cursor: pointer;
}

.back-button:hover:not(:disabled) { color: #1d4ed8; }
.back-button:disabled { opacity: 0.55; cursor: progress; }
.back-button:focus-visible { outline: 2px solid #818cf8; outline-offset: 3px; }
:global(:root[data-theme="dark"]) .back-button { color: #60a5fa; }
:global(:root[data-theme="dark"]) .back-button:hover:not(:disabled) { color: var(--theme-accent); }

.settings-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 1rem;
  padding: 1.1rem 1.2rem 0.9rem;
  background: linear-gradient(135deg, rgba(79, 70, 229, 0.08), rgba(14, 165, 233, 0.05));
}

.settings-eyebrow {
  margin: 0 0 0.35rem;
  font-size: 0.74rem;
  letter-spacing: 0.18em;
  text-transform: uppercase;
  color: #64748b;
}

.settings-header h2 {
  margin: 0;
  font-size: 1.1rem;
  color: #0f172a;
}



.settings-body {
  display: grid;
  gap: 1rem;
  padding: 1rem 1.2rem 1.2rem;
}

.setting-field {
  display: grid;
  gap: 0.45rem;
  color: #334155;
  font-size: 0.9rem;
}

.setting-field span {
  font-weight: 600;
}

.setting-field input {
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

.setting-field input:focus {
  outline: none;
  border-color: rgba(79, 70, 229, 0.55);
  background: #fff;
  box-shadow: 0 0 0 4px rgba(79, 70, 229, 0.12);
}

.setting-field fieldset {
  border: none;
  padding: 0;
  margin: 0;
}

.setting-field legend {
  font-weight: 600;
  margin-bottom: 0.45rem;
  padding: 0;
}

.radio-option {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin: 0.6rem 0;
  cursor: pointer;
  user-select: none;
}

.radio-option input[type="radio"] {
  width: 1.1rem;
  height: 1.1rem;
  cursor: pointer;
  accent-color: #4f46e5;
  flex-shrink: 0;
  outline: none;
  box-shadow: none;
  border: none;
  background: none;
  padding: 0;
  margin: 0;
}

.radio-option input[type="radio"]:focus {
  outline: none;
  box-shadow: none;
}

.radio-option span {
  font-weight: 400;
  color: #334155;
}

.settings-actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
  padding: 0 1.2rem 1.2rem;
}

.action-button {
  all: unset;
  min-width: 4.8rem;
  padding: 0.72rem 1rem;
  border-radius: 0.85rem;
  cursor: pointer;
  text-align: center;
  font-weight: 600;
  transition: transform 0.18s ease, background 0.18s ease, color 0.18s ease, box-shadow 0.18s ease;
}

.action-button:hover {
  transform: translateY(-1px);
}

.action-button.secondary {
  background: rgba(241, 245, 249, 0.95);
  color: #334155;
}

.action-button.secondary:hover {
  background: rgba(226, 232, 240, 0.95);
}

.action-button.primary {
  background: linear-gradient(135deg, #4f46e5, #2563eb);
  color: #fff;
  box-shadow: 0 10px 24px rgba(37, 99, 235, 0.22);
}

.action-button.primary:hover {
  box-shadow: 0 12px 28px rgba(37, 99, 235, 0.28);
}

</style>
