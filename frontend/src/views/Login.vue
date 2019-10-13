<template>
    <b-modal id="modal-tall" title="Login" v-model="modalShow">
        <div> 
            <b-card bg-variant="light">
                <b-form-group
                    label-cols-sm="3"
                    label="Team name:"
                    label-align-sm="right"
                    label-for="login_team_name"
                >
                    <b-form-input
                        v-model="teamName"
                        id="login_team_name" 
                        trim
                    >
                    </b-form-input>
                </b-form-group>

                <b-form-group
                    label-cols-sm="3"
                    label="Token:"
                    label-align-sm="right"
                    :invalid-feedback="invalidFeedbackToken()"
                    :state="inputTokenState"
                    label-for="login_token"
                >
                    <b-form-input   
                        ref="inputToken"                    
                        v-model="token"
                        v-on:input="forInputToken()"
                        id="login_token"
                        :state="inputTokenState"
                        trim
                    >
                    </b-form-input>
                </b-form-group>
            </b-card>
        </div>
        {{info}}
        <template v-slot:modal-footer="{ cancel, ok }">
            <b-button
                v-b-modal.modal-tall
                class="float-right"
                variant="danger"
                to="/"
            >
                Cancel
            </b-button>
            <b-button 
                ref="buttonSignIn"
                v-b-modal.modal-tall
                :disabled="buttonSignInIsDisabled"
                class="float-right"
                variant="primary"
                v-on:click="signIn()"
            >
                Sign In
            </b-button>
        </template>

    </b-modal>
</template>

<script>
    import axios from 'axios'

    export default {
        data() {
            return {
                modalShow: true,

                info: '',

                teamName: '',
                token: '',

                inputTokenState: null,
                buttonSignInIsDisabled: true,
                responseForTokenIsValid: true
            }
        },

        watch: {
            modalShow() {
                if(this.modalShow == false) {
                    this.$router.go(-1)
                }
            }
        },

        methods: {
            invalidFeedbackToken() {              
                if (this.token.length == 0) {                 
                    return 'The Token name field is required'                   
                } 
                else {
                    return 'Incorrect token'
                }
            },

            forInputToken() {
                this.responseForTokenIsValid = true
                if(this.token.length > 0) {
                    this.buttonSignInIsDisabled = false
                    this.inputTokenState = null
                }
                else {
                    this.inputTokenState = false
                }
            },

            signIn() {
                var team = JSON.stringify({
                    team_name: this.teamName, 
                    token: this.token,          
                })
                axios
                    .post('api/v1/login', team, {
                        headers: {
                            'Content-Type': 'application/json',
                        }
                    })
                    .then(() => {
                        this.$router.push('/')
                    })
                    .catch(error => {
                        console.log(error)
                        var errorInfo = JSON.parse(error.response.data).error
                        this.info = errorInfo
                            
                        if(errorInfo == 'Login error! Incorrect token!') {
                            this.buttonSignInIsDisabled = true
                            this.inputTokenState = false
                        }
                    })
            }
        }

    }
</script>