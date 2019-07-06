<template>
  <Wip/>
</template>

<script lang="ts">
  import { Component } from 'vue-property-decorator';
  import Oom from '@/components/Oom.vue';
  import Wip from '@/components/Wip.vue';

  @Component({
    components: {
      Wip,
    },
  })
  export default class Users extends Oom {
  }
</script>
