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
      dataIndex: 'source',
    },
    {
      title: '结构类型',
      dataIndex: 'structure',
    },
    {
      title: '分子量',
      dataIndex: 'mol',
    },
    {
      title: '分子式',
      dataIndex: 'molfomula',
    },
    {
      title: '化合物量',
      dataIndex: 'comno',
    },
    {
      title: '生物活性信息',
      dataIndex: 'info',
    },
    {
      title: '已知/新',
      dataIndex: 'new',
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
  ];
  const data = ref([])
axios.get("http://localhost:1105/api/com/"+id+"").then(res => {
        // const data = res
        console.log("fffffffffff")
        console.log(res.data.array)
        data.value = (res.data.array)
      });
 

</script>

<template>
<div class="searchbox">
  <div class="title" >
    <span class="nav"> <a href="/">首页&nbsp;</a> > <a href="/com"> &nbsp;数据库-2018YFA0901903-化合物库</a></span>

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

  <a-table :columns="columns" :data-source="data" bordered class="ant-table-cell" :scroll="{x:true }">
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
