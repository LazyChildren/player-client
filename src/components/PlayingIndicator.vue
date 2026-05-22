<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref, watch } from "vue";
import lottie, { type AnimationItem } from "lottie-web";
import playingSpectrumAnimation from "../assets/playing-spectrum.json";

const props = defineProps<{
  autoplay?: boolean;
}>();

const containerRef = ref<HTMLDivElement | null>(null);
let animation: AnimationItem | null = null;

function ensureAnimation() {
  if (!containerRef.value || animation) return;
  animation = lottie.loadAnimation({
    container: containerRef.value,
    renderer: "svg",
    loop: true,
    autoplay: props.autoplay ?? true,
    animationData: playingSpectrumAnimation,
  });
}

watch(
  () => props.autoplay,
  (autoplay) => {
    if (!animation) return;
    if (autoplay) {
      animation.play();
      return;
    }
    animation.pause();
  },
);

onMounted(() => {
  ensureAnimation();
});

onBeforeUnmount(() => {
  animation?.destroy();
  animation = null;
});
</script>

<template>
  <div ref="containerRef" class="playing-indicator" aria-hidden="true"></div>
</template>

<style scoped>
.playing-indicator {
  width: 16px;
  height: 16px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.playing-indicator :deep(svg) {
  width: 100%;
  height: 100%;
  display: block;
}
</style>
