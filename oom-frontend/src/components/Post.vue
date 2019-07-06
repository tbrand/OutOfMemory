<template>
  <div class="post-container">
    <div class="post-ups">
      <div :class="upClass" @click="up(true)"><fa icon="caret-up"/></div>
      <div class="post-ups-num">{{ postAsRef.ups - postAsRef.downs }}</div>
      <div :class="downClass" @click="up(false)"><fa icon="caret-down"/></div>
      <div :class="bestAnswerClass" @click="updateBestAnswer" v-if="!postAsRef.isQuestion"><fa icon="check"/></div>
    </div>

    <div class="post-main">
      <div class="post-body">
        <RenderMarkdown :body="postAsRef.body"/>
      </div>

      <div class="post-tags">
        <Tag :tag="tag" :actionType="'link'" v-for="tag in tags" v-bind:key="tag.id"/>
      </div>

      <div class="post-menu">
        <div class="post-menu-links">
          <a class="post-menu-link" href="/">share</a>
          <a class="post-menu-link" href="/">edit</a>
        </div>

        <div class="post-menu-profile" v-if="user">
          <div class="post-menu-profile-image">
            <img :src="user.avatar_url" width="30px" height="30px"/>
          </div>
          <div class="post-menu-profile-user">
            <div class="post-menu-profile-name">
              <router-link :to="userLink">
                {{ user.name }}
              </router-link>
            </div>
            <div class="post-menu-profile-nums">
              <div class="post-menu-profile-num">
                B (<span class="num">{{ user.count_best_answers }}</span>),
              </div>
              <div class="post-menu-profile-num">
                A (<span class="num">{{ user.count_answers }}</span>),
              </div>
              <div class="post-menu-profile-num">
                Q (<span class="num">{{ user.count_questions }}</span>)
              </div>
            </div>
          </div>
        </div>
      </div>

      <el-divider />

      <template v-for="comment in comments">
        <Comment :comment="comment" v-bind:key="comment.id"/>
        <el-divider v-bind:key="comment.id"/>
      </template>

      <div class="post-add-comment-container">

        <template v-if="commentFormExpanded">
          <el-input type="textarea" rows="3" v-model="comment" v-if="isLogin"/>
          <Login :doWhat="'post a comment'" v-else/>

          <div class="post-add-comment">
            <span @click="switchCommentForm" class="action">cancel</span>
            <span @click="createComment" class="action" v-if="isLogin">add</span>
          </div>
        </template>

        <template v-else>
          <div class="post-add-comment">
            <span @click="switchCommentForm" class="action">add a comment</span>
          </div>
        </template>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
  import { Component, Prop, Watch } from 'vue-property-decorator';
  import Oom from '@/components/Oom.vue';
  import Tag from '@/components/Tag.vue'
  import Comment from '@/components/Comment.vue';
  import RenderMarkdown from '@/components/RenderMarkdown.vue';
  import Login from '@/components/Login.vue';
  import Api from '@/modules/api';

  export type PostAsRef = {
    id: number,
    body: string,
    ups: number,
    downs: number,
    userId: number,
    questionUserId: number,
    isQuestion: boolean,
    isBestAnswer: boolean,
  }

  @Component({
    components: {
      Tag,
      Comment,
      RenderMarkdown,
      Login,
    },
  })
  export default class Post extends Oom {
    @Prop() private postAsRef!: PostAsRef;

    private comment: string = '';
    private comments: any[] = [];
    private commentFormExpanded: boolean = false;
    private user: any | null = null;
    private tags: any[] = [];
    private isUp: boolean | null = null;

    @Watch('postAsRef', { immediate: true })
    private async syncComments(): Promise<void> {
      const api = new Api(this.$cookies);
      const res = await api.fetchComments(this.postAsRef.id);

      this.comments = res.data;
    }

    @Watch('postAsRef', { immediate: true })
    private async syncUser(): Promise<void> {
      const api = new Api(this.$cookies);
      const res = await api.fetchUser(this.postAsRef.userId);

      this.user = res.data;
    }

    @Watch('postAsRef', { immediate: true })
    private async syncTags(): Promise<void> {
      const api = new Api(this.$cookies);
      const res = await api.fetchPostTags(this.postAsRef.id);

      this.tags = res.data;
    }

    @Watch('postAsRef', { immediate: true })
    @Watch('isLogin')
    private async syncIsUp(): Promise<void> {
      if (this.isLogin) {
        const api = new Api(this.$cookies);
        const res = await api.isUp(this.postAsRef.id);

        this.isUp = res.data.is_up;
      }
    }

    private async syncUps(): Promise<void> {
      if (this.postAsRef.isQuestion) {
        await this.fetchQuestion();
      } else {
        await this.fetchAnswer();
      }

      await this.syncIsUp();
    }

    private async fetchQuestion(): Promise<void> {
      const api = new Api(this.$cookies);
      const res = await api.fetchQuestion(this.postAsRef.id);

      this.postAsRef.ups = res.data.count_ups;
      this.postAsRef.downs = res.data.count_downs;
      this.postAsRef.isBestAnswer = false;
    }

    private async fetchAnswer(): Promise<void> {
      const api = new Api(this.$cookies);
      const res = await api.fetchAnswer(this.postAsRef.id);

      this.postAsRef.ups = res.data.count_ups;
      this.postAsRef.downs = res.data.count_downs;
      this.postAsRef.isBestAnswer = res.data.is_best_answer;
    }

    private async up(isUp: boolean): Promise<void> {
      if (this.isLogin) {
        const api = new Api(this.$cookies);
        const res = await api.upsertUps(this.postAsRef.id, { is_up: isUp });

        await this.syncUps();
      }
    }

    private async updateBestAnswer(): Promise<void> {
      if (this.isLogin && this.authCookie.userId === this.postAsRef.questionUserId) {
        const api = new Api(this.$cookies);
        const res = await api.updateBestAnswer(this.postAsRef.id, { is_best_answer: !this.postAsRef.isBestAnswer })

        await this.syncUps();
      }
    }

    private get upClass(): string {
      if (this.isUp === true) {
        return 'post-ups-up do';
      }

      return 'post-ups-up';
    }

    private get downClass(): string {
      if (this.isUp === false) {
        return 'post-ups-up do';
      }

      return 'post-ups-up';
    }

    private get bestAnswerClass(): string {
      return this.postAsRef.isBestAnswer ? 'post-best-answer do' : 'post-best-answer';
    }

    private get userLink(): any {
      return {
        name: 'user',
        params: {
          id: this.user!.id,
        },
      };
    }

    private switchCommentForm(): void {
      this.commentFormExpanded = !this.commentFormExpanded;
    }

    private async createComment(): Promise<void> {
      const api = new Api(this.$cookies);
      const res = await api.createComment(this.postAsRef.id, { body: this.comment });

      if (res.status === 201) {
        await this.syncComments();

        this.commentFormExpanded = false;
        this.comment = '';
      }
    }
  }
