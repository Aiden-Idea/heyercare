<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';


const checkAppUpdate = async () => {
  await invoke('run_check_update')
}

</script>
<template>
  <button @click="checkAppUpdate">更新</button>
</template>