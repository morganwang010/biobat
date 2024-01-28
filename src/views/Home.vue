<script setup lang="ts">
// import { ref } from 'vue'
import { useRoute } from 'vue-router'
 
const { params } = useRoute()
const id = params.id
console.log(id)
// const value = ref<string>('');
  
  // const onSearch = (searchValue: string) => {
  //   console.log('use value', searchValue);
  //   console.log('or use this.value', value.value);
  // };
const data = [
{
    "link":"/bacteria",
    "name":"2018YFA0901903-菌种库",
    "total": "1200"
},
{
    "link":"/block",
    "name":"2018YFA0901903-元件库",
    "total": "1320"
},
{
    "link":"/bacteria",
    "name":"2018YFA0901903-化合物库",
    "total": "68"
}
      
 
     
  ];
 

</script>

<template>
<div class="searchbox">
  <div class="header">
    <span > <img style="width: 50px;height: 50px;margin-bottom: -15px;" src="/imgs/logo.jpg" ></span><span>国家重点研发“合成生物学”专项：微生物天然产物的智能创建与改良</span>
  </div>
  <div style="background-color: #ececec; padding: 20px">

        <a-card title="首席科学家：林双君 教授" :bordered="false" style="width: 100%; text-align: left">
          <p>    参加单位：上海交通大学、中国医学科学院药物研究所、中科院上海有机化学研究所、中科院微生物研究所、中科院南海海洋研究所、中科院天津工业生物技术研究所
</p>
        </a-card>
   <div class="content">
    <a-list size="large" bordered :data-source="data" style="margin-top: 20px;  background: #fff;">
    <template  #renderItem="{ item }">
        <a-list-item><div class="item"><span class="db"><a :href="item.link"> {{ item.name }}</a></span> <span class="total">{{ item.total }}</span></div></a-list-item>

    </template>
    <template #header>
      <div class="title"><h3>数据汇交系统：
</h3></div>
    </template>

  </a-list>

   </div>
  </div>
  </div>
</template>

<style scoped>
.header{
    font-size: 30px;
    font-weight: bold;
    text-align: left;
    margin: 10px auto;
    padding-bottom: 20px;
}
.content{
   
    box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.03), 0 1px 6px -1px rgba(0, 0, 0, 0.02), 0 2px 4px 0 rgba(0, 0, 0, 0.02);
}
.ant-list-header{
    text-align: left;
}
.total{
    float: right;
    margin-right: 100px;
}
.db{
    float:left;
    margin-left: 50px;
}
.item{
    left: 0%;
    width: 100%;
}
.nav{
  display: flex;
  margin-left: 0;
  position: absolute;
  margin-top: 20px;
}
.title{
  height: 30px;
  text-align: left;
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
