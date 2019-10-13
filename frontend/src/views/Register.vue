<template>
    <b-modal id="modal-tall" title="Register" v-model="modalShow">
        <div> 
            <b-card bg-variant="light">
                <b-form-group
                    label-cols-sm="3"
                    label="Team name*:"
                    label-align-sm="right"
                    :invalid-feedback="invalidFeedbackTeamName()"
                    :state="stateTeamName"
                    label-for="register_team_name"
                >
                    <b-form-input
                        v-model="teamName"
                        @input="forInputTeamName()"
                        id="register_team_name"
                        :state="stateTeamName"
                        trim
                    >
                    </b-form-input>
                </b-form-group>

                <b-form-group
                    label-cols-sm="3"
                    label="Email*:"
                    label-align-sm="right"
                    :invalid-feedback="invalidFeedbackEmail()"
                    :state="stateEmail"
                    label-for="register_email"
                >
                    <b-form-input
                        v-model="email"
                        @input="forInputEmail()"
                        id="register_email"
                        :state="stateEmail"
                        trim
                    >
                    </b-form-input>
                </b-form-group>

                <b-form-group
                    label-cols-sm="3"
                    label="Country:"
                    label-align-sm="right"
                    label-for="register_country"
                >
                    <b-form-input
                        v-model="country"
                        id="register_country"
                        trim
                    >
                    </b-form-input>
                </b-form-group>

                <b-form-group
                    label-cols-sm="3"
                    label="University:"
                    label-align-sm="right"
                    label-for="register_university"
                >
                    <b-form-input
                        v-model="university"
                        id="register_university"
                        trim
                    >
                    </b-form-input>
                </b-form-group>
            </b-card>
          {{info}}
        </div>

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
                v-b-modal.modal-tall
                :disabled="signUpIsDisabled"
                class="float-right"
                variant="primary"
                @click="signUp()"
            >
                Sign Up
            </b-button>
        </template>

    </b-modal>
</template>

<script>
    import axios from 'axios';

    export default {
        data() {
            return {
                endpoint: '/api/v1/register',
                modalShow: true,
                teamName: '',
                email: '',
                country: '',
                university: '',
                info: '',

                teamNameRegexp: /^([а-яА-ЯЁё\sa-zA-Z0-9._\-]+){3,40}$/,
                emailRegexp: /^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,12}$/,

                validFieldTeamNameResponse: true,
                validFieldEmailResponse: true,
                
                stateTeamName: null,
                stateEmail: null,
                signUpIsDisabled: true,
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
            forInputTeamName() {
                this.validFieldTeamNameResponse = true
                if(this.teamName.length > 2
                    && this.validFieldTeamNameResponse
                    && this.teamName.length <= 40
                    && this.teamName.match(this.teamNameRegexp) != null) {

                    this.stateTeamName = null

                    if(this.email.length > 0) {
                        this.signUpIsDisabled = false
                    }
                }
                else {
                    this.stateTeamName = false
                    this.signUpIsDisabled = true
                } 
            },

            invalidFeedbackTeamName() {
                if (this.teamName.length == 0) {
                    return 'The Team name field is required'
                }
                else if (this.teamName.length < 3) {
                    return 'The Team name must be greater than 2'
                }
                else if (!this.validFieldTeamNameResponse) {
                    return 'Team already exists'
                }
                else if (this.teamName.length > 40) {
                    return 'The Team name must be less than 40'
                }
                else if (this.teamName.match(this.teamNameRegexp) == null) {
                    return 'Invalid Team name'
                }
                else if (this.teamName.length > 3) {
                    return ''
                } 
                else {
                    return 'Please enter something'
                }
            },

            forInputEmail() {
                this.validFieldEmailResponse = true
                if(this.email.length > 0
                    && this.validFieldEmailResponse
                    && this.email.match(this.emailRegexp) != null) {

                    this.stateEmail = null
                    this.signUpIsDisabled = false
                }
                else {
                    this.stateEmail = false
                    this.signUpIsDisabled = true
                }
            },

            invalidFeedbackEmail() {
                if (this.email.length == 0) {
                    return 'The Email field is required'
                }
                else if (this.email.match(this.emailRegexp) == null) {
                    return 'Invalid email'
                }
                else if (!this.validFieldEmail) {
                    return 'Invalid email'
                }
                else if (this.email.length > 0) {
                    return ''
                }
                else {
                    return 'Please enter something'
                }
            },

            signUp() {
                var team = JSON.stringify({
                    team_name: this.teamName, 
                    email: this.email,          
                    country: this.country,
                    university: this.university
                })

                axios
                    .post(this.endpoint, team, {
                    headers: {
                        'Content-Type': 'application/json',
                    }
                    })
                    .then(response => {
                        this.info = 'Your token: ' + response.data.token
                        this.validFieldTeamNameResponse = true
                        this.validFieldEmailResponse = true
                    })
                    .catch(error => {
                        console.log(error);
                        this.errored = true;
                        var errorInfo = JSON.parse(error.response.data).error

                        this.signUpIsDisabled = true

                        if(errorInfo == 'Team already exists!') {
                            this.stateTeamName = false
                            this.validFieldTeamNameResponse = false
                        } 
                        else if (errorInfo == 'Invalid email'){
                            this.stateEmail = false
                            this.validFieldEmailResponse = false
                        }
                    })
            },

        }
    }
</script>