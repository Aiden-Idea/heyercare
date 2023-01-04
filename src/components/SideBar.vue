<script setup lang="ts">
import { ref } from 'vue';


export interface Menu {
  label: string,
  title: string
}

const activeMenu = ref("统计数据")

const props = defineProps<{
  menu: Menu[]
}>()

const emit = defineEmits<{
  (e: "handelMenuItem"): void
}>()

const handleMenu = (menuItem: Menu) => {
  activeMenu.value = menuItem.title
  emit("handelMenuItem")
}


</script>
<template>
  <div class="side_bar">
    <div class="avatar">
      <svg-icon icon-class="touxiang"></svg-icon>
    </div>
    <div class="user_name">
      <p>张</p>
      <p>三</p>
    </div>
    <div class="menu_list">
      <div v-for="item in props.menu" @click="handleMenu(item)" class="menu_item cs" :class="{'menu_active': item.title === activeMenu}">
        {{item.title}}
      </div>
    </div>
  </div>
</template>
<style lang="scss" scoped>
.menu_active {
  color: #18B364 !important;
  background-image: linear-gradient( to right, rgba(0, 140, 70, 0.2), rgba(0, 162, 100, 0));
}
.side_bar {
  width: 200px;
  height: calc(100vh - 50px);
  box-sizing: border-box;
  box-shadow: 2px 0px 6px 0px rgba(0,0,0,0.1);
  overflow: hidden;
  .avatar {
    text-align: center;
    font-size: 40px;
    margin-top: 40px;
  }
  .user_name {
    margin-top: 20px;
    text-align: center;
  }
  .menu_list {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-top: 40px;
    width: 100%;
    .menu_item {
      height: 50px;
      line-height: 50px;
      width: 100%;
      text-align: center;
    }
  }
}
</style>