import {createRouter, createWebHistory} from "vue-router"
import BaDetail from '../views/BaDetail.vue'
import ComDetail from '../views/ComDetail.vue'
import EleDetail from '../views/EleDetail.vue'
import Home from '../views/Home.vue'
import BaList from '../components/BaList.vue'
import ComList from '../components/ComList.vue'
import EleList from '../components/EleList.vue'
// import { Header } from "ant-design-vue/es/layout/layout"
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
        component: ComDetail
        
    },
    {
        path: "/com",
        component: ComList
    },
    {
        path: "/ele/:id",
        component: EleDetail
        
    },
    {
        path: "/ele",
        component: EleList
    },
]
})

export default router