<template>
  <h1>Welcome to LM-CHAT</h1>
  <div class="app-location">
    <h2>Welcome to LM-Chat</h2>
    <div class="line"><span></span></div>
    <div class="location">
      <img alt="lm-chat" src="./assets/logo.png">
    </div>
    <CompLogin />
  </div>
</template>

<script>
import CompLogin from './components/CompLogin.vue'

export default {
  name: 'App',
  components: {
    CompLogin
  }
}
</script>

<style>
@import './css/style.css'
</style>