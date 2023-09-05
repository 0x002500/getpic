<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
</script>

<template>
    <h1>GetPic</h1>
    <input type="number" name="number of shots" size="100" v-model="nos" />
    <button @click="register()">Run!</button>
  </template>
  
  <script>
  export default {
    data() {
      return {
        nos: "",
      };
    },
    methods: {
      register() {
        const inputValue = this.nos;
        // 这里可以使用inputValue进行进一步处理或传递
        invoke('getpic_core', { n: inputValue });
      },
    },
  };
  </script>