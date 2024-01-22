<template>
  <div>
    <!-- 计算结果：{{ res }} -->
  </div>
  <div class="btn-group">
    <button @click="start">开始</button>
    <button @click="over">结束</button>
    <button @click="invoke('repeat')">重复</button>
    <button @click="invoke('save')">保存</button>
  </div>
  <ul class="logs">
    <li v-for="i in 3">
      <input v-if="i == curLogIdx" type="text">
      <div v-else></div>
      <button @click="curLogIdx = i">编辑</button>
      <button>运行</button>
    </li>
  </ul>
  </template>
  
  
  <script setup lang="ts">
    import {ref, Ref} from "vue"
    import { invoke } from "@tauri-apps/api";
    // import { listen, UnlistenFn} from '@tauri-apps/api/event';
    // import { appWindow } from "@tauri-apps/api/window";
  
    // let unlisten:UnlistenFn
    // ;(
    //   async ()=>{
    //   unlisten = await listen<string>('event-name', (event) => {
    //   console.log(`Got error in window ${event.windowLabel}, payload: ${event.payload}`);
    //   unlisten()
    // });
    // })()
  
    // const res: Ref<number | null> = ref(null)
    const curLogIdx: Ref<number> = ref(-1)
    
    function start () {
      invoke("start_record")
    }
  
    function over () {
      invoke("stop_record")
    }
  </script>
  
  
  <style scoped lang="scss">
  .btn-group {
    button {
      margin:10px 10px 10px 0;
    }
  }
  .logs {
    li {
      margin: 10px 0 10px;
      display: flex;
      align-items: center;
      >input{
        width: 260px;
        height: 20px;
      }
      >div {
        width: 300px;
        height: 50px;
        display: inline-block;
      }
      button {
        margin-left: 20px;
      }
    }
  }
  
  </style>
  