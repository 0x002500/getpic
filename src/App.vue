<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
</script>

<template>
  <button @click = "invoke('getpic_core')">Run!</button>
</template>