import { createApp } from 'vue'
import App from './App.vue'
import {createRouter, createWebHistory} from 'vue-router'
import UserLogin from './components/UserLogin'

import HomePage from '@/views/HomePage.vue'
import ChatRecords from '@/views/ChatRecords.vue'
import FriendList from '@/views/FriendList.vue'
import MyFavorites from '@/views/MyFavorites.vue'
import HistoryFiles from '@/views/HistoryFiles.vue'
import FriendCircle from '@/views/FriendCircle.vue'

const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: '/', 
            name: 'Home',
            component: HomePage
        },
        {
            path: '/login',
            name: "UserLogin",
            component: UserLogin
        },
        {
            path: '/records', 
            name: 'ChatRecords',
            component: ChatRecords
        },
        {
            path: '/friends',
            name: "FriendList",
            component: FriendList
        },
        {
            path: '/files',
            name: "HistoryFiles",
            component: HistoryFiles
        },
        {
            path: '/favorites',
            name: "MyFavorites",
            component: MyFavorites
        },
        {
            path: '/factionalism',
            name: "FriendCircle",
            component: FriendCircle
        }
        ]
    })

createApp(App).use(router).mount('#app')
