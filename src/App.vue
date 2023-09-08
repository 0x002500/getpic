<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
</script>

<template>
  <h1 class="text-gray-700 text-center bg-gray-400 px-4 py-2 m-2">GetPic</h1>
  <input type="number"
    class="peer block min-h-[auto] w-full rounded border-0 bg-transparent px-3 py-[0.32rem] leading-[1.6] outline-none transition-all duration-200 ease-linear focus:placeholder:opacity-100 peer-focus:text-primary data-[te-input-state-active]:placeholder:opacity-100 motion-reduce:transition-none dark:text-neutral-200 dark:placeholder:text-neutral-200 dark:peer-focus:text-primary [&:not([data-te-input-placeholder-active])]:placeholder:opacity-0 text-gray-700 text-center bg-gray-400 px-4 py-2 m-2"
    id="exampleFormControlInputNumber" placeholder="Example label" v-model="nos" />
  <label for="exampleFormControlInputNumber"
    class="pointer-events-none absolute left-3 top-0 mb-0 max-w-[90%] origin-[0_0] truncate pt-[0.37rem] leading-[1.6] text-neutral-500 transition-all duration-200 ease-out peer-focus:-translate-y-[0.9rem] peer-focus:scale-[0.8] peer-focus:text-primary peer-data-[te-input-state-active]:-translate-y-[0.9rem] peer-data-[te-input-state-active]:scale-[0.8] motion-reduce:transition-none dark:text-neutral-200 dark:peer-focus:text-primary">Number
    input
  </label>
  <button @click="register()">Run!</button>
</template>
  
<script lang="ts">
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