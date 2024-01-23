<template>
  <div>
    <!-- 计算结果：{{ res }} -->
  </div>
  <div class="btn-group1">
    <button type="button" class="btn btn-primary" @click="start">开始</button>
    <button @click="over">结束</button>
    <button @click="invoke('repeat')">测试</button>
    <button class="btn btn-primary" data-bs-toggle="modal" data-bs-target="#exampleModal" 
    @click="invoke('save')">保存</button>
  </div>
  <ul class="logs">
    <li v-for="(v, i) in logList">
      <input v-if="i == curLogIdx" type="text">
      <div v-else>{{ v }}</div>
      <button @click="curLogIdx = i">编辑</button>
      <button @click="runLog(v)">运行</button>
      <button>删除</button>
    </li>
  </ul>

  <a-space wrap>
    <a-button type="primary">Primary Button</a-button>
    <a-button>Default Button</a-button>
    <a-button type="dashed">Dashed Button</a-button>
    <a-button type="text">Text Button</a-button>
    <a-button type="link">Link Button</a-button>
  </a-space>




  </template>
  
  
  <script setup lang="ts">
    import {ref, Ref, onMounted, reactive} from "vue"
    import { invoke } from "@tauri-apps/api";
    // import * as bootstrap from 'bootstrap';
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
    const logList: String[] = reactive([]);
    
    function start () {
      invoke("start_record")
    }
  
    function over () {
      invoke("stop_record")
    }

    /* 点击运行 */
    function runLog (f: String) {
      invoke<number>("run_log", {f}).then(err => console.log(err));
    }

    onMounted (() => {
      invoke<String[]>("get_filenames").then(d => logList.push(...d))
    })
  </script>
  
  
  <style scoped lang="scss">
  .btn-group1 {
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
  