import Vue from 'vue'
import Router from 'vue-router'

import Register from '../views/Register.vue'
import Login from '../views/Login.vue'
import About from '../views/About.vue'
import TaskMap from '../views/Map.vue'

Vue.use(Router)


export default new Router({
    mode: 'history',
    routes: [
        {
            path: '/about',
            name: 'About',
            component: About,
            children: [
                { 
                    path: '/register', 
                    component: Register,
                },
                { 
                    path: '/login', 
                    component: Login,
                }
            ]
        },
        {
            path: '/',
            name: 'Map',
            component: TaskMap,
        },
        
    ]
})