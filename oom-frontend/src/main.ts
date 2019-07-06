import Vue from 'vue';
import Element, { Button } from 'element-ui';
import VueReactiveCookie from 'vue-reactive-cookie';
import 'element-ui/lib/theme-chalk/index.css';
import { library } from '@fortawesome/fontawesome-svg-core';
import {
    faHome,
    faGlobe,
    faUsers,
    faTags,
    faCrown,
    faCaretUp,
    faCaretDown,
    faCog,
    faUser,
    faSignOutAlt,
    faSignInAlt,
    faMinusCircle,
    faPlusCircle,
    faCheck,
} from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
import '@/vendor/google-code-prettify/prettify';
import App from './App.vue';
import router from './router';
import store from './store';

library.add(
    faHome,
    faGlobe,
    faUsers,
    faTags,
    faCrown,
    faCaretUp,
    faCaretDown,
    faCog,
    faUser,
    faSignOutAlt,
    faSignInAlt,
    faMinusCircle,
    faPlusCircle,
    faCheck,
);

Vue.config.productionTip = false;

Vue.use(Element);
Vue.use(VueReactiveCookie);

Vue.component('fa', FontAwesomeIcon);

new Vue({
    router,
    store,
    render: h => h(App),
}).$mount('#app');
