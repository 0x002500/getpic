<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
</script>

<template>
  <h1>GetPic</h1>
  <input type="number" id="number of shots" name="number of shots" size="100" />
  <button @click = "invoke('getpic_core', {n: m})">Run!</button>
</template>