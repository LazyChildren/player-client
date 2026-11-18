import { createApp } from "vue";
import ElementPlus from "element-plus";
import "element-plus/dist/index.css";
import App from "./App.vue";
import TrayMenu from "./components/TrayMenu.vue";
import "element-plus/theme-chalk/dark/css-vars.css";
import "./assets/theme.css";
import { initializeTheme } from "./utils/theme";

initializeTheme();

createApp(window.location.hash === "#/tray" ? TrayMenu : App).use(ElementPlus).mount("#app");
