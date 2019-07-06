<template>
  <div class="side-menu-container">
    <div :class="menuClass('home')" @click="switchRoute('home')">
      <fa icon="home" />
      Home
    </div>

    <div :class="menuClass('users')" @click="switchRoute('users')">
      <fa icon="users"/>
      Users
    </div>

    <div :class="menuClass('tags')" @click="switchRoute('tags')">
      <fa icon="tags"/>
      Tags
    </div>
  </div>
</template>

<script lang="ts">
  import { Component, Watch } from 'vue-property-decorator';
  import Oom from '@/components/Oom.vue';

  @Component
  export default class SideMenu extends Oom {
    private activeRoute: string = 'home';

    private switchRoute(route: string): void {
      switch (route) {
        case 'home':
          this.$router.push({ name: 'home' });
          break;
        case 'users':
          this.$router.push({ name: 'users' });
          break;
        case 'tags':
          this.$router.push({ name: 'tags' });
          break;
        default:
          break;
      }
    }

    @Watch('$route', { immediate: true })
    private syncRoute(): void {
      if (this.$route.name === 'users' || this.$route.name === 'tags') {
        this.activeRoute = this.$route.name;
      } else {
        this.activeRoute = 'home';
      }
    }

    private menuClass(route: string): string {
      if (this.activeRoute === route) {
        return 'side-menu-row active';
      } else {
        return 'side-menu-row';
      }
    }
  }
</script>

<style lang="scss" scoped>
  .side-menu-container {
    text-align: left;
    padding-left: 40px;
  }

  .side-menu-row {
    color: #3e3e3e;
    font-size: 14px;
    margin: 14px 20px 0 24px;
    padding: 8px 16px;
    cursor: pointer;
  }

  .side-menu-row.active {
    background-color: #f0f0f0;
    border-right: 6px solid #409EFF;
  }

  .svg-inline--fa {
    padding-right: 10px;    
  }
</style>
