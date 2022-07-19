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
            <input name="unique" type="radio" @click="menuChecked(menu, item.name)" />
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
    <component :is="which_to_show" :contact_detail="contact_detail"></component>
  </nav>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";
import FriendDetail from "@/views/contacts/FriendDetail.vue";
import EmptyTab from "@/views/contacts/EmptyTab.vue";
import SavedGroupDetail from "@/views/contacts/SavedGroupDetail.vue";
import OfficialAccountsDetail from "@/views/contacts/OfficialAccountsDetail.vue";
import NewFriendDetail from "@/views/contacts/NewFriendDetail.vue";
import { markRaw } from "vue";

const CONTACTS = "contacts";
const SAVED_GROUPS = "saved_groups";
const OFFICIAL_ACCOUNTS = "official_accounts";

export default {
  components: {
    FriendDetail,
    SavedGroupDetail,
    OfficialAccountsDetail,
    NewFriendDetail,
    EmptyTab
  },
  data() {
    return {
      list: [],
      newFriends: [],
      savedGroups: [],
      officialAccounts: [],
      contacts: [],
      which_to_show: markRaw(EmptyTab),
      contact_detail: {}
    };
  },
  methods: {
    menuChecked(data, tab_name) {
      this.contact_detail = data;
      if (tab_name == CONTACTS) {
        this.which_to_show = markRaw(FriendDetail);
      } else if (tab_name == SAVED_GROUPS) {
        this.which_to_show = markRaw(SavedGroupDetail);
      } else if (tab_name == OFFICIAL_ACCOUNTS) {
        this.which_to_show = markRaw(OfficialAccountsDetail);
      } else {
        this.which_to_show = markRaw(NewFriendDetail);
      }
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
