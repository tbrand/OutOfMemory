<template>
  <div class="post-body-form-container">
    <el-tabs type="card" @tab-click="switchTab">
      <el-tab-pane label="Edit"/>
      <el-tab-pane label="Preview"/>
    </el-tabs>

    <el-input type="textarea" rows="10" v-model="body" v-if="isEdit"/>

    <div class="post-body-form-preview" v-else>
      <RenderMarkdown :body="body"/>
    </div>
  </div>
</template>

<script lang="ts">
  import { Component, Prop, Watch } from 'vue-property-decorator';
  import Oom from '@/components/Oom.vue';
  import RenderMarkdown from '@/components/RenderMarkdown.vue';

  @Component({
    components: {
      RenderMarkdown,
    },
  })
  export default class PostBodyForm extends Oom {
    @Prop() private resetKey!: number;

    private body: string = '';
    private isEdit: boolean = true;

    @Watch('resetKey', { immediate: true })
    private resetBody(): void {
      this.body = '';
    }

    @Watch('body', { immediate: true })
    private syncBody(): void {
      this.$emit('syncBody', this.body);
    }

    private switchTab(tab: any, event: any): void {
      this.isEdit = tab.label === 'Edit';
    }
  }
</script>

<style lang="scss" scoped>
  @import '../vendor/google-code-prettify/prettify.css';

  .post-body-form-container {}

  .post-body-form-preview {
    padding: 10px;
    border: 1px solid #DCDFE6;
    border-radius: 5px;
  }
</style>
