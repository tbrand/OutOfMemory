<template>
  <div class="user-container">
    <div class="user-profile" v-if="user">
      <div class="user-profile-image">
        <img :src="user.avatar_url" width="140px" height="140px"/>
      </div>

      <div class="user-profile-all">
        <div class="user-profile-data">
          <div class="user-profile-basic">
            <div class="user-profile-name">
              {{ user.name }}
            </div>

            <div class="user-profile-created">
              Created 5 months ago
            </div>
          </div>

          <div class="user-profile-analytics">
            <div class="user-profile-analytics-category">
              <div class="user-profile-analytics-num">
                {{ user.count_best_answers }}
              </div>

              <div class="user-profile-analytics-title">
                Best Answers
              </div>
            </div>

            <div class="user-profile-analytics-category">
              <div class="user-profile-analytics-num">
                {{ user.count_answers }}
              </div>

              <div class="user-profile-analytics-title">
                Answers
              </div>
            </div>

            <div class="user-profile-analytics-category">
              <div class="user-profile-analytics-num">
                {{ user.count_questions }}
              </div>

              <div class="user-profile-analytics-title">
                Questions
              </div>
            </div>
          </div>
        </div>

        <div class="user-tags">
          <div class="user-tags-title">
            Tags
          </div>

          <div class="user-tags-list">
            <Tag :tag="tag" v-for="tag in tags" v-bind:key="tag.id" :actionType="'link'"/>

            <div class="edit" v-if="isMe">
              <router-link :to="{ name: 'tags' }"><fa icon="cog"/></router-link>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="user-activities" v-if="user">
      <div class="user-activities-content">
        <div class="user-activities-title">
          Questions
        </div>

        <div class="user-activities-divider">
          <el-divider/>
        </div>

        <QuestionRowShort :question="question"
                          v-for="(question, idx) in questions"
                          v-bind:key="'q-' + idx"/>
      </div>

      <div class="user-activities-content">
        <div class="user-activities-title">
          Answers
        </div>

        <div class="user-activities-divider">
          <el-divider/>
        </div>

        <QuestionRowShort :question="question"
                          v-for="(question, idx) in answersQuestions"
                          v-bind:key="'a-' + idx"/>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
  import { Component, Watch } from 'vue-property-decorator';
  import Oom from '@/components/Oom.vue';
  import QuestionRowShort from '@/components/QuestionRowShort.vue';
  import Tag from '@/components/Tag.vue';
  import Api from '@/modules/api';

  @Component({
    components: {
      Tag,
      QuestionRowShort,
    },
  })
  export default class User extends Oom {
    private activityOrder: string = "latest";
    private user: any | null = null;
    private tags: any[] = [];
    private questions: any[] = [];
    private answersQuestions: any[] = [];

    private get userId(): number {
      return Number(this.$route.params.id);
    }

    @Watch('userId', { immediate: true })
    private async syncUser(): Promise<void> {
      const api = new Api(this.$cookies);
      const res = await api.fetchUser(this.userId);

      this.user = res.data;
    }

    @Watch('userId', { immediate: true })
    private async syncTags(): Promise<void> {
      const api = new Api(this.$cookies);
      const res = await api.fetchUserTags(this.userId);

      this.tags = res.data;
    }

    @Watch('userId', { immediate: true })
    private async syncQuestions(): Promise<void> {
      const api = new Api(this.$cookies);
      const res = await api.fetchUserQuestions(this.userId, { page: 0 });

      this.questions = res.data;
    }

    @Watch('userId', { immediate: true })
    private async syncAnswersQuestions(): Promise<void> {
      const api = new Api(this.$cookies);
      const res = await api.fetchUserAnswersQuestions(this.userId, { page: 0 });

      this.answersQuestions = res.data;
    }

    private get isMe(): boolean {
      if (!this.$cookies) {
        return false;
      }

      return this.authCookie.userId === this.userId;
    }

    private switchActivity(): void {
    }
  }
</script>

<style lang="scss" scoped>
  .user-container {
    padding: 20px 60px;
  }

  .user-profile {
    display: flex;
    justify-content: flex-start;
  }

  .user-profile-image {
  }

  .user-profile-all {
    flex: 1;
    display: flex;
    justify-content: flex-start;
    flex-direction: column;
    padding: 0 30px;
  }

  .user-profile-data {
    display: flex;
    justify-content: flex-start;
    margin-bottom: 20px;
  }

  .user-profile-basic {
    flex: 1;
  }

  .user-profile-name {
    font-size: 20px;
    font-weight: 600;
    text-align: left;
  }

  .user-profile-created {
    font-size: 12px;
    text-align: left;
  }

  .user-profile-analytics {
    display: flex;
    justify-content: center;
    background-color: #ebebeb;
    border-radius: 5px;
    padding: 10px;
  }

  .user-profile-analytics-category {
    display: flex;
    justify-content: flex-start;
    flex-direction: column;
    padding: 0 10px;
  }

  .user-profile-analytics-num {
    font-size: 20px;
    font-weight: 600;
  }

  .user-profile-analytics-title {
    font-size: 12px;
  }

  .user-tags {
    font-weight: 600;
  }

  .user-tags-title {
    text-align: left;
    margin-bottom: 5px;
    padding-left: 5px;
  }

  .user-tags-list {
    display: flex;
    justify-content: flex-start;
    align-items: center;

    .edit {
      cursor: pointer;
      color: #565656;
      font-size: 14px;
    }
  }

  .user-activities {
    display: flex;
    justify-content: flex-start;
    text-align: left;
    width: 100%;
  }

  .user-activities-content {
    flex: 1;
    padding: 10px;
  }

  .user-activities-title {
    margin: 30px 0 10px 0;
    padding-left: 5px;
    font-weight: 600;
  }

  .user-activities-divider {
    margin: 0 0 20px 0;
    padding: 0 6px;
  }

  .user-activities-order {
    margin: 5px 5px 15px 5px;
  }
</style>
