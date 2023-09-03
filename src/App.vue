<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
</script>

<template>
  <h1>GetPic</h1>
  <input type="number" id="nos" name="number of shots" size="100" />
  <button @click="invoke('getpic_core', { n: m })">Run!</button>
</template>

<script setup lang="js">
function getValue() {
  // 获取输入框元素
  var inputElement: any = document.getElementById("nos");
  // 获取输入框的值
  var inputValue: any = inputElement.value;
  // 将输入框的值赋值给其他变量
  var m = inputValue;
}
</script>