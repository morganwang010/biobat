import {createRouter, createWebHistory} from "vue-router"
import BaDetail from '../views/BaDetail.vue'
import Home from '../views/Home.vue'
import BaList from '../components/BaList.vue'
import ComList from '../components/ComList.vue'
import { Header } from "ant-design-vue/es/layout/layout"
// import Bainfo from '../views/Bainfo.vue'

const router = createRouter({
    history: createWebHistory(),
    routes: [
    {
        path: "/",
        component: Home

    },
    {
        path: "/badetail/:id",
        component: BaDetail
        
    },
    {
        path: "/bacteria",
        component: BaList
    },
    {
        path: "/com/:id",
        component: BaDetail
        
    },
    {
        path: "/com",
        component: ComList
    },
    {
        path: "/ele/:id",
        component: BaDetail
        
    },
    {
        path: "/ele",
        component: BaList
    },
]
})

export default router