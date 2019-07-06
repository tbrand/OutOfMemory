<template>
  <div class="question-container">
    <div :class="upsClass">
      {{ question.count_ups }}
    </div>

    <div class="question-title">
      <router-link :to="questionLink">
        {{ question.title }}
      </router-link>
    </div>
  </div>
</template>

<script lang="ts">
  import { Component, Prop } from 'vue-property-decorator';
  import Oom from '@/components/Oom.vue';

  @Component
  export default class QuestionRowShort extends Oom {
    @Prop() question: any;

    private get questionLink(): any {
      return {
        name: 'questions',
        params: {
          id: this.question.id,
        },
      };
    }

    private get upsClass(): string {
      if (this.question.is_solved) {
        return 'question-ups solved';
      }

      return 'question-ups';
    }
  }
</script>

<style lang="scss" scoped>
  .question-container {
    display: flex;
    justify-content: flex-start;
    align-items: center;
    padding: 0 10px 6px 5px;
    font-size: 14px;
  }

  .question-ups {
    padding: 2px 8px;
    border: 1px solid #e3e3e3;
    border-radius: 5px;
    margin-right: 10px;
  }

  .question-ups.solved {
    background-color: #409EFF;
    border-color: #409EFF;
    color: #ffffff;
  }

  .question-title {
    flex: 1;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
    cursor: pointer;
    width :200px;
  }
</style>