</script>

<style lang="scss" scoped>
  .post-container {
    display: flex;
    justify-content: left;
  }

  .post-ups {
    padding: 24px 0;
    width: 30px;
  }

  .post-ups-num {
    font-size: 20px;
  }

  .post-ups-up {
    color: #e3e3e3;
    font-size: 40px;
    cursor: pointer;
  }

  .post-ups-up.do {
    color: #409EFF;
  }

  .post-best-answer {
    font-size: 24px;
    color: #e3e3e3;
    cursor: pointer;    
  }

  .post-best-answer.do {
    font-size: 24px;
    color: #409EFF;
    cursor: pointer;
  }

  .post-main {
    flex: 1;
    padding: 20px;
    font-size: 14px;
    text-align: left;
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
  }

  .post-body {
    width: 100%;
    word-break : break-all;
  }

  .post-tags {
    margin-top: 20px;
    text-align: left;
    display: flex;
    justify-content: flex-start;
  }

  .post-menu {
    margin: 20px 0;
    display: flex;
  }

  .post-menu-links {
    flex: 1 1 100px;
  }

  .post-menu-link {
    margin-right: 10px;
    font-size: 12px;
  }

  .post-menu-profile {
    background-color: #E1ECF4;
    width: 150px;
    display: flex;
    justify-content: flex-start;
    padding: 6px;
  }

  .post-menu-profile-image {
    width: 30px;
    height: 30px;
    margin-right: 10px;
  }

  .post-menu-profile-user {
    flex: 1;
    display: flex;
    justify-content: flex-start;
    flex-direction: column;
  }

  .post-menu-profile-name {
    font-size: 14px;
  }

  .post-menu-profile-nums {
    display: flex;
    justify-content: flex-start;
  }

  .post-menu-profile-num {
    font-size: 12px;
    margin-right: 3px;

    .num {
      font-weight: 600;
    }
  }

  .post-add-comment-container {
    margin-top: 10px;
  }

  .post-add-comment {
    font-size: 14px;
    margin-top: 8px;
    cursor: pointer;
    text-align: right;

    .action {
      margin-left: 10px;
    }
  }
</style>
