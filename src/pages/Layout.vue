<script setup lang="ts">
import MenuBar from '@/components/MenuBar.vue'
import setKeepAlive, { kEEP_ALIVE_MAP } from '@/utils/useCache'
import { onBeforeRouteLeave, useRoute } from 'vue-router';

const route = useRoute()

onBeforeRouteLeave((to, from, next) => {
  // if(to.name !== "Data") {
  //   setKeepAlive("Data", new Date().getTime())
  // }
  console.log('aa');
  
  console.log(to, from);
  
  next()
})

</script>
<template>
  <MenuBar />
  <!-- TODO 过渡动画 -->
  <!-- 利用缓存的时间戳key 保证页面保鲜 -->
  <router-view v-slot="{ Component }">

  </router-view>
</template>
<style>
/*页面切换动画*/
/*进入过渡的结束状态，元素被插入时就生效，在过渡过程完成后移除*/
.fade-enter-active {
    transition: opacity .5s;
}
/*进入过渡的开始状态，元素被插入时生效，只应用一帧后立刻删除*/
.fade-enter-from {
    opacity: 0;
}
/*离开过渡的开始状态，元素被删除时触发，只应用一帧后立刻删除*/
.fade-leave-to {
    opacity: 0;
}
/*离开过渡的结束状态，元素被删除时生效，离开过渡完成后被删除*/
.fade-leave-active {
    /* opacity:0; */
    transition: opacity .5s;
}
</style>