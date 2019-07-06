<template>
  <div id="app" class="app-container">
  <div class="app-header">
    <Header />
  </div>

  <div class="app-under-header">
    <div class="app-side-menu">
      <SideMenu />
    </div>

    <div class="app-content">
      <div class="app-content-main">
        <router-view />

        <div class="app-footer">
          <Footer />
        </div>
      </div>
      <div class="app-content-info">
        <Info />
      </div>
    </div>
  </div>
  </div>
</template>

<script lang="ts">
  import { Component, Vue } from 'vue-property-decorator';
  import Header from '@/views/Header.vue';
  import Info from '@/views/Info.vue';
  import Footer from '@/views/Footer.vue';
  import SideMenu from '@/views/SideMenu.vue';

  @Component({
    components: {
      Header,
      Info,
      Footer,
      SideMenu,
    },
  })
  export default class App extends Vue { }
</script>

<style lang="scss">
  #app {
    font-family: 'Avenir', Helvetica, Arial, sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    text-align: center;
    color: #2c3e50;
  }

  .app-container {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .app-header {
    height: 60px;
  }

  .app-under-header {
    flex: 1;
    display: flex;
    justify-content: center;
    height: 100%;
  }

  .app-side-menu {
    width: 250px;
  }

  .app-content {
    flex: 1;
    display: flex;
    justify-content: flex-start;
    height: 100%;
    overflow-y: scroll;
  }

  .app-content-main {
    display: flex;
    justify-content: flex-start;
    flex-direction: column;
    flex: 1;
  }

  .app-content-info {
    width: 340px;
  }

  .app-footer {
    height: 60px;
  }

  a {
    text-decoration: none;

    &:link {
      color: #005999;
    }

    &:visited {
      color: #003777;
    }

    &:hover {
      color: #227BBB;
    }

    &:active {
      color: #227BBB;
    }
  }

  .el-divider {
    margin: 0;
  }
</style>
