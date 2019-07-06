<template>
  <Post :postAsRef="postAsRef"/>
</template>

<script lang="ts">
  import { Component, Prop, Watch } from 'vue-property-decorator';
  import Oom from '@/components/Oom.vue';
  import Post, { PostAsRef } from '@/components/Post.vue';

  @Component({
    components: {
      Post,
    },
  })
  export default class Answer extends Oom {
    @Prop() private answerInner: any;
    @Prop() private questionUserId!: number;

    private postAsRef: PostAsRef = {
      id: 0,
      body: '',
      ups: 0,
      downs: 0,
      userId: 0,
      questionUserId: this.questionUserId,
      isQuestion: false,
      isBestAnswer: false,
    };

    @Watch('answerInner', { immediate: true })
    private sync(): void {
      this.postAsRef = {
        id: this.answerInner.id,
        body: this.answerInner.body,
        ups: this.answerInner.count_ups,
        downs: this.answerInner.count_downs,
        userId: this.answerInner.user_id,
        questionUserId: this.questionUserId,
        isQuestion: false,
        isBestAnswer: this.answerInner.is_best_answer,
      };
    }
  }
</script>
