<script setup lang="ts">
import { onMounted, reactive, ref } from 'vue';

const tableHeight = ref()
const searchCont = ref('')
const pageInfo = reactive({
  pageNum: 1,
  pageSize: 10,
  pages: 0,
  total: 1
})
const patientDataList = ref([
  {
    firstName: 'xxx',
    lastName: 'yyy',
    treatingDoctorId: 'xxx',
    hostModel: 'xxx',
    hostSerialNumber: 'xxx',
    createTime: 'xxx',
  },
  {
    firstName: 'xxx',
    lastName: 'yyy',
    treatingDoctorId: 'xxx',
    hostModel: 'xxx',
    hostSerialNumber: 'xxx',
    createTime: 'xxx',
  },
  {
    firstName: 'xxx',
    lastName: 'yyy',
    treatingDoctorId: 'xxx',
    hostModel: 'xxx',
    hostSerialNumber: 'xxx',
    createTime: 'xxx',
  },
  {
    firstName: 'xxx',
    lastName: 'yyy',
    treatingDoctorId: 'xxx',
    hostModel: 'xxx',
    hostSerialNumber: 'xxx',
    createTime: 'xxx',
  },
  {
    firstName: 'xxx',
    lastName: 'yyy',
    treatingDoctorId: 'xxx',
    hostModel: 'xxx',
    hostSerialNumber: 'xxx',
    createTime: 'xxx',
  },
  {
    firstName: 'xxx',
    lastName: 'yyy',
    treatingDoctorId: 'xxx',
    hostModel: 'xxx',
    hostSerialNumber: 'xxx',
    createTime: 'xxx',
  },
  {
    firstName: 'xxx',
    lastName: 'yyy',
    treatingDoctorId: 'xxx',
    hostModel: 'xxx',
    hostSerialNumber: 'xxx',
    createTime: 'xxx',
  },
  {
    firstName: 'xxx',
    lastName: 'yyy',
    treatingDoctorId: 'xxx',
    hostModel: 'xxx',
    hostSerialNumber: 'xxx',
    createTime: 'xxx',
  },{
    firstName: 'xxx',
    lastName: 'yyy',
    treatingDoctorId: 'xxx',
    hostModel: 'xxx',
    hostSerialNumber: 'xxx',
    createTime: 'xxx',
  },{
    firstName: 'xxx',
    lastName: 'yyy',
    treatingDoctorId: 'xxx',
    hostModel: 'xxx',
    hostSerialNumber: 'xxx',
    createTime: 'xxx',
  },{
    firstName: 'xxx',
    lastName: 'yyy',
    treatingDoctorId: 'xxx',
    hostModel: 'xxx',
    hostSerialNumber: 'xxx',
    createTime: 'xxx',
  },{
    firstName: 'xxx',
    lastName: 'yyy',
    treatingDoctorId: 'xxx',
    hostModel: 'xxx',
    hostSerialNumber: 'xxx',
    createTime: 'xxx',
  },{
    firstName: 'xxx',
    lastName: 'yyy',
    treatingDoctorId: 'xxx',
    hostModel: 'xxx',
    hostSerialNumber: 'xxx',
    createTime: 'xxx',
  }
])
const isEn = ref(false)
let timer: NodeJS.Timeout | null = null // ??????
const tableRef = ref<HTMLElement | null>(null)

onMounted(() => {
  // table ????????????
  tableHeight.value = tableRef.value!.clientHeight
  window.onresize = () => {
    if(timer) {
      return
    }
    timer = setTimeout(() => {
      tableHeight.value = tableRef.value!.clientHeight
      timer = null
    }, 500);
  }
})


const rowClick = () => {

}

const del = (index: any, row: any) => {

}
const handleEdit = (a: any,b: any,c: any,d: any) => {

}

const handleIconClick = () => {

}
const handleCurrentChange = () => {

}
const handleSizeChange = () => {

}

