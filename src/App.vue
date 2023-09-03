<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
</script>

<template>
  <h1>GetPic</h1>
  <button @click = "invoke('getpic_core', {n: '5'})">Run!</button>
</template>