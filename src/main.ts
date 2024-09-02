import { createApp } from "vue";
import App from "./App.vue";
import router from "./router/index";

import "./style.css";

import "./demos/ipc";
// If you want use Node.js, the`nodeIntegration` needs to be enabled in the Main process.
// import './demos/node'

let app = createApp(App);
app.use(router);
app.mount("#app").$nextTick(() => {
  postMessage({ payload: "removeLoading" }, "*");
});
