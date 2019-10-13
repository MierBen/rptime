import { AUTH_REQUEST, AUTH_ERROR, AUTH_SUCCESS, AUTH_LOGOUT } from '../actions/auth'
import axios from 'axios'
import VueCookies from 'vue-cookies'

const state = { token: VueCookies.get('token') || '', status: '', hasLoadedOnce: false }

const getters = {
    isAuthenticated: state => !!state.token,
    authStatus: state => state.status,
}

const actions = {
    [AUTH_REQUEST]: ({commit, dispatch}, user) => {
        return new Promise((resolve, reject) => {
            commit(AUTH_REQUEST)
            axios
                .post('api/v1/login', user, {
                    headers: {
                        'Content-Type': 'application/json',
                    }
                })
                .then(resp => {
                    $cookies.set('token', resp.data.token)
                    axios.defaults.headers.common['Authorization'] = resp.data.token
                    commit(AUTH_SUCCESS, resp)
                    //dispatch(USER_REQUEST)
                    resolve(resp)
                })
                .catch(err => {
                    commit(AUTH_ERROR, err)
                    $cookies.remove('token')
                    reject(err)
                })
            })
        },
        [AUTH_LOGOUT]: ({commit, dispatch}) => {
            return new Promise((resolve, reject) => {
                commit(AUTH_LOGOUT)
                $cookies.remove('token')
                resolve()
        })
      }
}

const mutations = {
    [AUTH_REQUEST]: (state) => {
        state.status = 'loading'
    },
    [AUTH_SUCCESS]: (state, resp) => {
        state.status = 'success'
        state.token = resp.data.token
        state.hasLoadedOnce = true
    },
    [AUTH_ERROR]: (state) => {
        state.status = 'error'
        state.hasLoadedOnce = true
    },
    [AUTH_LOGOUT]: (state) => {
        state.token = ''
  }
}

export default {
    state,
    getters,
    actions,
    mutations,
}