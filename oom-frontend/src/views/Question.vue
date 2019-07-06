<template>
  <div class="question-container">
    <div class="question-top">
      <div class="question-title">
        {{ questionTitle }}
      </div>

      <router-link :to="{ name: 'ask' }">
        <el-button type="primary" round>
          Ask Question
        </el-button>
      </router-link>
    </div>

    <el-divider/>

    <QuestionInner :questionInner="questionInner"
                   :questionUserId="questionUserId"
                   v-if="questionInner"/>

    <div class="question-answers">
      {{ answersInner.length }} Answers
    </div>

    <template v-for="answerInner in answersInner">
      <el-divider/>
      <AnswerInner :answerInner="answerInner" :questionUserId="questionUserId"/>
    </template>

    <div class="question-answer-header">
      Your Answer
    </div>

    <template v-if="isLogin">
      <PostBodyForm @syncBody='syncAnswerBody' :resetKey="resetKey"/>

      <div class="question-answer-footer">
        <el-button type="primary" round @click="createAnswer">
          Post Answer
        </el-button>
      </div>
    </template>

    <template v-else>
      <Login :doWhat="'create an answer'"/>
    </template>
  </div>
</template>

<script lang="ts">
  import { Component, Watch } from 'vue-property-decorator';
  import Oom from '@/components/Oom.vue';
  import QuestionInner from '@/components/Question.vue';
  import AnswerInner from '@/components/Answer.vue';
  import PostBodyForm from '@/components/PostBodyForm.vue';
  import Login from '@/components/Login.vue';
  import Api from '@/modules/api';

  @Component({
    components: {
      QuestionInner,
      AnswerInner,
      PostBodyForm,
      Login,
    },
  })
  export default class Question extends Oom {
    private questionInner: any | null = null;
    private answersInner: any[] = [];
    private answerBody: string = '';
    private questionTitle: string = '';
    private questionUserId: number = 0;
    private resetKey: number = 0;

    private get postId(): number {
      return Number(this.$route.params.id);
    }

    @Watch('postId', { immediate: true })
    private async syncQuestion(): Promise<void> {
      const api = new Api(this.$cookies);
      const res = await api.fetchQuestion(this.postId);

      this.questionTitle = res.data.title;
      this.questionUserId = res.data.user_id;
      this.questionInner = res.data;
    }

    @Watch('postId', { immediate: true })
    private async syncAnswers(): Promise<void> {
      const api = new Api(this.$cookies);
      const res = await api.fetchAnswers(this.postId, { page: 0 });

      this.answersInner = res.data;
    }

    @Watch('postId', { immediate: true })
    @Watch('isLogin', { immediate: true })
    private async addView(): Promise<void> {
      if (this.isLogin) {
        const api = new Api(this.$cookies);
        await api.upsertViews(this.postId);
      }
    }

    private async createAnswer(): Promise<void> {
      const api = new Api(this.$cookies);
      const res = await api.createPost({
        title: '',
        body: this.answerBody,
        question_post_id: this.postId,
      });

      if (res.status === 200) {
        this.resetKey = Math.random();

        await this.syncAnswers();
      }
    }

    private syncAnswerBody(body: string): void {
      this.answerBody = body;
    }
  }
</script>

<style lang="scss" scoped>
  .question-container {
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    padding: 20px;
  }

  .question-top {
    display: flex;
    justify-content: left;
    margin: 10px;
  }

  .question-title {
    flex: 1;
    font-size: 26px;
    font-weight: 500;
    text-align: left;
  }

  .question-answers {
    margin: 10px;
    text-align: left;
    font-size: 22px;
    font-weight: 600;
  }

  .question-answer-header {
    font-size: 18px;
    font-weight: 600;
    text-align: left;
    margin: 20px 0;
    padding-left: 5px;
  }

  .question-answer-footer {
    text-align: right;
    margin: 20px 0;
  }
</style>
