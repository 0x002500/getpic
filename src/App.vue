<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
</script>

<template>
  <h1>GetPic</h1>
  <input type="number" id="nos" name="number of shots" size="100" />
  <button @click="invoke('getpic_core', { n: m })">Run!</button>
</template>

<script setup lang="ts">
	export default {
       name: 'Login',
        data() {
            return {
                username: '',
                password: ''
            }
        },
        methods: {c
            login() {
                   console.log(this.username)
                   console.log(this.password)
            }
        }
    }
	<script/>