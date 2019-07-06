<script lang="ts">
  import { Component, Vue } from 'vue-property-decorator';
  import parseCookie, { OomCookie } from '@/modules/cookie';

  @Component
  export default class Oom extends Vue {
    public get authCookie(): OomCookie {
      return parseCookie(this.$cookies);
    }

    public get isLogin(): boolean {
      return !!this.$cookies.oom;
    }
  }
</script>
