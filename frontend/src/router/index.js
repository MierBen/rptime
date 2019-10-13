import Vue from 'vue'
import Router from 'vue-router'

import Register from '../views/Register.vue'
import Login from '../views/Login.vue'
import About from '../views/About.vue'
import TaskMap from '../views/Map.vue'

import store from '../store'

Vue.use(Router)

const ifNotAuthenticated = (to, from, next) => {
    if (!store.getters.isAuthenticated) {
        next()
        return
    }
    next('/map')
}

const ifAuthenticated = (to, from, next) => {
    if (store.getters.isAuthenticated) {
        next()
        return
    }
    next('/')
}

export default new Router({
    mode: 'history',
    routes: [
        {
            path: '/',
            name: 'About',
            component: About,
            beforeEnter: ifNotAuthenticated,
        },
        {
            path: '/map',
            name: 'Map',
            component: TaskMap,
            beforeEnter: ifAuthenticated,
        },
        { 
            path: '/register', 
            component: Register,
            beforeEnter: ifNotAuthenticated,
        },
        { 
            path: '/login', 
            component: Login,
            beforeEnter: ifNotAuthenticated,
        }
    ]
})