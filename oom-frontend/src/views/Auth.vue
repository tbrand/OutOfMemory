<template>
  <div class="auth-container">
    Please wait for a while... <i class="el-icon-loading"></i>
  </div>
</template>

<script lang="ts">
  import { Component, Watch } from 'vue-property-decorator';
  import Oom from '@/components/Oom.vue';
  import Api from '@/modules/api';
  import queryString from 'query-string';

  @Component
  export default class Auth extends Oom {

    @Watch('$route', { immediate: true })
    private async login(): Promise<void> {
      const code = this.$route.query.code as string || '';
      const api = new Api(null);
      const res = await api.login(code);

      if (res.status === 200) {
        this.$setCookie('oom', queryString.stringify(res.data), { expires: 1 });

        // TODO: back to previous page
        this.$router.push({ name: 'home' });
      }
    }
  }
</script>

<style lang="scss" scoped>
  .auth-container {
    padding: 30px;
    font-size: 20px;
  }
</style>
