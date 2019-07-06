<template>
  <div class="header-container">
    <div class="header-content">
      <router-link :to="{ name: 'home' }" class="header-logo">
        <div class="header-logo-container">
          <div class="header-logo-main">
            OutOfMemory
          </div>
          <div class="header-logo-sub">
            Developers Community
          </div>
        </div>
      </router-link>
      <div class="header-search">
        <el-input placeholder="Search words" v-model="searchInput" class="input-with-select">
          <el-button slot="append" icon="el-icon-search"></el-button>
        </el-input>
      </div>
      <div class="header-users">
        <el-dropdown @command="dropdown" v-if="isLogin && user">
          <span class="el-dropdown-link">
            <img :src="user.avatar_url" width="30px" height="30px" style="border-radius: 50%;"/>
          </span>
          <el-dropdown-menu slot="dropdown">
            <el-dropdown-item command="profile"><fa icon="user"/> Profile</el-dropdown-item>
            <el-dropdown-item command="logout"><fa icon="sign-out-alt"/> Logout</el-dropdown-item>
          </el-dropdown-menu>
        </el-dropdown>

        <el-dropdown @command="dropdown" v-else>
          <span class="el-dropdown-link">
            <fa icon="user" class="header-users-avatar-icon"/>
          </span>
          <el-dropdown-menu slot="dropdown">
            <el-dropdown-item command="login"><fa icon="sign-in-alt"/> Login</el-dropdown-item>
          </el-dropdown-menu>
        </el-dropdown>
      </div>
    </div>

    <div class="divider"/>
  </div>
</template>

<script lang="ts">
  import { Component, Watch } from 'vue-property-decorator';
  import Oom from '@/components/Oom.vue';
  import Api from '@/modules/api';
  import login from '@/modules/login';

  @Component
  export default class Header extends Oom {
    private searchInput: string = '';
    private user: any | null = null;

    private async dropdown(command: string): Promise<void> {
      switch (command) {
        case 'profile':
          this.$router.push({ name: 'user', params: { id: `${1}` }});
          break;
        case 'logout':
          this.$removeCookie('oom');
          break;
        case 'login':
          await login();
          break;
      }
    }

    @Watch('$cookies', { immediate: true })
    private async syncUser(): Promise<void> {
      if (this.isLogin) {
        const api = new Api(this.$cookies);
        const res = await api.fetchUser(this.authCookie.userId);

        this.user = res.data;
      }
    }
  }
</script>

<style lang="scss" scoped>
  .header-container {
  }

  .header-content {
    display: flex;
    justify-content: center;
    align-items: center;
    font-size: 14px;
    height: 58px;
  }

  .header-logo {
    width: 260px;
    text-align: left;
    padding-left: 30px;
  }

  .header-logo-container {
    width: 150px;
    text-align: center;
  }

  .header-logo-main {
    font-size: 16px;
    font-weight: 600;
  }

  .header-logo-sub {
    font-size: 10px;
  }

  .header-search {
    flex: 1;
    text-align: center;
  }

  .header-users {
    width: 280px;
    text-align: right;
    padding-right: 30px;
  }

  .header-users-avatar-icon {
    font-size: 20px;
  }

  .divider {
    width: 100%;
    border-top: 1px solid #e6e6e6;
  }
</style>
