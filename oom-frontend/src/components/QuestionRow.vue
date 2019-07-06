<template>
  <div class="question-container">
    <div class="question-num-col">
      <div class="question-num-col-num">
        {{ question.count_ups }}
      </div>
      <div class="question-num-col-type">
        ups
      </div>
    </div>

    <div :class="answerClass">
      <div class="question-num-col-num">
        {{ question.count_answers }}
      </div>
      <div class="question-num-col-type">
        answers
      </div>
    </div>

    <div class="question-num-col">
      <div class="question-num-col-num">
        {{ question.count_views }}
      </div>
      <div class="question-num-col-type">
        views
      </div>
    </div>

    <div class="question-content">
      <div class="question-title">
        <router-link tag="a" :to="questionLink">
          {{ question.title }}
        </router-link>
      </div>

      <div class="question-description">
        <div class="question-tags">
          <Tag :tag="tag" :actionType="'link'" v-for="tag in tags" v-bind:key="tag.id"/>
        </div>
        <div class="question-author">
          modified 9 hours ago

          <template v-if="user">
            <router-link :to="userLink">
              {{ user.name }}
            </router-link>
          </template>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
  import { Component, Prop, Watch } from 'vue-property-decorator';
  import Oom from '@/components/Oom.vue';
  import Tag from '@/components/Tag.vue';
  import Api from '@/modules/api';

  @Component({
    components: {
      Tag,
    },
  })
  export default class QuestionRow extends Oom {
    @Prop() private question: any;

    private user: any | null = null;
    private tags: any[] = [];

    private get questionLink(): any {
      return {
        name: 'questions',
        params: {
          id: this.question.id,
        },
      };
    }

    @Watch('question', { immediate: true })
    private async syncUser(): Promise<void> {
      const api = new Api(this.$cookies);
      const res = await api.fetchUser(this.question.user_id);

      this.user = res.data;
    }

    @Watch('question', { immediate: true })
    private async syncTags(): Promise<void> {
      const api = new Api(this.$cookies);
      const res = await api.fetchPostTags(this.question.id);

      this.tags = res.data;
    }

    private get userLink(): any {
      return {
        name: 'user',
        params: {
          id: this.user!.id,
        },
      };
    }

    private get answerClass(): any {
      if (this.question.is_solved) {
        return 'question-num-col solved';
      }

      return 'question-num-col';
    }
  }
</script>

<style lang="scss" scoped>
  .question-container {
    display: flex;
    justify-content: center;
    text-align: center;
    padding: 10px;
  }

  .question-num-col {
    width: 44px;
    color: #6a737c;
    padding-top: 5px;

    .solved {
      border: 1px solid #409EFF;
      border-radius: 4px;
    }
  }

  .question-num-col-num {
    font-size: 16px;
    font-weight: 600;
  }

  .question-num-col-type {
    font-size: 10px;
  }

  .question-content {
    flex: 1;
    padding-left: 15px;
  }

  .question-title {
    font-size: 16px;
    text-align: left;
    margin-bottom: 8px;
    word-break : break-all;
  }

  .question-description {
    display: flex;
    justify-content: flex-start;
  }

  .question-tags {
    flex: 1 1 100px;
    display: flex;
    justify-content: flex-start;
  }

  .question-tag {
    margin-right: 4px;
  }

  .question-author {
    width: 300px;
    font-size: 12px;
    text-align: right;
  }

  .el-tag {
    background-color: #E1ECF4;
    border-color: #E1ECF4;
    color: #39739D;
    cursor: pointer;

    &:hover {
      background-color: #C0CAD2;
      border-color: #C0CAD2;
    }
  }
</style>
