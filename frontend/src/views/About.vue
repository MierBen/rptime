<template>
    <div>
      <b-button v-b-modal.modal-tall @click="$router.push('register'); modalShow = !modalShow">Register</b-button>
      <b-button v-b-modal.modal-tall @click="$router.push('login'); modalShow = !modalShow">Login</b-button>

      <router-view></router-view>
    </div>
</template>

<script>
    export default {
        data() {
            return {
                modalShow: false
            }
        }
    }
</script>