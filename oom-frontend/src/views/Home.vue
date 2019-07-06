<template>
  <div class="home-container">
    <div class="home-tag" v-if="tag">
      <div class="home-tag-title">
        <div class="home-tag-title-name">{{ tag.name }}</div>
      </div>

      <div class="home-tag-description">
        {{ tag.description }}
      </div>
    </div>

    <div class="home-tags" v-else-if="isLogin">
      <div class="home-tags-list">
        <div class="home-tags-list-title">Your Tags</div>
        <Tag :tag="tag" :actionType="'link'" v-for="tag in tags" v-bind:key="tag.id"/>

        <div class="edit">
          <router-link :to="{ name: 'tags' }"><fa icon="cog"/></router-link>
        </div>
      </div>
    </div>

    <el-divider />

    <div class="home-top">
      <div class="home-options">
        <div class="home-options-order">
          <el-radio-group v-model="searchOrder" size="small">
            <el-radio-button label="latest">Latest</el-radio-button>
            <el-radio-button label="active">Active</el-radio-button>
            <el-radio-button label="hot">Hot</el-radio-button>
            <el-radio-button label="popular">Popular</el-radio-button>
          </el-radio-group>
        </div>

        <div class="home-options-ask">
          <router-link :to="askLink">
            <el-button type="primary" round>
              Ask Question
            </el-button>
          </router-link>
        </div>
      </div>
    </div>

    <template v-for="question in questions">
      <QuestionRow :question="question" v-bind:key="question.id"/>
      <el-divider v-bind:key="question.id"/>
    </template>
  </div>
</template>

<script lang="ts">
  import { Component, Watch } from 'vue-property-decorator';
  import Oom from '@/components/Oom.vue';
  import QuestionRow from '@/components/QuestionRow.vue';
  import Tag from '@/components/Tag.vue';
  import Api from '@/modules/api';

  @Component({
    components: {
      QuestionRow,
      Tag,
    },
  })
  export default class Home extends Oom {
    private page: number = 0;
    private searchOrder: string = 'latest';
    private questions: any[] = [];
    private tags: any[] = [];
    private tag: any | null = null;

    @Watch('$route', { immediate: true, deep: true })
    @Watch('searchOrder')
    private async fetchQuestions(): Promise<void> {
      const api = new Api(this.$cookies);

      if (!this.tagId) {
        const res = await api.fetchQuestions({ page: this.page });
        this.questions = res.data;        
      } else if (this.tagId && this.tag) {
        const res = await api.fetchTagQuestions(this.tag.id);
        this.questions = res.data;        
      } else {
        // tag is specified but it's not feched.
      }
    }

    @Watch('isLogin', { immediate: true })
    private async fetchUserTags(): Promise<void> {
      if (this.isLogin) {
        const api = new Api(this.$cookies);
        const res = await api.fetchUserTags(this.authCookie.userId);

        this.tags = res.data;
      }
    }

    @Watch('tagId', { immediate: true })
    private async fetchTag(): Promise<void> {
      if (this.tagId) {
        const api = new Api(this.$cookies);
        const res = await api.fetchTag(this.tagId);

        this.tag = res.data;
      } else {
        this.tag = null;
      }

      await this.fetchQuestions();
    }

    private get tagId(): number | null {
      if (this.$route.params.id) {
        return Number(this.$route.params.id);
      }

      return null;
    }

    private get askLink(): object {
      if (this.tag) {
        return {
          name: 'ask',
          query: {
            tagId: this.tag.id,
          }
        };
      }

      return { name: 'ask' };
    }
  }
</script>

<style lang="scss" scoped>
  .home-container {
    padding: 10px 10px 0 10px;
  }

  .home-top {
    display: flex;
    justify-content: flex-start;
    flex-direction: column;
  }

  .home-tags {
    padding: 20px 0 0 20px;
    text-align: left;
  }

  .home-tags-title {
    margin-bottom: 4px;
    padding-left: 2px;
  }

  .home-tags-list {
    display: flex;
    flex-direction: row;
    align-items: center;
    margin-bottom: 10px;

    .edit {
      cursor: pointer;
      color: #565656;
      font-size: 14px;
    }
  }

  .home-tags-list-title {
    font-size: 14px;
    font-weight: 600;
    margin-right: 10px;
  }

  .home-tag {
    padding: 20px 0 10px 20px;
    text-align: left;
  }

  .home-tag-title {
    display: flex;
    align-items: center;
    margin-bottom: 6px;
  }

  .home-tag-title-name {
    font-size: 24px;
    font-weight: 600;
  }

  .home-tag-description {
    font-size: 16px;
    margin-bottom: 20px;
  }

  .home-options {
    display: flex;
    justify-content: flex-start;
    align-items: center;
    padding: 20px;
  }

  .home-options-order {
    flex: 1;
    text-align: left;
  }

  .home-options-ask {
    width: 137.5px;
  }
</style>
