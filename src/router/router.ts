import {createRouter, createWebHistory} from "vue-router"
import Detail from '../views/Detail.vue'
import Home from '../views/Home.vue'
import HelloWorld from '../components/HelloWorld.vue'
// import Bainfo from '../views/Bainfo.vue'

const router = createRouter({
    history: createWebHistory(),
    routes: [
    {
        path: "/",
        component: Home

    },
    {
        path: "/detail/:id",
        component: Detail
        
    },
    {
        path: "/bacteria",
        component: HelloWorld
    },
]
})

export default router