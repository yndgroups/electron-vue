<template>
    <div>
      <button
        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
        @click="createWindow(1)"
      >
        登录
      </button>
  
      <ul>
          <li @click="toTest">to Test</li>
      </ul>
    </div>
  </template>
  <script lang="ts" setup>
  import { useRoute, useRouter } from 'vue-router';
  const router = useRouter();
  const toTest = () => {
      router.push('test')
  }
  function createWindow(type: number) {
    console.log(window.Cmd.test(type));
  }
  </script>
  