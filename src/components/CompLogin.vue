<template>
  <div class="main">
    <input type="text" v-model="uname" placeholder="Phone, E-mail or Username" />
    <input type="password" v-model="passwd" />
    <div class="submit">
      <input id="submit" type="button" value="Sign in" @click="signIn" />
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri';

export default {
  name: 'CompLogin',
  data() {
    return {
      uname: '',
      passwd: '',
      count: 100
    }
  },
  methods: {
    signIn() {
      login(this.uname, this.passwd)
    }
  }
}

async function login(username, password) {
  const loginResult = await invoke('sign_in', { username: username, password: password })
  if (loginResult) {
    console.log('login success')
  } else {
    console.log('login failed')
  }
}

</script>

<style scoped>
@import '../css/style.css'
</style>