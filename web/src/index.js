// Import Vue
import Vue from 'vue';
import VueRouter from 'vue-router';

// Import Vue App, routes, store
import App from './App';
import routes from './routes';

import mavonEditor from 'mavon-editor'
import 'mavon-editor/dist/css/index.css'

Vue.use(VueRouter);
Vue.use(mavonEditor)

import md from "./plugins/markdown";

Vue.prototype.$md = md;

// Configure router
const router = new VueRouter({
    routes,
    linkActiveClass: 'active',
    mode: 'history'
});

new Vue({
    el: '#app',
    render: h => h(App),
    router
});
