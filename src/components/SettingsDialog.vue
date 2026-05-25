<script setup lang="ts">
import { reactive, watch } from "vue";

type Settings = {
  serverIp: string;
  serverPort: string;
};

const props = defineProps<{
  open: boolean;
  initialSettings: Settings;
}>();

const emit = defineEmits<{
  (event: "save", settings: Settings): void;
  (event: "cancel"): void;
}>();

const draft = reactive<Settings>({
  serverIp: "",
  serverPort: "",
});

function syncDraft(settings: Settings) {
  draft.serverIp = settings.serverIp;
  draft.serverPort = settings.serverPort;
}

watch(
  () => props.open,
  (isOpen) => {
    if (isOpen) {
      syncDraft(props.initialSettings);
    }
  },
  { immediate: true }
);

watch(
  () => props.initialSettings,
  (settings) => {
    if (props.open) {
      syncDraft(settings);
    }
  },
  { deep: true }
);

function handleSave() {
  emit("save", {
    serverIp: draft.serverIp.trim(),
    serverPort: draft.serverPort.trim(),
  });
}

function handleCancel() {
  emit("cancel");
}
</script>

<template>
  <Teleport to="body">
    <Transition name="settings-fade">
      <div v-if="open" class="settings-overlay" @click.self="handleCancel">
        <section class="settings-dialog" role="dialog" aria-modal="true" aria-labelledby="settings-title">
          <header class="settings-header">
            <div>
              <p class="settings-eyebrow">系统设置</p>
              <h2 id="settings-title">服务器连接</h2>
            </div>

            <button type="button" class="dialog-close-button" aria-label="关闭设置" @click="handleCancel">
              <svg width="14" height="14" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round">
                <path d="M2 2l8 8M10 2l-8 8" />
              </svg>
            </button>
          </header>

          <div class="settings-body">
            <label class="setting-field">
              <span>服务器 IP</span>
              <input v-model="draft.serverIp" type="text" placeholder="例如 127.0.0.1" />
            </label>

            <label class="setting-field">
              <span>端口</span>
              <input v-model="draft.serverPort" type="text" placeholder="例如 3000" />
            </label>
          </div>

          <footer class="settings-actions">
            <button type="button" class="action-button secondary" @click="handleCancel">取消</button>
            <button type="button" class="action-button primary" @click="handleSave">保存</button>
          </footer>
        </section>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.settings-overlay {
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

.settings-dialog {
  width: min(100%, 28rem);
  border-radius: 1.15rem;
  background: rgba(255, 255, 255, 0.96);
  box-shadow:
    0 0 0 1px rgba(79, 70, 229, 0.18),
    0 28px 78px rgba(15, 23, 42, 0.28);
  border: 1px solid rgba(79, 70, 229, 0.22);
  overflow: hidden;
  -webkit-app-region: no-drag;
  animation: settings-pop 0.18s ease-out;
}

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

.settings-fade-enter-active,
.settings-fade-leave-active {
  transition: opacity 0.18s ease;
}

.settings-fade-enter-from,
.settings-fade-leave-to {
  opacity: 0;
}

@keyframes settings-pop {
  from {
    transform: translateY(8px) scale(0.98);
  }

  to {
    transform: translateY(0) scale(1);
  }
}
</style>
