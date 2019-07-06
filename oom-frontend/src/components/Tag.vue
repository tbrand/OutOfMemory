<template>
  <div class="tag-container">
    <template v-if="actionType === 'event'">
      <el-tag size="mini" class="tag-inner" @click="tagEvent">{{ tag.name }}</el-tag>
    </template>
    <template v-else-if="actionType === 'outerLink'">
      <router-link :to="tagLink" tag="a" target="_blank">
        <el-tag size="mini" class="tag-inner">{{ tag.name }}</el-tag>
      </router-link>
    </template>
    <template v-else>
      <router-link :to="tagLink">
        <el-tag size="mini" class="tag-inner">{{ tag.name }}</el-tag>
      </router-link>
    </template>
  </div>
</template>

<script lang="ts">
  import { Component, Prop } from 'vue-property-decorator';
  import Oom from '@/components/Oom.vue';

  @Component
  export default class Tag extends Oom {
    @Prop() private tag!: any;
    @Prop() private actionType!: string;

    private tagEvent(): void {
      this.$emit('tagEvent', Number(this.tag.id));
    }

    private get tagLink(): object {
      return {
        name: 'tag',
        params: {
          id: this.tag.id,
        }
      };
    }
  }
</script>

<style lang="scss">
  .tag-container {
    padding-right: 6px;
  }

  .tag-inner {
    color: #409EFF;
    font-weight: 500;
    cursor: pointer;
  }
</style>
