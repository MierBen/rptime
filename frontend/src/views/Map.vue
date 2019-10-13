<template>
    <div v-if="isAuthenticated">Map</div>
</template>

<script>
    import axios from 'axios'

    export default {
        data() {
            return {
                isAuthenticated: false
            }
        },

        created() {
            axios
                .get('api/v1/map', {
                    data: {}, 
                    headers: {
                        'Content-Type': 'application/json',
                    }
                })
                .then((resp) => {
                    console.log(resp)
                    this.isAuthenticated = true
                    return
                })
                .catch(error => {
                    console.log(error)
                    var errorInfo = error.response.data //JSON.parse(error.response.data).error
                    if(errorInfo == "error: You didn't login!") {
                        this.$router.push('/about')
                    }

                })
        },

    }
</script>