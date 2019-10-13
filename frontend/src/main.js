import Vue from 'vue'
import BootstrapVue from 'bootstrap-vue'

import App from './App.vue'

import router from './router'


import 'bootstrap/dist/css/bootstrap.css'
import 'bootstrap-vue/dist/bootstrap-vue.css'

Vue.use(BootstrapVue)


new Vue({
    el: '#app',
    router,
    render: h => h(App)
})
