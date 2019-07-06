<template>
  <div class="wip">
    This is NOT an error! But just the page is not created!
  </div>
</template>

<script lang="ts">
  import { Component } from 'vue-property-decorator';
  import Oom from '@/components/Oom.vue';
  
  @Component
  export default class Wip extends Oom { }
</script>

<style lang="scss">
  .wip {
    padding: 30px;
  }
</style>
