<script setup lang="ts">
import { ref } from 'vue'
import { useRoute } from 'vue-router'
import axios from "axios"

const { params } = useRoute()
const id = params.id
console.log(id)

  const value = ref<string>('');
  
  const onSearch = (searchValue: string) => {
    console.log('use value', searchValue);
    console.log('or use this.value', value.value);
  };
 

  const columns = [
    {
      title: '基因元件编号',
      dataIndex: 'number',
    },
    {
      title: '基因元件名称',
      className: 'column-money',
      dataIndex: 'name',
    },
    {
      title: '元件类别',
      dataIndex: 'catlog',
    },
    {
      title: '大类',
      dataIndex: 'class',
    },
    {
      title: '基因来源',
      dataIndex: 'source',
    },
    {
      title: '来源描述',
      dataIndex: 'describe',
    },
    {
      title: '详细信息',
      dataIndex: 'detail',
    },
    {
      title: '大小（bp）',
      dataIndex: 'size',
    },
    {
      title: ' 登录号（如有）',
      dataIndex: 'regno',
    },
    {
      title: '基因元件提供单位',
      dataIndex: 'researcher',
    },
    {
      title: '序列信息/质粒图谱',
      dataIndex: 'seqinfo',
    },
    {
      title: '登记日期',
      dataIndex: 'sdate',
    },
  ];
  const data = ref([])
axios.get("http://localhost:1105/api/ele/"+id+"").then(res => {
        // const data = res
        console.log("fffffffffff")
        console.log(res.data.array)
        data.value = (res.data.array)
      });
 

</script>

<template>
<div class="searchbox">

  <div class="title" >
    <span class="nav"> <a href="/">首页&nbsp;</a> > <a href="/ele"> &nbsp;数据库-2018YFA0901903-元件库</a></span>

  </div>

  <a-space direction="vertical" style="width: 80%;">
    
    <a-input-search
      v-model:value="value"
      placeholder="input search text"
      enter-button="Search"
      size="large"
      @search="onSearch"
    />

  </a-space>
</div>

  <a-table :columns="columns" :data-source="data" bordered class="ant-table-cell" :scroll="{x:true}">
      <template >
      </template>
      <!-- <template #title>Header</template>
      <template #footer>Footer</template> -->
    </a-table>

</template>

<style scoped>
.header{
    font-size: 30px;
    font-weight: bold;
    text-align: left;
    margin: 10px auto;
    padding-bottom: 20px;
}
.nav{
  display: flex;
  margin-left: 0;
  position: absolute;
  margin-top: 20px;
}
.title{
  height: 80px;
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
.searchbox{
  margin-bottom: 20px;
  width: 100%;
}

</style>
