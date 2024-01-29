- 未添加数据库时，先按照这个文档安装   https://tauri.app/v1/guides/getting-started/setup/vite  使用pnpm
- 然后添加对sqlite安装依赖，包括引入cargo.toml中的依赖，创建models.rs及schema.rs和db.rs,本来想自动生成schema的，但是太麻烦

前端：
使用axios来获取数据,fetch使用有问题,未解决
后端简化使用,先用node http-server来启动,直接读取sqlite3数据库
前端使用table,异步获取后,再渲染table中的数据

app.vue里需要使用router-view 组件
main.js里需要引入router模块
router.js里需要配置route规则

pnpm create vite@latest
pnpm i
pnpm run dev
pnpm i --save ant-design-vue
