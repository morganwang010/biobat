<script setup lang="ts">
import type { CSSProperties } from "vue";

const headerStyle: CSSProperties = {
  textAlign: "center",
  color: "#fff",
  minHeight: 120,
  paddingInline: 50,
  lineHeight: "80px",
  backgroundColor: "#0070C0",
  height:"120px",
};

const contentStyle: CSSProperties = {
  textAlign: "center",
  minHeight: 120,
  lineHeight: "10px",
  color: "#fff",
  backgroundColor: "#108ee9",
};

const footerStyle: CSSProperties = {
  textAlign: "center",
  color: "#fff",
  backgroundColor: "#7dbcea",
};
</script>

<template>
  <div class="app">
    <a-space direction="vertical" :style="{ width: '100%' }" :size="[0, 48]">
      <a-layout>
        <a-layout-header :style="headerStyle">
          <div class="header">
            <span>
              <img
                style="width: 50px; height: 50px; margin-bottom: -15px"
                src="/imgs/logo.jpg" /></span
            ><span
              >国家重点研发“合成生物学”专项：微生物天然产物的智能创建与改良</span
            >
          </div>
        </a-layout-header>
        <a-layout-content :style="contentStyle">
          <router-view
        /></a-layout-content>
        <!-- <a-layout-footer :style="footerStyle">Footer</a-layout-footer> -->
      </a-layout>
    </a-space>
  </div>
</template>

<style scoped>
.header{
  display: block;
    font-size: 30px;
    font-weight: bold;
    text-align: left;
    margin: 10px auto;
    padding-bottom: 20px;
}
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}
.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}
.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
.app {
  top: 10px;
  height: auto;
}
</style>
