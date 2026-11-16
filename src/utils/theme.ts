import { readonly, ref } from "vue";

type Theme = "light" | "dark";
const storageKey = "simple-music-theme";
const theme = ref<Theme>("light");

function applyTheme() {
  document.documentElement.dataset.theme = theme.value;
  document.documentElement.classList.toggle("dark", theme.value === "dark");
  document.documentElement.style.colorScheme = theme.value;
}

export function initializeTheme() {
  try {
    theme.value = localStorage.getItem(storageKey) === "dark" ? "dark" : "light";
  } catch {
    theme.value = "light";
  }
  applyTheme();
}

export function useTheme() {
  function toggleTheme() {
    theme.value = theme.value === "dark" ? "light" : "dark";
    applyTheme();
    try {
      localStorage.setItem(storageKey, theme.value);
    } catch {
      // Switching still works when persistent storage is unavailable.
    }
  }
  return { theme: readonly(theme), toggleTheme };
}
