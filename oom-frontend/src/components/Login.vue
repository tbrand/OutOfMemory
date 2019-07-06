<template>
  <div class="login-container">
  <div class="login-message">
    You need to login to {{ doWhat }}.
  </div>

  <el-button type="primary" round @click="login">
    Login
  </el-button>
  </div>
</template>

<script lang="ts">
  import { Component, Prop } from 'vue-property-decorator';
  import Oom from '@/components/Oom.vue';
  import login from '@/modules/login';

  @Component
  export default class Login extends Oom {
    @Prop() private doWhat!: string;

    private async login(): Promise<void> {
      await login();
    }
  }
</script>

<style lang="scss" scoped>
  .login-container {
    text-align: center;
  }

  .login-message {
    margin: 20px;
  }
</style>
