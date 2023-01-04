<script lang="ts">
export default {
  name: "Archives"
};
</script>
<script setup lang="ts">
import SideBar from '@/components/SideBar.vue';
import { ref } from 'vue';
import type { Menu } from '@/components/SideBar.vue';

const menu = ref<Menu[]>([
  {
    label: "",
    title: "设备设置"
  },
  {
    label: "",
    title: "统计数据"
  },
  {
    label: "",
    title: "周期数据"
  },
  {
    label: "",
    title: "详细数据"
  }
])

</script>
<template>
  <SideBar :menu="menu" />
</template>
<style>

</style>