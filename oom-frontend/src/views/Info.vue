<template>
  <div class="info-container">
    <Ranking :title="'Best Answers'"
             :sort="'best_answers'"
             :description="'A ranking of number of best answers within a week.'"/>
    <Ranking :title="'Answers'"
             :sort="'answers'"
             :description="'A ranking of number of answers within a week.'"/>
  </div>
</template>

<script lang="ts">
  import { Component } from 'vue-property-decorator';
  import Oom from '@/components/Oom.vue';
  import Ranking from '@/components/Ranking.vue';

  @Component({
    components: {
      Ranking,
    },
  })
  export default class Info extends Oom { }
</script>

<style lang="scss">
  .info-container {
    padding: 10px 40px 10px 0;
    font-size: 12px;
    color: #3B3B3B;
    text-align: left;

    .info-ranking {
      background-color: #FFF8DC;
      border: 1px solid #e6e6e6;
      border-radius: 5px;
      margin: 10px 10px 20px 10px;

      .info-ranking-header {
        padding: 8px;
        font-size: 16px;
        font-weight: 600;
        background-color: #ede7cd;
        margin-bottom: 4px;

        .fa-crown {
          font-size: 14px;
        }

        .info-ranking-header-description {
          padding-left: 4px;
          margin-top: 4px;
          font-size: 10px;
          font-weight: 500;
        }
      }

      .info-ranking-rows {
        margin: 0 !important;
        padding: 0 6px 6px 6px !important;

        .info-ranking-row {
          font-size: 12px;
          padding: 6px;
          list-style-type: none;
        }
      }

      .info-ranking-footer {
        text-align: right;
        padding: 0 10px 10px 0;
      }
    }
  }
</style>
