<script setup lang="ts">
import { onMounted, onUnmounted, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import TrayIcon from "./TrayIcon.vue";
import PlaybackOrderIcon from "./PlaybackOrderIcon.vue";

type TrayState = { playing: boolean; hasTracks: boolean; volume: number; playbackOrder: string; theme: string };
const state = ref<TrayState>({ playing: false, hasTracks: false, volume: 80, playbackOrder: "loop", theme: "light" });
const volume = ref(80);
const dragging = ref(false);
const error = ref("");
const menuRef = ref<HTMLElement | null>(null);
let sizeObserver: ResizeObserver | undefined;
const modes = [
  { value: "loop", label: "列表循环" },
  { value: "one", label: "单曲循环" },
  { value: "shuffle", label: "随机播放" },
];
let unlisten: UnlistenFn | undefined;
let disposed = false;
let actionChain: Promise<unknown> = Promise.resolve();
function update(next: TrayState) {
  state.value = next;
  if (!dragging.value) volume.value = next.volume;
}
function act(action: string, value?: number) {
  actionChain = actionChain.then(() => invoke("tray_action", { action, value }))
    .then(() => { error.value = ""; })
    .catch(() => { error.value = "操作失败，请重试"; });
}
function setVolume(event: Event) {
  volume.value = Number((event.target as HTMLInputElement).value);
  act("volume", volume.value);
}
function finishVolume() {
  if (!dragging.value) return;
  dragging.value = false;
  act("volume", volume.value);
}
function onKeydown(event: KeyboardEvent) {
  if (event.key === "Escape") act("hide");
}
watch(() => state.value.theme, theme => {
  document.documentElement.dataset.theme = theme;
  document.documentElement.classList.toggle("dark", theme === "dark");
  document.documentElement.style.colorScheme = theme;
}, { immediate: true });
onMounted(async () => {
  let previousHeight = 0;
  sizeObserver = new ResizeObserver(() => {
    const height = Math.ceil(menuRef.value?.getBoundingClientRect().height ?? 0);
    if (!height || height === previousHeight) return;
    previousHeight = height;
    void invoke("resize_tray_menu", { height }).catch(error => console.error("Failed to fit tray menu", error));
  });
  if (menuRef.value) sizeObserver.observe(menuRef.value);
  window.addEventListener("keydown", onKeydown);
  window.addEventListener("pointerup", finishVolume);
  window.addEventListener("pointercancel", finishVolume);
  window.addEventListener("blur", finishVolume);
  try {
    const stop = await listen<TrayState>("tray-state", ({ payload }) => update(payload));
    if (disposed) { stop(); return; }
    unlisten = stop;
    update(await invoke<TrayState>("get_tray_state"));
  } catch { error.value = "播放器连接失败"; }
});
onUnmounted(() => {
  disposed = true;
  sizeObserver?.disconnect();
  unlisten?.();
  window.removeEventListener("keydown", onKeydown);
  window.removeEventListener("pointerup", finishVolume);
  window.removeEventListener("pointercancel", finishVolume);
  window.removeEventListener("blur", finishVolume);
});
</script>

<template>
  <main ref="menuRef" class="tray-menu" :class="{ dark: state.theme === 'dark' }" @contextmenu.prevent>
    <header>Simple Music</header>
    <button @click="act('show')"><TrayIcon name="open" /><span>打开播放器</span></button>
    <div class="divider"></div>
    <button :disabled="!state.hasTracks" @click="act('tray-play-pause')"><TrayIcon :name="state.playing ? 'pause' : 'play'" /><span>{{ state.playing ? '暂停' : '播放' }}</span></button>
    <button :disabled="!state.hasTracks" @click="act('tray-previous')"><TrayIcon name="previous" /><span>上一首</span></button>
    <button :disabled="!state.hasTracks" @click="act('tray-next')"><TrayIcon name="next" /><span>下一首</span></button>
    <div class="divider"></div>
    <div class="volume-heading">
      <button class="mute-button" :aria-label="volume === 0 ? '取消静音' : '静音'" :aria-pressed="volume === 0" @click="act('tray-mute')"><TrayIcon :name="volume === 0 ? 'mute' : 'volume'" /></button>
      <label for="tray-volume">{{ volume === 0 ? '静音' : '音量' }}</label><span>{{ volume }}%</span>
    </div>
    <input id="tray-volume" class="volume-range" type="range" min="0" max="100" step="1" :value="volume" @pointerdown="dragging = true" @input="setVolume" @change="finishVolume" />
    <div class="divider"></div>
    <button v-for="mode in modes" :key="mode.value" :aria-pressed="state.playbackOrder === mode.value" @click="act(`tray-mode-${mode.value}`)">
      <PlaybackOrderIcon :mode="mode.value" /><span>{{ mode.label }}</span><TrayIcon v-if="state.playbackOrder === mode.value" class="selected-mark" name="check" />
    </button>
    <div class="divider"></div>
    <button class="quit-button" @click="act('quit')"><TrayIcon name="quit" /><span>退出</span></button>
    <p v-if="error" role="alert">{{ error }}</p>
  </main>
</template>

<style scoped>
.tray-menu { --tray-bg: #fff; --tray-text: #334155; --tray-muted: #64748b; --tray-hover: #f1f5f9; --tray-border: #e2e8f0; box-sizing: border-box; width: 100%; margin: 0; padding: 10px; background: var(--tray-bg); color: var(--tray-text); font: 13px/1.4 "Segoe UI", "Microsoft YaHei", sans-serif; user-select: none; -webkit-app-region: no-drag; }
.tray-menu.dark { --tray-bg: #1b202b; --tray-text: #e2e8f0; --tray-muted: #a0aec0; --tray-hover: #2b3342; --tray-border: #354052; }
header { padding: 4px 10px 8px; color: var(--tray-muted); font-size: 11px; letter-spacing: .06em; }
button { display: flex; align-items: center; gap: 12px; width: 100%; min-height: 34px; padding: 7px 10px; border: 0; border-radius: 6px; background: transparent; color: inherit; font: inherit; text-align: left; cursor: pointer; }
button:hover:not(:disabled) { background: var(--tray-hover); }
button:disabled { opacity: .35; cursor: default; }
button:focus-visible { outline: 2px solid var(--tray-text); outline-offset: -2px; }
svg { width: 18px; height: 18px; flex-shrink: 0; }
.selected-mark { margin-left: auto; width: 15px; }
.divider { height: 1px; margin: 5px 8px; background: var(--tray-border); }
.volume-heading { display: flex; align-items: center; gap: 8px; padding: 0 10px 0 2px; }
.volume-heading span { margin-left: auto; color: var(--tray-muted); font-variant-numeric: tabular-nums; }
.mute-button { width: 34px; padding: 8px; }
.volume-range { display: block; width: calc(100% - 24px); height: 5px; margin: 8px 12px 12px; accent-color: var(--tray-text); cursor: pointer; }
.quit-button { color: #dc5656; }
p { margin: 6px 10px; color: #dc5656; font-size: 11px; }
</style>
