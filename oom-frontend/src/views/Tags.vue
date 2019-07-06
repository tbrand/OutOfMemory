<template>
  <div class="tags-container">

    <div class="tags-header">
      <div class="tags-header-search">
        <el-input placeholder="Search" v-model="searchInput" class="tags-header-search-form">
          <el-button slot="append" icon="el-icon-search"></el-button>
        </el-input>
      </div>

      <div class="tags-header-sort">
        <el-radio-group v-model="searchOrder" size="small">
          <el-radio-button label="alphabet">Alphabet</el-radio-button>
          <el-radio-button label="popular">Popular</el-radio-button>
        </el-radio-group>
      </div>

      <div class="tags-header-create">
        <el-button type="primary" round @click="create">
          Create
        </el-button>
      </div>
    </div>

    <template v-if="isLogin">
      <div class="tags-title">
        your tags
      </div>

      <div class="tags-list">
        <ListTag :tag="tag"
                 :userTagIds="userTagIds"
                 @sync="fetchUserTags"
                 v-for="tag in userTags"
                 v-bind:key="tag.id"/>
      </div>

      <el-divider/>
    </template>

    <div class="tags-title">
      all
    </div>

    <div class="tags-list">
      <ListTag :tag="tag"
               :userTagIds="userTagIds"
               @sync="fetchUserTags"
               v-for="tag in tags"
               v-bind:key="tag.id"/>
    </div>
  </div>
</template>

<script lang="ts">
  import { Component, Watch } from 'vue-property-decorator';
  import Oom from '@/components/Oom.vue';
  import ListTag from '@/components/ListTag.vue';
  import Api from '@/modules/api';

  @Component({
    components: {
      ListTag,
    },
  })
  export default class Tags extends Oom {
    private page: number = 0;
    private searchInput: string = '';
    private searchOrder: string = 'alphabet';
    private tags: any[] = [];
    private userTags: any[] = [];

    private create(): void {
      this.$router.push({ name: 'createTag'});
    }

    @Watch('page', { immediate: true })
    @Watch('searchInput')
    @Watch('searchOrder')
    private async fetchTags(): Promise<void> {
      const api = new Api(this.$cookies);
      const res = await api.fetchTags({ page: this.page });

      this.tags = res.data;
    }

    @Watch('isLogin', { immediate: true })
    private async fetchUserTags(): Promise<void> {
      if (this.isLogin) {
        const api = new Api(this.$cookies);
        const res = await api.fetchUserTags(this.authCookie.userId);

        this.userTags = res.data;
      }
    }

    private get userTagIds(): number[] {
      return this.userTags.map((u) => u.id);
    }
  }
</script>

<style lang="scss" scoped>
  .tags-container {
    padding: 50px;
  }

  .tags-header {
    display: flex;
    align-items: center;
    margin-bottom: 30px;
  }

  .tags-header-search {
    padding: 0 20px 0 0;
  }

  .tags-header-search-form {
    width: 200px;
  }

  .tags-header-sort {
    flex: 1;
    text-align: left;
  }

  .tags-title {
    text-align: left;
    font-size: 16px;
    font-weight: 600;
    margin: 20px 0 15px 0;
  }

  .tags-list {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    margin-bottom: 10px;
  }
</style>
