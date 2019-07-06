<template>
  <div class="render-markdown-container">
    <div v-html="bodyCompiled"/>
  </div>
</template>

<script lang="ts">
  import { Component, Prop, Watch } from 'vue-property-decorator';
  import Oom from '@/components/Oom.vue';
  import marked from 'marked';

  @Component
  export default class RenderMarkdown extends Oom {
    @Prop() private body!: string;

    private bodyCompiled: string = '';

    @Watch('body', { immediate: true })
    private compile(): void {
      this.bodyCompiled = marked(this.body, { sanitize: true })
        .replace(
          /<pre>/g,
          '<pre class="prettyprint" ' +
          'style="background-color: #f2f2f2; border: none; padding: 10px;">'
        );
    }

    @Watch('bodyCompiled', { immediate: true })
    private prettyPrint(): void {
      this.$nextTick(() => {
        PR.prettyPrint();
      });
    }
  }
</script>

<style lang="scss" scoped>
  .render-markdown-container {
    text-align: left;
  }
</style>
