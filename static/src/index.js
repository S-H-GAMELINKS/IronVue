import Vue from 'vue';
import * as BootstrapUmi from 'bootstrap-umi';
import 'bootstrap-umi/dist/css/bootstrap.css';

import Header from '../components/layouts/Header.vue';

import Router from '../router/router';

Vue.use(BootstrapUmi);

const app = new Vue({
    el: ".app",
    router: Router,
    components: {
        'nav-bar': Header
    },
    data: function() {
        return {
            text: "Hello Iron & Vue.js"
        }
    }
})