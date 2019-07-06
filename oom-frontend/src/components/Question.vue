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
  export default class Question extends Oom {
    @Prop() private questionInner: any;
    @Prop() private questionUserId!: number;

    private postAsRef: PostAsRef = {
      id: 0,
      body: '',
      ups: 0,
      downs: 0,
      userId: 0,
      questionUserId: this.questionUserId,
      isQuestion: true,
      isBestAnswer: false,
    };

    @Watch('questionInner', { immediate: true })
    private sync(): void {
      this.postAsRef = {
        id: this.questionInner.id,
        body: this.questionInner.body,
        ups: this.questionInner.count_ups,
        downs: this.questionInner.count_downs,
        userId: this.questionInner.user_id,
        questionUserId: this.questionUserId,
        isQuestion: true,
        isBestAnswer: false,
      };
    }
  }
</script>
