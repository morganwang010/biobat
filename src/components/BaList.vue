<script setup lang="ts">
import { ref } from 'vue'
import axios from "axios"
  const value = ref<string>('');
  const onSearch = (searchValue: string) => {
    console.log('use value', searchValue);
    console.log('or use this.value', value.value);
  };
    const columns = [
    {
      title: '统一菌株编号',
      dataIndex: 'number',
    },
    {
      title: '原始代号',
      className: 'column-money',
      dataIndex: 'code',
    },
    {
      title: '菌株英文名称',
      dataIndex: 'nameen',
    },
    {
      title: '菌株中文名称',
      dataIndex: 'namecn',
    },
  ];
    const data = ref([])
    axios.get("http://localhost:1105/api/ba").then(res => {
        console.log(res.data.array)
        data.value = (res.data.array)
      });
</script>
<template>
<div class="content">
  <div class="title" >
<!-- 
    <a-breadcrumb>
    <template #separator><span style="color: red">></span></template>
    <a-breadcrumb-item href="/"> Home</a-breadcrumb-item>
    <a-breadcrumb-item   href="/aaa">统一菌株</a-breadcrumb-item>
  </a-breadcrumb> -->
    <span class="nav"> <a href="/">首页&nbsp;</a> > <a href="/bacteria"> &nbsp;数据库-2018YFA0901903-菌种库</a></span>
  </div>
  <div class="searchbox">
    <a-space direction="vertical" style="width: 80%;">
    <a-input-search
      v-model:value="value"
      placeholder="请输入统一菌株编号"
      enter-button="Search"
      size="large"
      @search="onSearch"
    />
  </a-space>
  </div>
</div>
  <a-table :columns="columns" :data-source="data" bordered class="ant-table-cell"  scroll={ y: true, x: true }>
      <template #bodyCell="{ column, record }">
        <template v-if="column.dataIndex === 'number'">
 <a :href="'/badetail/'+record.id">{{ record.number }}</a>
        </template>
      </template>
    </a-table>

</template>
<style scoped>
.nav{
  display: flex;
  margin-left: 0;
  position: absolute;
  margin-top: 20px;
}
.title{
  height: 60px;
}
.read-the-docs {
  color: #888;
}
th.column-money,
  td.column-money {
    text-align: right !important;
  }
  .ant-table-cell {
  white-space: nowrap
}
.content{
  margin: auto;
  /* margin-bottom: 20px; */
  width:100%;
}
.searchbox{
  margin: auto;
  margin-bottom: 20px;
  width:100%;
}

</style>
