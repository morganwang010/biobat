<script setup lang="ts">
import { ref } from 'vue'
import axios from "axios"


// const count = ref(0)
 
  const value = ref<string>('');
  
  const onSearch = (searchValue: string) => {
    console.log('use value', searchValue);
    console.log('or use this.value', value.value);
  };
 

    const columns = [
    {
      title: '编号',
      dataIndex: 'number',
    },
    {
      title: '化合物代号',
      className: 'column-money',
      dataIndex: 'code',
    },
    {
      title: '来源',
      dataIndex: 'nameen',
    },
    {
      title: '结构类型',
      dataIndex: 'namecn',
    },
    {
      title: '分子量',
      dataIndex: 'namecn',
    },
    {
      title: '分子式',
      dataIndex: 'namecn',
    },
    {
      title: '生物活性信息',
      dataIndex: 'namecn',
    },
    {
      title: '已知/新',
      dataIndex: 'namecn',
    },
    {
      title: '化合物鉴定',
      children: [
              {
                title: '1H',
                dataIndex: 'oneh',
                key: 'building',
              },
              {
                title: '13C',
                dataIndex: 'cc',
                
              },
              {
                title: 'HSQC',
                dataIndex: 'hsqc',
                
              },
              {
                title: 'HMBC',
                dataIndex: 'hmbc',
                
              },
              {
                title: 'COSY ',
                dataIndex: 'cosy',
                
              },
              {
                title: 'HR-MS',
                dataIndex: 'hrms',
                
              },
              {
                title: 'IR',
                dataIndex: 'ir',
                
              },
              {
                title: 'UV',
                dataIndex: 'uv',
                
              },
              {
                title: 'X-ray',
                dataIndex: 'xray',
                
              },
            ],
    },
    {
      title: '备注',
      dataIndex: 'note',
    },
    {
      title: '所属课题组负责人及所在单位',
      dataIndex: 'charger',
    },
    {
      title: '入库日期',
      dataIndex: 'sdate',
    },


    // {
    //   title: '菌株来源描述',
    //   dataIndex: 'source',
    // },
    // {
    //   title: '采集地点',
    //   dataIndex: 'place',
    // },
    // {
    //   title: '保存单位',
    //   dataIndex: 'org',
    // },
    // {
    //   title: '保存人',
    //   dataIndex: 'research',
    // },
    // {
    //   title: '登记日期',
    //   dataIndex: 'sdate',
    // },
  ];
    const data = ref([])
    axios.get("http://localhost:1105/api/ba").then(res => {
        // const data = res
        console.log("fffffffffff")
        console.log(res.data.array)
        data.value = (res.data.array)
      });
 

</script>

<template>
<div class="searchbox"> 
  <div class="title" >
    <span class="nav"> <a href="/">首页 &nbsp;</a> > 数据库-2018YFA0901903-化合物库</span>

  </div>
  <a-space direction="vertical" style="width: 80%;">
    
    <a-input-search
      v-model:value="value"
      placeholder="请输入化合物编号"
      enter-button="Search"
      size="large"
      @search="onSearch"
    />

  </a-space>
</div>

  <a-table :columns="columns" :data-source="data" bordered class="ant-table-cell"  :scroll="{ x: true}">
      <template #bodyCell="{ column, record }">
        <template v-if="column.dataIndex === 'number'">
 <a :href="'/badetail/'+record.id">{{ record.number }}</a>
        </template>
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
  margin: auto;
  margin-bottom: 20px;
  width:100%;
}

</style>
