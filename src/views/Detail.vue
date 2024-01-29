<script setup lang="ts">
import { ref } from 'vue'
import { useRoute } from 'vue-router'
 
const { params } = useRoute()
const id = params.id
console.log(id)

  const value = ref<string>('');
  
  const onSearch = (searchValue: string) => {
    console.log('use value', searchValue);
    console.log('or use this.value', value.value);
  };
 
  import axios from "axios"
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
    {
      title: '菌株来源描述',
      dataIndex: 'source',
    },
    {
      title: '采集地点',
      dataIndex: 'place',
    },
    {
      title: '保存单位',
      dataIndex: 'org',
    },
    {
      title: '保存人',
      dataIndex: 'research',
    },
    {
      title: '登记日期',
      dataIndex: 'sdate',
    },
  ];
  const data = ref([])
axios.get("http://localhost:1105/api/ba/"+id+"").then(res => {
        // const data = res
        console.log("fffffffffff")
        console.log(res.data.array)
        data.value = (res.data.array)
      });
 

</script>

<template>
<div class="searchbox">
  <div class="header">
    <span > <img style="width: 50px;height: 50px;margin-bottom: -15px;" src="/imgs/logo.jpg" ></span><span>国家重点研发“合成生物学”专项：微生物天然产物的智能创建与改良</span>
  </div>
  <div class="title" >
    <span class="nav"> 首页 > 数据库-2018YFA0901903-菌种库</span>

  </div>

  <a-space direction="vertical" style="width: 100%;">
    
    <a-input-search
      v-model:value="value"
      placeholder="input search text"
      enter-button="Search"
      size="large"
      @search="onSearch"
    />

  </a-space>
</div>

  <a-table :columns="columns" :data-source="data" bordered class="ant-table-cell">
      <template >
        <!-- <template v-if="column.dataIndex === 'number'">
 <a :href="'/detail/'+record.number">{{ record.number }}</a>
        </template> -->
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
