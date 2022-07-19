<template>
  <nav class="fl-container">
    <ul>
      <li class="dropdown" :id="item.name" v-for="item in list" :key="item.name">
        <input type="checkbox" @click="collapse(item.name, $event)" />
        <span class="dropdown-title" href="#" data-toggle="dropdown">
          <img src="../assets/enter.png" />
          <span>{{ item.show_name }}</span>
          <span class="item-count" v-if="item.name == 'new_friends'"></span>
          <span v-else class="item-count">{{ item.count }}</span>
        </span>
        <ul class="dropdown-menu">
          <li v-for="menu in getShowingList(item.name)" :key="menu">
            <span>{{ menu.first_letter }}</span>
            <span class="line"></span>
            <input name="unique" type="radio" @click="menuChecked(menu, $event)" />
            <span class="dropdown-menu-span">
              <img :src="menu.avatar" />
              <span>{{ menu.nick_name }}</span>
            </span>
          </li>
        </ul>
      </li>
    </ul>
  </nav>
  <nav id="friend-list-main">
    <FriendDetail />
  </nav>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";
import FriendDetail from "@/views/FriendDetail.vue";
const CONTACTS = "contacts";
const SAVED_GROUPS = "saved_groups";
const OFFICIAL_ACCOUNTS = "official_accounts";
export default {
  components: {
    FriendDetail
  },
  data() {
    return {
      list: [],
      newFriends: [],
      savedGroups: [],
      officialAccounts: [],
      contacts: [],
    };
  },
  methods: {
    menuChecked(data, e) {
      console.log(data);
      console.log(e);
      const main = document.getElementById('friend-list-main');
      main.innerHTML = FriendDetail;
    },
    getShowingList(name) {
      if (name == CONTACTS) {
        return this.contacts;
      } else if (name == SAVED_GROUPS) {
        return this.savedGroups;
      } else if (name == OFFICIAL_ACCOUNTS) {
        return this.officialAccounts;
      } else {
        return this.newFriends;
      }
    },
    collapse(name, event) {
      var target = event.currentTarget.nextElementSibling.firstElementChild;
      if (event.currentTarget.checked) {
        target.src = require('../assets/unfold.png');
      } else {
        target.src = require('../assets/enter.png');
      }
      invoke("show_friend_list", { name: name })
        .then((list) => {
          if (name == CONTACTS) {
            this.contacts = list;
          } else if (name == SAVED_GROUPS) {
            this.savedGroups = list;
          } else if (name == OFFICIAL_ACCOUNTS) {
            this.officialAccounts = list;
          } else {
            this.newFriends = list;
          }
        })
        .catch((e) => {
          console.log(e);
        });
    },
  },
  mounted() {
    invoke("show_friend_tabs").then((list) => {
      this.list = list;
    });
  },
};
</script>

<style scoped lang="scss">
@import "../css/friendlist.scss";
</style>
