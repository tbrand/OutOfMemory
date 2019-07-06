<!-- Source -->
<!-- https://github.com/tobiasahlin/SpinKit -->
<!-- License -->
<!-- https://github.com/tobiasahlin/SpinKit/blob/master/LICENSE -->
<template>
  <div class="spinner" :style="spinnerStyle">
    <div class="rect1" :style="rectStyle"></div>
    <div class="rect2" :style="rectStyle"></div>
    <div class="rect3" :style="rectStyle"></div>
    <div class="rect4" :style="rectStyle"></div>
    <div class="rect5" :style="rectStyle"></div>
  </div>
</template>

<script lang="ts">
  import { Component, Prop } from 'vue-property-decorator';
  import Oom from '@/components/Oom.vue';
  
  @Component
  export default class Loading extends Oom {
    @Prop() private size!: string;

    private get spinnerStyle(): any {
      switch (this.size) {
        case 'lg':
          return { width: '50px', height: '40px' };
        case 'sm':
          return { width: '25px', height: '20px' };
        default:
          return { width: '25px', height: '20px' };
      }
    }

    private get rectStyle(): any {
      switch (this.size) {
        case 'lg':
          return { width: '6px' };
        case 'sm':
          return { width: '3px' };
        default:
          return { width: '3px' };
      }
    }
  }
</script>

<style lang="scss" scoped>
  .spinner {
    text-align: center;
    font-size: 10px;
  }

  .spinner > div {
    background-color: #409EFF;
    height: 100%;
    width: 5px;
    display: inline-block;
    
    -webkit-animation: sk-stretchdelay 1.2s infinite ease-in-out;
    animation: sk-stretchdelay 1.2s infinite ease-in-out;
  }

  .spinner .rect2 {
    -webkit-animation-delay: -1.1s;
    animation-delay: -1.1s;
  }

  .spinner .rect3 {
    -webkit-animation-delay: -1.0s;
    animation-delay: -1.0s;
  }

  .spinner .rect4 {
    -webkit-animation-delay: -0.9s;
    animation-delay: -0.9s;
  }

  .spinner .rect5 {
    -webkit-animation-delay: -0.8s;
    animation-delay: -0.8s;
  }

  @-webkit-keyframes sk-stretchdelay {
    0%, 40%, 100% { -webkit-transform: scaleY(0.4) }  
    20% { -webkit-transform: scaleY(1.0) }
  }

  @keyframes sk-stretchdelay {
    0%, 40%, 100% { 
      transform: scaleY(0.4);
      -webkit-transform: scaleY(0.4);
    }  20% { 
      transform: scaleY(1.0);
      -webkit-transform: scaleY(1.0);
    }
  }
</style>
