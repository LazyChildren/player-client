<script setup lang="ts">
import { getCurrentWindow, type Window as TauriWindow } from "@tauri-apps/api/window";
import { onMounted, ref } from "vue";

const isMaximized = ref(false);
let windowHandle: TauriWindow | null = null;

async function ensureWindowHandle() {
  if (windowHandle) {
    return true;
  }

  try {
    windowHandle = getCurrentWindow();
    return true;
  } catch (e) {
    console.warn("Tauri window API not available", e);
    windowHandle = null;
    return false;
  }
}

async function refreshMaximizeState() {
  if (!windowHandle) return;
  try {
    isMaximized.value = await windowHandle.isMaximized();
  } catch {
    isMaximized.value = false;
  }
}

onMounted(async () => {
  if (!(await ensureWindowHandle())) {
    return;
  }

  await refreshMaximizeState();
});

async function minimize() {
  if (!(await ensureWindowHandle())) return;
  await windowHandle?.minimize();
}

async function toggleMaximize() {
  if (!(await ensureWindowHandle())) return;

  try {
    await windowHandle?.toggleMaximize();
  } catch {
    if (!windowHandle) return;
    const maximized = await windowHandle.isMaximized();
    if (maximized) {
      await windowHandle.unmaximize();
    } else {
      await windowHandle.maximize();
    }
  }

  await refreshMaximizeState();
}

async function closeWindow() {
  if (!(await ensureWindowHandle())) return;
  await windowHandle?.close();
}
</script>

<template>
  <div class="window-controls">
    <button
      class="window-button"
      @click="minimize"
      title="最小化"
      aria-label="最小化"
    >
      <svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.2">
        <rect x="1" y="5.5" width="10" height="1" rx="0.5" />
      </svg>
    </button>

    <button
      class="window-button"
      @click="toggleMaximize"
      :title="isMaximized ? '还原' : '最大化'"
      :aria-label="isMaximized ? '还原' : '最大化'"
    >
      <svg v-if="isMaximized" width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.2">
        <rect x="3" y="3" width="7" height="7" rx="1" />
        <path d="M3 5H2V2a1 1 0 0 1 1-1h5v1" />
      </svg>
      <svg v-else width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.2">
        <rect x="1.5" y="1.5" width="9" height="9" rx="1.5" />
      </svg>
    </button>

    <button
      class="window-button close-button"
      @click="closeWindow"
      title="关闭"
      aria-label="关闭"
    >
      <svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round">
        <path d="M2 2l8 8M10 2l-8 8" />
      </svg>
    </button>
  </div>
</template>

<style scoped>
.window-controls {
  position: absolute;
  top: 0;
  right: 0;
  display: flex;
  gap: 0;
  align-items: flex-start;
  justify-content: flex-end;
  padding: 0;
  margin: 0;
  background: transparent;
  -webkit-app-region: no-drag;
}
.window-button {
  all: unset;
  width: 2.4rem;
  height: 2.4rem;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 0;
  background: transparent;
  color: inherit;
  cursor: pointer;
  transition: background 0.18s ease, transform 0.18s ease, color 0.18s ease;
  -webkit-app-region: no-drag;
}
.window-button:hover {
  background: rgba(0, 0, 0, 0.08);
  transform: translateY(-1px);
}
.window-button svg {
  display: block;
}
.close-button {
  color: inherit;
}
.close-button:hover {
  background: rgba(239, 68, 68, 0.12);
  color: #dc2626;
}
</style>
