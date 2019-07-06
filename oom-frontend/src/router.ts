import Vue from 'vue';
import Router from 'vue-router';
import Home from '@/views/Home.vue';
import Auth from '@/views/Auth.vue';
import Question from '@/views/Question.vue';
import AskQuestion from '@/views/AskQuestion.vue';
import Users from '@/views/Users.vue';
import User from '@/views/User.vue';
import Tags from '@/views/Tags.vue';
import CreateTag from '@/views/CreateTag.vue';

Vue.use(Router);

export default new Router({
    mode: 'history',
    base: process.env.BASE_URL,
    routes: [
        {
            path: '/',
            name: 'home',
            component: Home,
        },
        {
            path: '/auth',
            name: 'auth',
            component: Auth,
        },
        {
            path: '/questions/ask',
            name: 'ask',
            component: AskQuestion,
        },
        {
            path: '/questions/:id',
            name: 'questions',
            component: Question,
        },
        {
            path: '/users',
            name: 'users',
            component: Users,
        },
        {
            path: '/users/:id',
            name: 'user',
            component: User,
        },
        {
            path: '/tags',
            name: 'tags',
            component: Tags,
        },
        {
            path: '/tags/:id',
            name: 'tag',
            component: Home,
        },
        {
            path: '/tags/create',
            name: 'createTag',
            component: CreateTag,
        },
    ],
});
