<template>
  <nav class="container">
    <ul>
      <li
        class="dropdown"
        :id="item.name"
        v-for="item in list"
        :key="item.name"
        @click="change(item.name)"
      >
        <input type="checkbox" />
        <span href="#" data-toggle="dropdown">
          <span v-if="item.active">
            <img src="../assets/unfold.png" />
          </span>
          <span v-else>
            <img src="../assets/enter.png" />
          </span>
          {{ item.show_name }}
          <span v-if="item.name == 'new_friends'"></span>
          <span v-else class="item-count">{{ item.count }}</span>
        </span>
        <ul class="dropdown-menu" :id="'item-tab-' + item.name">
          <li v-for="menu in getTabSubListByName(item.name)" :key="menu">
            <span class="line"></span>
            <span class="dropdown-menu-span">
              <img :src="menu.avatar" />
              <span>{{ menu.nick_name }}</span>
            </span>
          </li>
        </ul>
      </li>
    </ul>
  </nav>
</template>

<script>
// import { invoke } from "@tauri-apps/api/tauri";
export default {
  data() {
    return {
      // list: [],
      list: [
        {
          active: false,
          count: 0,
          id: 0,
          name: "new_friends",
          show_name: "New Friends",
        },
        {
          active: false,
          count: 0,
          id: 0,
          name: "saved_groups",
          show_name: "Saved Groups",
        },
        {
          active: false,
          count: 0,
          id: 0,
          name: "official_accounts",
          show_name: "Official Accounts",
        },
        {
          active: false,
          count: 0,
          id: 0,
          name: "contacts",
          show_name: "Contacts",
        },
      ],
      newFriends: [
        {
          avatar: require("/public/images/avatar.jpeg"),
          name: "apple",
          nick_name: "Apple",
          type: 0,
        },
      ],
      savedGroups: [
        {
          avatar: require("../static/images/avatar.jpeg"),
          name: "groups",
          nick_name: "Saved Groups",
          type: 1,
        },
      ],
      officialAccounts: [
        {
          avatar: require("../static/images/avatar.jpeg"),
          name: "official",
          nick_name: "Official Accounts",
          type: 2,
        },
      ],
      contacts: [
        {
          avatar: require("../assets/avatar.jpeg"),
          name: "lm",
          nick_name: "Lm",
          type: 3,
        },
        {
          avatar: require("../static/images/avatar.jpeg"),
          name: "sdy",
          nick_name: "Sdy",
          type: 3,
        },
        {
          avatar: require("../static/images/avatar.jpeg"),
          name: "gm",
          nick_name: "Gm",
          type: 3,
        },
        {
          avatar: require("../static/images/avatar.jpeg"),
          name: "lily",
          nick_name: "Lily",
          type: 3,
        },
      ],
    };
  },
  methods: {
    isTabActive(name) {
      this.list.forEach((li) => {
        if (li.name == name) {
          if (li.active) {
            li.active = false;
            return true;
          } else {
            li.active = true;
            return false;
          }
        }
        return false;
      });
    },
    getTabSubListByName(name) {
      if (name == "contacts") {
        return this.contacts;
      } else if (name == "saved_groups") {
        return this.savedGroups;
      } else if (name == "official_accounts") {
        return this.officialAccounts;
      } else {
        return this.newFriends;
      }
    },
    change(name) {
      // invoke("show_friend_list", { name: name })
      //   .then((list) => {
      //     console.log(list);
      //     const tab = document.getElementById(name);
      //     const item = document.getElementById("item-" + name);
      //     if (item != null && item != undefined) {
      //       tab.removeChild(item);
      //     }
      //     const tabRootUl = document.createElement("ul");
      //     tabRootUl.id = "item-" + name;
      //     tabRootUl.classList.add("item-friend");
      //     tab.appendChild(tabRootUl);
      //     if (this.isTabActive(name)) {
      //       tabRootUl.classList.remove("inactive");
      //       tabRootUl.classList.add("active");
      //     } else {
      //       tabRootUl.classList.remove("active");
      //       tabRootUl.classList.add("inactive");
      //     }
      //     list.forEach((friend) => {
      //       console.log(friend);
      //       const tabLeafLi = document.createElement("li");
      //       tabLeafLi.innerText = friend.nick_name;
      //       tabRootUl.appendChild(tabLeafLi);
      //     });
      //   })
      //   .catch((e) => {
      //     console.log(e);
      //   });
      this.isTabActive(name);
    },
  },
  mounted() {
    // init tab list.
    // invoke("show_friend_tabs").then((list) => {
    //   console.log(list);
    //   this.list = list;
    // });
  },
};
</script>

<style scoped lang="scss">
@import "../css/friendlist.scss";
</style>
