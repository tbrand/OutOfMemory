import Vue from 'vue';

declare module 'vue/types/vue' {
    interface Vue {
        $cookies: any;
        $setCookie: any;
        $removeCookie: any;
    }
}
