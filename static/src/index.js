import Vue from 'vue';
import * as BootstrapUmi from 'bootstrap-umi';
import 'bootstrap-umi/dist/css/bootstrap.css';

Vue.use(BootstrapUmi);

const app = new Vue({
    el: ".app",
    data: function() {
        return {
            text: "Hello Iron & Vue.js"
        }
    }
})