<template>
    <div class="global-icon" :class="{rec: iconId==0, play: iconId==1}">
    
    </div>
</template>


<script setup lang="ts">
import { listen } from '@tauri-apps/api/event';
import { ref } from 'vue';

interface P {
  i: number
}

const iconId = ref(-1)

console.log("ok")
listen<P>("change", ev  => {
  iconId.value = ev.payload.i
})
</script>


<style scoped lang="scss">
.global-icon {
  animation: blink 1s infinite;
  &.rec {
    width: 50px;
    height: 50px;
    border-radius: 50%;
    background-color: red;
  }
  &.play {
    width: 0;
    border: solid transparent;
    border-width: 25px 0px 25px 40px;
    border-left-color: green;
  }

  @keyframes blink {
    from { opacity: 0; }
    to { opacity: 1; }
  }
}

</style>
    