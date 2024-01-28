import { createApp } from 'vue'
import './style.css'
import Antd from "ant-design-vue"
// import 'ant-design-vue/dist/antd.css'
import App from './App.vue'
import router from './router/router'
// router.afterEach((to, from, next) => {
//     document.querySelector("body").setAttribute("style", "overflow: auto !important;")
//    });
createApp(App)
.use(Antd)
.use(router)
.mount('#app')
