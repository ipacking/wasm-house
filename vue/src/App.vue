<script setup>
import { ref, onMounted } from "vue";
import init, { add } from "@/wasm/slint";

onMounted(() => {
  console.log("mounted...");
  init();
});

const test_add = () => {
  alert(add(1, 2));
}

</script>

<template>
  <div>
    <canvas id="canvas"></canvas>
    <div>
      <button @click="test_add">add</button>
    </div>
  </div>
</template>

<style scoped>

</style>
