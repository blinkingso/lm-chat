<template>
  <div class="nav">
    <div class="nav-views">
      <div class="user-profile">
        <img src="../assets/avatar.jpeg" />
      </div>

      <ul class = "nav-views-list">
        <li v-for="item in nav" :key="item.id" :class="{ active: item.id == current, inactive: item.id != current }">
          <a v-bind:href="item.path" @click="change(item.id)">{{ item.text }}</a>
        </li>
      </ul>

      <div class="nav-menu">
        <div class="nav-menu-settings"><a href="#/settings">Settings</a></div>
      </div>
    </div>
    <div class="nav-content">
      <component :is="currentView" />
    </div>
  </div>
</template>

<script>
import ChatRecords from "@/views/ChatRecords.vue";
import FriendList from "@/views/FriendList.vue";
import MyFavorites from "@/views/MyFavorites.vue";
import HistoryFiles from "@/views/HistoryFiles.vue";
import FriendCircle from "@/views/FriendCircle.vue";
const routes = {
  "/records": ChatRecords,
  "/friends": FriendList,
  "/favorites": MyFavorites,
  "/files": HistoryFiles,
  "/factionalism": FriendCircle,
};
export default {
  data() {
    return {
      current: 0,
      currentPath: window.location.hash,
      nav: [
        { id: 0, path: "#/records", text: "Records" },
        { id: 1, path: "#/friends", text: "Friends" },
        { id: 2, path: "#/favorites", text: "Favorites" },
        { id: 3, path: "#/files", text: "Files" },
        { id: 4, path: "#/factionalism", text: "Factionalism" },
      ],
    };
  },
  methods: {
    change(index) {
      this.current = index;
    },
  },
  computed: {
    currentView() {
      return routes[this.currentPath.slice(1) || "/records"];
    },
  },
  mounted() {
    window.addEventListener("hashchange", () => {
      this.currentPath = window.location.hash;
    });
  },
};
</script>

<style scoped>
@import "../css/style.css";
@import "../css/homepage.css";
</style>
