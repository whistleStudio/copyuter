<template>
  <context-holder />
  <div class="btn-group1">
    <a-button type="primary" @click="start">开始</a-button>
    <a-button type="primary" @click="over">结束</a-button>
    <a-button type="primary" @click="test">测试</a-button>
    <a-button type="primary" class="btn btn-primary" data-bs-toggle="modal" data-bs-target="#exampleModal" 
    @click="save">保存</a-button>
    <a-button type="primary" @click="countdownWin.display()">窗口0</a-button>
    <a-button type="primary" @click="countdownWin.display()">窗口1</a-button>

  </div>
  <ul class="logs">
    <li v-for="(v, i) in logList" :class="{active: curLogIdx==i}">
      <div>{{ v }}</div>
      <a-button @click="editLog(i)">编辑</a-button>
      <a-button @click="runLog(v+'.log')">运行</a-button>
      <a-button @click="deleteLog(v+'.log')">删除</a-button>
    </li>
  </ul>

  <a-modal v-model:open="open" :title="modalTitles[modalMode]" @ok="handleOk(modalMode)" @cancel="handleCancel()">
    <a-input v-model:value.trim="logName" placeholder="请输入名称" />
  </a-modal>

  <div class="bg"></div>
</template>
  
  
<script setup lang="ts">
  import {ref, Ref, onMounted, reactive} from "vue"
  import { invoke } from "@tauri-apps/api";
  import { message } from 'ant-design-vue';
  import { once } from "@tauri-apps/api/event";
  import recDotWin from "./views/rec-dot/rec-dot-win"
  import countdownWin from "./views/countdown/countdown-win";


  const [messageApi, contextHolder] = message.useMessage();
  const curLogIdx: Ref<number> = ref(-1), open = ref(false), logName = ref(""), modalMode = ref(0),
    modalTitles= ref(["", "新建动作名称", "重命名动作名称"])
  const logList: string[] = reactive([])
  
  function start () {
    invoke("start_record")
    recDotWin.display(0)
  }

  function over () {
    invoke("stop_record")
    recDotWin.display(-1)
  }
  async function test () {
    invoke('repeat')
    recDotWin.display(1)
    once("repeat_over", () => recDotWin.display(-1))
  }
  function save () {
    modalMode.value = 1; open.value = true; logName.value = ""
  }
  /* 记录编辑 */
  function editLog (i: number) {
    curLogIdx.value = i
    modalMode.value = 2; open.value = true; logName.value = logList[i]
  }
  /* 记录运行 */
  async function runLog (f: string) {
    invoke<number>("run_log", {f}).then(err => {
      if (err) messageApi.error("运行失败：当前动作异常")
    });
    recDotWin.display(1)
    once("repeat_over", () => {console.log("repeat over"); recDotWin.display(-1)})
    console.log("oooo")
  }
  /* 记录删除 */
  function deleteLog (f: string) {
    invoke<number>("delete_log", {f}).then(err => {
      if (err) messageApi.error("删除失败")
      else {
        let i = logList.indexOf("f");
        logList.splice(i, 1);
      }
    });
  }
  /* Modal cancel */
  function handleCancel () {open.value = false; curLogIdx.value = -1}
  /* Modal ok */
  function handleOk (mode: number) {
    switch (mode) {
      case 1:
        modalSave()
        break
      case 2:
        modalEdit()
        break
      default:
        break
    }
    open.value = false
  }
  /* Modal ok: 保存 */
  const modalSave = () => {
    if (!logName.value) messageApi.error('保存失败：当前名称为空或存在非法字符');
    else if (logList.indexOf(logName.value) >= 0) messageApi.error(`保存失败：动作 ${logName.value} 已存在`);
    else invoke<number>("save", {name: logName.value+".log"}).then(err => {
      if (err == 1) messageApi.error("保存失败：当前动作为空")
      else logList.push(logName.value)
    })
  }
  /* Modal ok: 编辑 */
  const modalEdit = () => {
    const i = curLogIdx.value
    if (!logName.value) messageApi.error('编辑失败：当前名称为空或存在非法字符');
    else if (logList[i] != logName.value){
      const from = logList[i]+".log"
      const to = logName.value+".log"
      invoke<number>("edit_log", {from, to}).then(err => {
        if (err) messageApi.error("编辑失败：文件操作异常")
        else logList[i] = logName.value
      })
    }
    curLogIdx.value = -1
  }

  onMounted (() => {
    invoke<string[]>("get_filenames").then(d => logList.push(...(d.map(v => v.slice(0, -4)))))
    recDotWin.create()
  })
</script>

<style scoped lang="scss">
@import url('./app.scss');
</style>
./views/rec-dot/rec-dot-win