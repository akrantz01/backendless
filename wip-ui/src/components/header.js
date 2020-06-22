import React, { Component } from 'react';
import {Alignment, Button, Classes, Dialog, FormGroup, InputGroup, Navbar} from "@blueprintjs/core";
import { toast } from "react-toastify";

import { Authentication, Users } from "../bindings";

export default class Header extends Component {
    constructor(props) {
        super(props);

        this.state = {
            login: false,
            register: false,
            email: "",
            username: "",
            password: ""
        }
    }

    toggleLogin = () => this.setState({ login: !this.state.login, email: "", username: "", password: "" });
    toggleRegister = () => this.setState({ register: !this.state.register, email: "", username: "", password: "" });

    onEmailChange = e => this.setState({email: e.target.value});
    onUsernameChange = e => this.setState({username: e.target.value});
    onPasswordChange = e => this.setState({password: e.target.value});

    async onLogin() {
        let result = await Authentication.login(this.state.email, this.state.password);
        if (result.status !== 200) {
            toast.error("Invalid username or password");
            return;
        }

        let user_info = await Users.read();
        localStorage.setItem("user", JSON.stringify(user_info.data));

        toast.success("Successfully logged in");
    }

    async onRegister() {
        let result = await Authentication.register(this.state.email, this.state.username, this.state.password);
        if (result.status !== 200) {
            toast.error(`Registration failure: ${result.reason}`);
            return;
        }

        toast.success("Successfully registered, you may now login");
        this.toggleRegister();
    }

    render() {
        return (
            <>
                <Navbar>
                    <Navbar.Group align={Alignment.LEFT}>
                        <Navbar.Heading>Backendless</Navbar.Heading>
                        <Navbar.Divider/>
                        <Button className={Classes.MINIMAL} icon="home" text="Home"/>
                    </Navbar.Group>

                    <Navbar.Group align={Alignment.RIGHT}>
                        <Button className={Classes.MINIMAL} icon="user" text="Login" intent="primary" onClick={this.toggleLogin.bind(this)}/>
                        <Navbar.Divider/>
                        <Button className={Classes.MINIMAL} text="Register" intent="secondary" onClick={this.toggleRegister.bind(this)}/>
                    </Navbar.Group>
                </Navbar>

                <Dialog icon="user" title="Login" isOpen={this.state.login} usePortal={true} onClose={this.toggleLogin.bind(this)}>
                    <div className={Classes.DIALOG_BODY}>
                        <FormGroup label="Email" labelFor="login-email" labelInfo="(required)">
                            <InputGroup id="login-email" value={this.state.email} onChange={this.onEmailChange.bind(this)}/>
                        </FormGroup>
                        <FormGroup label="Password" labelFor="login-password" labelInfo="(required)">
                            <InputGroup id="login-password" value={this.state.password} type="password" onChange={this.onPasswordChange.bind(this)}/>
                        </FormGroup>
                    </div>
                    <div className={Classes.DIALOG_FOOTER}>
                        <div className={Classes.DIALOG_FOOTER_ACTIONS}>
                            <Button onClick={this.toggleLogin.bind(this)} minimal={true} text="Nevermind"/>
                            <Button onClick={this.onLogin.bind(this)} intent="success" text="Login"/>
                        </div>
                    </div>
                </Dialog>

                <Dialog icon="user" title="Register" isOpen={this.state.register} usePortal={true} onClose={this.toggleRegister.bind(this)}>
                    <div className={Classes.DIALOG_BODY}>
                        <FormGroup label="Email" labelFor="register-email" labelInfo="(required)">
                            <InputGroup id="register-email" value={this.state.email} onChange={this.onEmailChange.bind(this)}/>
                        </FormGroup>
                        <FormGroup label="Username" labelFor="register-username" labelInfo="(required)">
                            <InputGroup id="register-username" value={this.state.username} onChange={this.onUsernameChange.bind(this)}/>
                        </FormGroup>
                        <FormGroup label="Password" labelFor="register-password" labelInfo="(required)">
                            <InputGroup id="register-password" value={this.state.password} type="password" onChange={this.onPasswordChange.bind(this)}/>
                        </FormGroup>
                    </div>
                    <div className={Classes.DIALOG_FOOTER}>
                        <div className={Classes.DIALOG_FOOTER_ACTIONS}>
                            <Button onClick={this.toggleRegister.bind(this)} minimal={true} text="Nevermind"/>
                            <Button onClick={this.onRegister.bind(this)} intent="success" text="Register"/>
                        </div>
                    </div>
                </Dialog>
            </>
        )
    }
}
