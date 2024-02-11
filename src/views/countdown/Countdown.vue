<template>
  <div class="main">
    <div class="num">{{ num }}</div>
  </div>
</template>

<script setup lang="ts">
  import { emit } from '@tauri-apps/api/event';
  import { ref, onMounted } from 'vue';

  const num = ref(3)

  onMounted(() => {
    const tim = setInterval(() => {
      num.value --
      if (num.value < 1) {
        emit("countdown_over")
        clearInterval(tim)
      }
    }, 1000)
  })
</script>

<style scoped>
.main {
  width: 150px;
  height: 150px;
  background-color: burlywood;
  text-align: center;
  border-radius: 50%;
  .num {
    line-height: 150px;
    font-size: 100px;
    color: white;
    opacity: 0.8;
  }
}
</style>