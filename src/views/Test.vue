<template>
    <div>
      <button
        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
        @click="createWindow(1)"
      >
        Test
      </button>

      <div @click="goBack">返回</div>
    </div>
  </template>
  <script lang="ts" setup>
  import { useRoute, useRouter } from 'vue-router';
  const router = useRouter();

  function goBack() {
    router.back();
  }

  function createWindow(type: number) {
    console.log(window.Cmd.test(type));
  }
  </script>
  