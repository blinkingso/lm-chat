<template>
  <h1>Welcome to LM-CHAT</h1>
  <div class="app-location">
    <h2>Welcome to LM-Chat</h2>
    <div class="line"><span></span></div>
    <div class="location">
      <img alt="lm-chat" src="../assets/logo.png" />
    </div>
    <div class="main">
      <input
        type="text"
        v-model="uname"
        placeholder="Phone, E-mail or Username"
      />
      <input type="password" v-model="passwd" />
      <div class="submit">
        <input id="submit" type="button" value="Sign in" @click="signIn" />
      </div>
    </div>
  </div>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";
import { message } from "@tauri-apps/api/dialog";
import { appWindow, WebviewWindow } from "@tauri-apps/api/window";

export default {
  name: "UserLogin",
  data() {
    return {
      uname: "yaphets",
      passwd: "yaphets",
      count: 100,
    };
  },
  methods: {
    signIn() {
      invoke("sign_in", { username: this.uname, password: this.passwd })
        .then((user) => {
          appWindow.close().then();
          // create a HomePage window
          const webview = new WebviewWindow("homepage", {
            url: "/",
            visible: true,
            theme: "dark",
            center: true,
            title: "LmChat",
          });
          webview.once("tauri://created", function () {
            console.log("webview window create successfully");
            console.log(user);
          });
          webview.once("tauri://error", function (e) {
            console.log(`failed to create the webview window for: ${e}`);
          });
          webview.show().then();
        })
        .catch((msg) => message(msg, { title: "Error", type: "error" }).then());
    },
  },
  mounted() {},
};
</script>

<style scoped>
@import "../css/style.css";
</style>
