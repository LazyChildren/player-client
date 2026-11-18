<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow, type Window as TauriWindow } from "@tauri-apps/api/window";
import { onMounted, onUnmounted, ref } from "vue";
import { useTheme } from "../utils/theme";
import type { AppSettings } from "../types/music";

const emit = defineEmits<{
  (event: "open-settings"): void;
}>();

const { theme, toggleTheme } = useTheme();
const isMaximized = ref(false);
let windowHandle: TauriWindow | null = null;
let unlistenWindowEvents: Array<() => void> = [];

async function ensureWindowHandle() {
  if (windowHandle) {
    return true;
  }

  try {
    windowHandle = getCurrentWindow();
    return true;
  } catch (error) {
    console.warn("Tauri window API not available", error);
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

  if (!windowHandle) return;

  const unlistenResized = await windowHandle.onResized(async () => {
    await refreshMaximizeState();
  });

  const unlistenMoved = await windowHandle.onMoved(async () => {
    await refreshMaximizeState();
  });

  unlistenWindowEvents = [unlistenResized, unlistenMoved];
});

onUnmounted(() => {
  for (const unlisten of unlistenWindowEvents) {
    unlisten();
  }
  unlistenWindowEvents = [];
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
  
  try {
    const settings = await invoke<AppSettings>("load_settings");
    const behavior = settings.closeWindowBehavior || "minimize";
    
    if (behavior === "minimize") {
      // 隐藏到托盘
      await invoke("hide_window");
    } else {
      // 直接关闭应用
      await windowHandle?.close();
    }
  } catch (error) {
    console.warn("Failed to load settings, defaulting to hide", error);
    await invoke("hide_window");
  }
}
</script>

<template>
  <div class="window-controls">
    <button
      class="window-button theme-button"
      type="button"
      :title="theme === 'dark' ? '切换浅色主题' : '切换深色主题'"
      :aria-label="theme === 'dark' ? '切换浅色主题' : '切换深色主题'"
      :aria-pressed="theme === 'dark'"
      @click="toggleTheme"
    >
      <svg viewBox="0 0 1024 1024" width="24" height="24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
        <path v-if="theme === 'light'" d="M539.9552 225.28c33.4848 0 39.936 45.568 18.944 71.68a204.8 204.8 0 0 0 159.744 332.8c34.2016 0 66.6624 33.792 45.3632 60.6208A285.2864 285.2864 0 0 1 539.9552 798.72a286.72 286.72 0 1 1 0-573.44z" fill="currentColor" />
        <path v-else d="M512 706.56c22.528 0 40.96 18.432 40.96 40.96v40.96a40.96 40.96 0 0 1-81.92 0v-40.96a40.96 40.96 0 0 1 40.96-40.96z m-195.584-57.0368a40.96 40.96 0 0 1 58.0608 57.9584l-28.9792 28.9792a40.96 40.96 0 1 1-57.9584-58.0608l28.8768-28.8768z m333.1072 0a40.96 40.96 0 0 1 57.9584 0l28.9792 28.8768a40.96 40.96 0 0 1-57.9584 58.0608l-28.9792-28.9792a40.96 40.96 0 0 1 0-57.9584zM511.7952 388.8128a122.88 122.88 0 1 1-0.1024 245.8624 122.88 122.88 0 0 1 0-245.76zM276.48 471.04a40.96 40.96 0 1 1 0 81.92h-40.96a40.96 40.96 0 0 1 0-81.92h40.96z m512 0a40.96 40.96 0 1 1 0 81.92h-40.96a40.96 40.96 0 0 1 0-81.92h40.96zM287.5392 287.4368a40.96 40.96 0 0 1 57.9584 0l28.9792 28.9792a40.96 40.96 0 0 1-58.0608 57.9584l-28.8768-28.8768a40.96 40.96 0 0 1 0-58.0608z m390.9632 0a40.96 40.96 0 0 1 57.9584 58.0608l-28.9792 28.8768a40.96 40.96 0 1 1-57.9584-57.9584l28.9792-28.9792zM512 194.56c22.528 0 40.96 18.432 40.96 40.96v40.96a40.96 40.96 0 0 1-81.92 0v-40.96a40.96 40.96 0 0 1 40.96-40.96z" fill="currentColor" />
      </svg>
    </button>
    <button
      class="window-button"
      type="button"
      title="设置"
      aria-label="设置"
      @click="emit('open-settings')"
    >
      <svg t="1779688208128" class="icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="3592" id="mx_n_1779688208128" width="16" height="16">
        <path d="M469.333333 60.693333a85.333333 85.333333 0 0 1 85.333334 0l326.826666 188.714667a85.333333 85.333333 0 0 1 42.666667 73.898667v377.386666a85.333333 85.333333 0 0 1-42.666667 73.898667L554.666667 963.306667a85.333333 85.333333 0 0 1-85.333334 0L142.506667 774.592a85.333333 85.333333 0 0 1-42.666667-73.898667v-377.386666a85.333333 85.333333 0 0 1 42.666667-73.898667z m42.666667 73.898667L185.173333 323.306667v377.386666L512 889.408l326.826667-188.714667v-377.386666L512 134.592zM512 341.333333a170.666667 170.666667 0 1 1 0 341.333334 170.666667 170.666667 0 0 1 0-341.333334z m0 85.333334a85.333333 85.333333 0 1 0 0 170.666666 85.333333 85.333333 0 0 0 0-170.666666z" fill="currentColor" p-id="3593"></path>
      </svg>
    </button>

    <button
      class="window-button"
      type="button"
      @click="minimize"
      title="最小化"
      aria-label="最小化"
    >
      <svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.2">
        <rect x="1" y="5.5" width="10" height="0.1" rx="0.5" />
      </svg>
    </button>

    <button
      class="window-button"
      type="button"
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
      type="button"
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
  display: flex;
  gap: 0;
  align-items: center;
  justify-content: flex-end;
  padding: 0;
  margin: 0;
  background: transparent;
  -webkit-app-region: no-drag;
}

.window-button {
  all: unset;
  width: 2.1rem;
  height: 2.1rem;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 0;
  background: transparent;
  color: rgba(31, 41, 55, 0.5);
  cursor: pointer;
  transition: color 0.18s ease, transform 0.18s ease;
  -webkit-app-region: no-drag;
}

.window-button:hover {
  color: rgba(17, 24, 39, 0.92);
  transform: translateY(-1px);
}

.window-button svg {
  display: block;
}

.close-button {
  color: rgba(239, 68, 68, 0.55);
}

.close-button:hover {
  color: rgba(185, 28, 28, 0.95);
}
</style>
