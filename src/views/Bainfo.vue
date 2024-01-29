<template>
    <a-table :columns="columns" :data-source="data" bordered class="ant-table-cell">
      <template #bodyCell="{ column, text }">
        <template v-if="column.dataIndex === 'name'">
          <a>{{ text }}</a>
        </template>
      </template>
      <template #title>Header</template>
      <template #footer>Footer</template>
    </a-table>
  </template>
  <script lang="ts" setup >
  import {ref} from "vue"
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
    axios.get("http://localhost:1105").then(res => {
        // const data = res
        console.log("fffffffffff")
        console.log(res.data)
        data.value = (res.data)
      });
 

  </script>
  <style scoped>
  th.column-money,
  td.column-money {
    text-align: right !important;
  }
  .ant-table-cell {
  white-space: nowrap
}

  </style>