<template>
  <div class="ranking-container">
    <div class="ranking-header">
      <fa icon="crown"/> {{ title }}

      <div class="ranking-header-description">
        {{ description }}
      </div>
    </div>

    <ul class="ranking-rows">
      <li class="ranking-row" v-for="(user, rank) in users">
        {{ rank + 1 }}. <router-link :to="{ name: 'user', params: { id: user.id } }">
        {{ user.name }}
        </router-link>

        <span>B ({{ user.count_best_answers }})</span>,
        <span>A ({{ user.count_answers }})</span>,
        <span>Q ({{ user.count_questions }})</span>,
      </li>
    </ul>
  </div>
</template>

<script lang="ts">
  import { Component, Prop, Watch } from 'vue-property-decorator';
  import Oom from '@/components/Oom.vue';
  import Api from '@/modules/api';

  @Component
  export default class Ranking extends Oom {
    @Prop() private sort!: string;
    @Prop() private title!: string;
    @Prop() private description!: string;

    private users: any[] = [];

    @Watch('sort', { immediate: true })
    private async fetchUsers(): Promise<void> {
      const api = new Api(this.$cookies);
      const res = await api.fetchUsers({ sort: this.sort });

      this.users = res.data;
    }
  }
</script>

<style lang="scss" scoped>
  .ranking-container {
    background-color: #84DFFF;
    border: 1px solid #e6e6e6;
    color: #ffffff;
    border-radius: 5px;
    margin: 10px 10px 20px 10px;

    .ranking-header {
      padding: 8px;
      font-size: 16px;
      font-weight: 600;
      background-color: #409EFF;
      margin-bottom: 4px;
      border-radius: 5px 5px 0 0;

      .fa-crown {
        font-size: 14px;
      }

      .ranking-header-description {
        padding-left: 4px;
        margin-top: 4px;
        font-size: 10px;
        font-weight: 500;
      }
    }

    .ranking-rows {
      color: #000000;
      margin: 0 !important;
      padding: 0 6px 6px 6px !important;

      .ranking-row {
        font-size: 12px;
        padding: 6px;
        list-style-type: none;

        span {
          margin-left: 2px;
        }
      }
    }
  }
</style>