</script>
<template>
  <div class="box patientlist-con">
    <div class="table" ref="tableRef">
      <el-table ref="singleTableRef" highlight-current-row @row-click="rowClick"  :height="tableHeight" :data="patientDataList" style="width: 100%">
        <el-table-column width="90">
          <template #default="scope">
            <div style="display: flex; align-items: center">
              <span class="delete" style="margin-left: 10px; color: #1f9156;"
                @click="del(scope.$index, scope.row)">??????</span>
            </div>
          </template>
        </el-table-column>
        <el-table-column align="center" type="index" label="??????" width="100">
          <template #default="scope">
           <!-- <span v-text="((pageInfo.pageNum - 1)  * 10) + (scope.$index + 1)"></span> -->
          </template>
        </el-table-column>
        <el-table-column label="??????"  min-width="70">
          <template #default="scope">
            <span v-if="isEn">{{scope.row.firstName}}  {{scope.row.lastName}}</span>
            <span v-else>{{scope.row.lastName}}{{scope.row.firstName}}</span>
          </template>
        </el-table-column>
        <el-table-column align="center" prop="treatingDoctorId" label="??????ID"  />
        <el-table-column prop="hostModel" label="????????????" />
        <el-table-column prop="hostSerialNumber" label="???????????????" />
        <el-table-column label="????????????" >
          <template #default="scope">
            <span>{{scope.row.createTime.slice(0,10).split('-')[1] + '-' + scope.row.createTime.slice(0,10).split('-')[2] + '-' + scope.row.createTime.slice(0,10).split('-')[0] + scope.row.createTime.slice(10,)}}</span>
          </template>
        </el-table-column>
        <el-table-column align="center" label="????????????" width="120">
          <template #default="scope">
            <el-link type="success" @click="handleEdit($event, scope.$index, scope.row, 'Archive')">
              ??????
            </el-link>
          </template>
        </el-table-column>
        <el-table-column align="center" label="????????????" width="120">
          <template #default="scope">
            <el-link type="success" @click="handleEdit($event, scope.$index, scope.row, 'Static')">
              ??????
            </el-link>
          </template>
        </el-table-column>
        <el-table-column align="center" label="??????" width="90">
          <template #default="scope">
            <el-link type="success" @click="handleEdit($event, scope.$index, scope.row, 'Report')">
              ??????
            </el-link>
          </template>
        </el-table-column>
        <template v-slot:empty>
          <span class="empty">??????????????????</span>
        </template>
      </el-table>
    </div>
    <div class="table-bottom flex-ai-center">
      <div class="flex-ai-center search">
        <input v-model="searchCont" @input="handleIconClick" placeholder="?????????" />
        <div class="input__icon">
          <svg-icon icon-class="sousuo"></svg-icon>
        </div>
      </div>
      <div class="pagination">
        <el-pagination 
          layout="prev, pager, next, sizes, jumper" 
          :total=pageInfo.total 
          @current-change="handleCurrentChange" 
          :page-sizes="[10, 20, 30, 50]"
          @size-change="handleSizeChange"
        />
      </div>
    </div>
  </div>
</template>
<style lang="scss" scoped>
.el-table__row:hover .delete {
  display: block;
}
.delete {
  cursor: pointer;
  display: none;
}
.patientlist-con {
  margin-top: 25px;
  
}
.table {
  height: calc(100vh - 150px);
}
.table-bottom {
  border-top: 1px solid rgb(211, 211, 211);
  height: 55px;
  justify-content: space-between;
  .search {
    height: 100%;
    box-sizing: border-box;
    position: relative;
    padding-left: 40px;
    width: 300px;
    >input {
      width: 100%;
      height: 70%;
      border-radius: 5px;
      padding-left: 35px;
      padding-right: 15px;
      background-color: #f9f9f9;
    }
    >.input__icon {
      position: absolute;
      left: 50px;
      color: #1a874f;
    }
  }
}
</style>