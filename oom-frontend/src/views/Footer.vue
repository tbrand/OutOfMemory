<template>
  <div class="footer-container">
    <div class="footer-copy-right">
      Copyright &copy; 2019 Taichiro Suzuki. All Rights Reserved.
    </div>
  </div>
</template>

<script lang="ts">
  import { Component } from 'vue-property-decorator';
  import Oom from '@/components/Oom.vue';

  @Component
  export default class Footer extends Oom { }
</script>

<style lang="scss" scoped>
  .footer-container {
    padding: 30px 0 10px 0;
  }

  .footer-copy-right {
    font-size: 12px;
    color: #6a737c;
  }
</style>
