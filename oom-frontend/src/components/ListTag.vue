<template>
  <div class="list-tag-container">
    <div class="list-tag-name">
      <div class="tag"><Tag :tag="tag" :actionType="'link'"/></div>
      <div class="count">{{ tag.count_posts }}</div>

      <div class="delete"
           @click="deleteUserLink(tag.id)"
           v-if="userTagIds.includes(tag.id) && isLogin">
        <fa icon="minus-circle"/>
      </div>

      <div class="create"
           @click="createUserLink(tag.id)"
           v-if="!userTagIds.includes(tag.id) && isLogin">
        <fa icon="plus-circle"/>
      </div>
    </div>

    <div class="list-tag-description">
      {{ tag.description }}
    </div>
  </div>
</template>

<script lang="ts">
  import { Component, Prop } from 'vue-property-decorator';
  import Oom from '@/components/Oom.vue';
  import Tag from '@/components/Tag.vue';
  import Api from '@/modules/api';

  @Component({
    components: {
      Tag,
    },
  })
  export default class ListTag extends Oom {
    @Prop() private userTagIds!: number[];
    @Prop() private tag: any;

    private async createUserLink(tag_id: number): Promise<void> {
      const api = new Api(this.$cookies);
      const res = await api.createUserTag(tag_id);

      if (res.status === 200) {
        this.$emit('sync');
      }
    }

    private async deleteUserLink(tag_id: number): Promise<void> {
      const api = new Api(this.$cookies);
      const res = await api.deleteUserTag(tag_id);

      if (res.status === 200) {
        this.$emit('sync');
      }
    }
  }
</script>

<style lang="scss" scoped>
  .list-tag-container {
    width: 150px;
    padding: 15px;
    background-color: #f5f5f5;
    margin-right: 5px;
  }

  .list-tag-name {
    display: flex;
    flex-direction: row;
    text-align: left;
    font-size: 12px;
    margin-bottom: 5px;
    height: 20px;
    line-height: 20px;

    .tag {
    }

    .count {
      flex: 1;
      text-align: left;
    }

    .delete {
      color: #ff5555;
      cursor: pointer;
    }

    .create {
      color: #33DD33;
      cursor: pointer;
    }
  }

  .list-tag-description {
    text-align: left;
    font-size: 12px;
  }
</style>
