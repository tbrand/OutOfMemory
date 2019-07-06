<template>
  <div class="comment-container">
    {{ comment.body }}

    <router-link :to="userLink" class="comment-user" v-if="user">
      {{ user.name }}
    </router-link>

    <span class="comment-date">2 days ago</span>
  </div>
</template>

<script lang="ts">
  import { Component, Prop, Watch } from 'vue-property-decorator';
  import Oom from '@/components/Oom.vue';
  import Api from '@/modules/api';

  @Component
  export default class Comment extends Oom {
    @Prop() private comment: any;

    private user: any | null = null;

    @Watch('comment', { immediate: true })
    private async syncUser(): Promise<void> {
      const api = new Api(this.$cookies);
      const res = await api.fetchUser(this.comment.user_id);

      this.user = res.data;
    }

    private get userLink(): any {
      return {
        name: 'user',
        params: {
          id: this.user!.id,
        },
      };
    }
  }
</script>

<style lang="scss" scoped>
  .comment-container {
    padding: 8px 0;
    font-size: 14px;
  }

  .comment-user {
    margin-left: 10px;
    font-size: 14px;
  }

  .comment-date {
    margin-left: 10px;
    font-size: 14px;
  }
</style>
