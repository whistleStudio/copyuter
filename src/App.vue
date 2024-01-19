<template>
  <div>
    计算结果：{{ res }}
  </div>
  <div>
    <button @click="start">开始</button>
    <button @click="over">结束</button>
    <button @click="invoke('repeat')">重复</button>
    <button @click="invoke('save')">保存</button>
  </div>
  </template>
  
  
  <script setup lang="ts">
    import {ref, Ref} from "vue"
    import { invoke } from "@tauri-apps/api";
    import { listen, UnlistenFn} from '@tauri-apps/api/event';
  // import { appWindow } from "@tauri-apps/api/window";
  
    let unlisten:UnlistenFn
    ;(
      async ()=>{
      unlisten = await listen<string>('event-name', (event) => {
      console.log(`Got error in window ${event.windowLabel}, payload: ${event.payload}`);
      unlisten()
    });
    })()
  
    const res: Ref<number | null> = ref(null)
    
    function start () {
      invoke("start_record")
    }
  
    function over () {
      invoke("stop_record")
    }
  </script>
  
  
  <style scoped>
    div>button {
      margin:10px 10px 10px 0;
    }
  
  </style>
  