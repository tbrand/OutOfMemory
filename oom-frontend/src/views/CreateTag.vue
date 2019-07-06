<template>
  <div class="create-tag-container" v-if="isLogin">
    <div class="create-tag-label">name</div>
    <div class="create-tag-title">
      <el-input v-model="name" />
    </div>

    <div class="create-tag-label">description</div>
    <div class="create-tag-description">
      <el-input type="textarea" rows="3" v-model="description"/>
    </div>

    <div class="create-tag-create">
      <el-button type="primary" round @click="create">
        Create
      </el-button>
    </div>
  </div>

  <Login :doWhat="'create a tag'" v-else/>
</template>

<script lang="ts">
  import { Component } from 'vue-property-decorator';
  import Oom from '@/components/Oom.vue';
  import Login from '@/components/Login.vue';
  import Api from '@/modules/api';

  @Component({
    components: {
      Login,
    },
  })
  export default class CreateTag extends Oom {
    private name: string = '';
    private description: string = '';

    private async create(): Promise<void> {
      const api = new Api(this.$cookies);
      const res = await api.createTag({ name: this.name, description: this.description });

      if (res.status === 201) {
        this.$router.push({ name: 'tags' });
      }
    }
  }
</script>

<style lang="scss" scoped>
  .create-tag-container {
    padding: 30px 100px;
    text-align: left;    
  }

  .create-tag-label {
    font-size: 14px;
    font-weight: 600;
    margin: 4px;
  }

  .create-tag-title {
    
  }

  .create-tag-create {
    margin-top: 20px;
    text-align: right;
  }
</style>
