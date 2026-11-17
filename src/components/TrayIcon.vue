<script setup lang="ts">
defineProps<{ name: string }>();
const paths: Record<string, string> = {
  open: 'M4 5h16v14H4z M4 9h16 M8 5v4',
  play: 'm8 5 11 7-11 7z',
  pause: 'M8 5v14 M16 5v14',
  previous: 'M5 5v14 M19 5 8 12l11 7z',
  next: 'M19 5v14 M5 5l11 7-11 7z',
  volume: 'M11 4 6 8H3v8h3l5 4z M15 8a6 6 0 0 1 0 8 M18 5a10 10 0 0 1 0 14',
  mute: 'M11 4 6 8H3v8h3l5 4z M16 9l6 6 M22 9l-6 6',
  quit: 'M12 3v9 M6 6a9 9 0 1 0 12 0',
  check: 'm5 12 4 4L19 6',
};
</script>

<template>
  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path :d="paths[name]" /></svg>
</template>
