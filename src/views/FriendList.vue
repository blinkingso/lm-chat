<template>
  <div class="nav-friends">
    <ul>
      <li v-bind:id="item.name" v-for="item in list" v-bind:key="item.name">
        <span v-if="item.active">
          <img class="item-img" src="../assets/unfold.png" />
        </span>
        <span v-else>
          <img class="item-img" src="../assets/enter.png" />
        </span>
        <span>{{ item.show_name }}</span>
        <span v-if="item.name == 'new_friends'"></span>
        <span v-else class="item-count">{{ item.count }}</span>
      </li>
    </ul>
  </div>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";
export default {
  data() {
    return {
      list: [],
    };
  },
  methods: {
    change(name) {
      invoke("show_friend_list", { name: name })
        .then((list) => {
          console.log(list);
          list.forEach((element) => {
            console.log(element);
          });
        })
        .catch((e) => {
          console.log(e);
        });
    },
  },
  mounted() {
    // init tab list.
    invoke("show_friend_tabs").then((list) => {
      this.list = list;
    });
  },
};
</script>

<style scoped>
@import "../css/friendlist.css";
</style>
