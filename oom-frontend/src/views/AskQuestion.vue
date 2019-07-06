<template>
  <div class="ask-question-container" v-if="isLogin">

    <div class="ask-question-label">Title</div>
    <div class="ask-question-title">
      <el-input v-model="title" />
    </div>

    <div class="ask-question-body">
      <PostBodyForm @syncBody='syncBody' :resetKey="0"/>
    </div>

    <div class="ask-question-label">
      <div class="ask-question-label-tag">
        Tags
      </div>

      <div class="ask-question-label-tags">
        <template v-for="tag in tags">
          <Tag :tag="tag" @tagEvent="removeTag" :actionType="'event'"/>
        </template>
      </div>
    </div>
    <div class="ask-question-tag">
      <el-input v-model="searchTag" />
    </div>

    <div class="ask-question-tags">
      <Loading v-if="searchTagsLoading" :size="'sm'"/>

      <div v-else-if="searchTags.length === 0 && searchTag.length > 0"
           class="ask-question-tags-not-found">
        No tag found for '{{ searchTag }}'
      </div>

      <div class="ask-question-tags-tag" v-for="tag in searchTags" v-bind:key="tag.id">
        <Tag :tag="tag" @tagEvent="addTag" :actionType="'event'"/>
      </div>
    </div>

    <div class="ask-question-post">
      <el-button type="primary" round @click="createQuestion">
        Post Question
      </el-button>
    </div>
  </div>

  <Login :doWhat="'create a question'" v-else/>
</template>

<script lang="ts">
  import { Component, Watch } from 'vue-property-decorator';
  import Oom from '@/components/Oom.vue';
  import { AxiosResponse } from 'axios';
  import PostBodyForm from '@/components/PostBodyForm.vue';
  import Login from '@/components/Login.vue';
  import Tag from '@/components/Tag.vue';
  import Loading from '@/components/Loading.vue';
  import Api from '@/modules/api';

  @Component({
    components: {
      PostBodyForm,
      Login,
      Tag,
      Loading,
    },
  })
  export default class AskQuestion extends Oom {
    private title: string = '';
    private body: string = '';
    private searchTagsLoading: boolean = false;
    private searchTags: any[] = [];
    private searchTag: string = '';
    private searchStack: number[] = [];
    private tags: any[] = [];

    private async createQuestion(): Promise<void> {
      const api = new Api(this.$cookies);
      const res = await api.createPost({
        title: this.title,
        body: this.body,
        question_post_id: null
      });

      if (res.status === 200) {
        await this.createTags(res.data.id);

        this.$router.push({
          name: 'questions',
          params: {
            id: res.data.id,
          },
        });
      }
    }

    private async createTags(postId: number): Promise<AxiosResponse[]> {
      const res = await Promise.all(this.tags.map(async (t) => {
        const api = new Api(this.$cookies);
        const res = await api.createPostTag(postId, t.id);

        return res;
      }));

      return res;
    }

    private syncBody(body: string): void {
      this.body = body;
    }

    @Watch('searchTag', { immediate: false })
    private fetchTags(): void {
      this.searchStack.push(0);

      if (this.searchTag.length > 0) {
        this.searchTagsLoading = true;
      }

      setTimeout(async () => {
        this.searchStack.pop();

        if (this.searchStack.length === 0) {
          if (this.searchTag.length > 0) {
            const api = new Api(this.$cookies);
            const res = await api.searchTags({ q: this.searchTag });

            this.searchTags = res.data;
          } else {
            this.searchTags = [];
          }

          this.searchTagsLoading = false;
        }
      }, 500);
    }

    @Watch('tagId', { immediate: true })
    private async fetchTag(): Promise<void> {
      if (this.tagId) {
        const api = new Api(this.$cookies);
        const res = await api.fetchTag(this.tagId);

        this.tags.push(res.data);
      }
    }

    private addTag(tagId: number): void {
      const addTag = this.searchTags.filter((t) => t.id === tagId);
      const existTag = this.tags.filter((t) => t.id === tagId);

      if (addTag.length === 1 && existTag.length === 0) {
        this.tags.push(addTag[0]);
        this.searchTags = [];
        this.searchTag = '';
      }
    }

    private removeTag(tagId: string): void {
      const removeTagIndex = this.tags.map((t) => t.id).indexOf(tagId);

      if (removeTagIndex >= 0) {
        this.tags.splice(removeTagIndex, 1);
      }
    }

    private get tagId(): number | null {
      if (this.$route.query.tagId) {
        return Number(this.$route.query.tagId);
      }

      return null;
    }
  }
</script>

<style lang="scss" scoped>
  .ask-question-container {
    padding: 30px 100px;
    text-align: left;
  }

  .ask-question-label {
    display: flex;
    flex-direction: row;
    font-size: 14px;
    font-weight: 600;
    margin: 4px 4px 10px 4px;
  }

  .ask-question-label-tag {
    margin-right: 10px;
  }

  .ask-question-label-tags {
    flex: 1;
    display: flex;
    flex-direction: row;
  }

  .ask-question-body {
    margin: 50px 0 40px 0;
  }

  .ask-question-title {
  }

  .ask-question-tag {
    margin-bottom: 4px;
  }

  .ask-question-tags {
    min-height: 30px;
    display: flex;
    flex-direction: row;
  }

  .ask-question-tags-tag {
  }

  .ask-question-tags-not-found {
    font-size: 12px;
    font-weight: 600;
  }

  .ask-question-post {
    width: 100%;
    text-align: right;
  }
</style>
