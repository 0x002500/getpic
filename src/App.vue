<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
</script>

<template>
  <h1>GetPic</h1>
  <input type="number" id="nos" name="number of shots" size="100" />
  <button @click="invoke('getpic_core', { n: "5" })">Run!</button>
</template>

<script setup lang="js">
function getValue() {
  // 获取输入框元素
  var inputElement = document.getElementById("myInput");
  // 获取输入框的值
  var inputValue = inputElement.value;
  // 将输入框的值赋值给其他变量
  var otherVariable = inputValue;
}
</script>