import React, {Component} from 'react';
import './App.scss';

import Button from 'react-bootstrap/Button';
import Navbar from 'react-bootstrap/Navbar';
import Nav from 'react-bootstrap/Nav';
import Form from 'react-bootstrap/Form';
import NavDropdown from 'react-bootstrap/NavDropdown';
import Modal from 'react-bootstrap/Modal';
import Jumbotron from "react-bootstrap/Jumbotron";
import {BrowserRouter as Router, Switch, Route, Link, Redirect} from "react-router-dom";

const MessageTypes = {
    newGame: "newGame"
}
const MessageFields = {
    sessionToken: "sessionToken"
}
const Cookies = {
    sessionToken: "sessionToken"
}

class App extends Component {
    ws = new WebSocket('ws://localhost:8000/ws')

    constructor(props) {
        super(props);
        this.gameName = React.createRef();
        this.login = React.createRef();

        this.state = {
            createShow: false,
            joinShow: false,
            dataFromServer: ""
        };
    }

    componentDidMount() {

        this.ws.onopen = () => {
            console.log('connected')
        }

        this.ws.onmessage = evt => {
            console.log("received")
            const message = JSON.parse(evt.data)
            this.setState({dataFromServer: message})
            if (MessageTypes.newGame in message) {
                const fields = message[MessageTypes.newGame]
                setCookie(Cookies.sessionToken, fields[MessageFields.sessionToken], {
                    maxAge: 24 * 60 * 60,
                    path: '/'
                })
                return <Redirect push to="/game" />;
                // route to game
            }
            console.log(message)
        }

        this.ws.onclose = () => {
            console.log('disconnected')
        }
    }

    sendMessage = (json) => {
        console.log("Senidng", this.ws.readyState)
        try {
            this.ws.send(JSON.stringify(json)) //send data to the server
        } catch (error) {
            console.log(error) // catch error
        }
    }

    createGame = () => {
        console.log("login: ", this.login.current.value)
        this.sendMessage({"newGame": {"login": this.login.current.value}})
    }

    render() {
        const closeCreate = () => this.setState({createShow: false})
        const showCreate = () => this.setState({createShow: true})
        const closeJoin = () => this.setState({joinShow: false})
        const showJoin = () => this.setState({joinShow: true})

        return (
            <>
                <Navbar bg="light" expand="lg">
                    <Navbar.Brand href="#home">Game</Navbar.Brand>
                    <Navbar.Toggle aria-controls="basic-navbar-nav"/>
                    <Navbar.Collapse id="basic-navbar-nav">
                        <Nav className="mr-auto">
                            <Nav.Link href="#home">Home</Nav.Link>
                            <Nav.Link href="#link">Games</Nav.Link>
                            <NavDropdown title="User" id="basic-nav-dropdown">
                                {/*<NavDropdown.Item href="#action/3.1">Login</NavDropdown.Item>*/}
                                {/*<NavDropdown.Item href="#action/3.2">Register</NavDropdown.Item>*/}
                            </NavDropdown>
                        </Nav>
                    </Navbar.Collapse>
                </Navbar>
                <Jumbotron style={{
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                }}>
                    <Button onClick={showCreate}> New Game</Button>
                    <Button onClick={showJoin}> Join Game</Button>
                </Jumbotron>

                {/* Create game */}
                <Modal show={this.state.createShow} onHide={closeCreate}>
                    <Modal.Header closeButton>
                        <Modal.Title>Create Game</Modal.Title>
                    </Modal.Header>
                    <Form>
                        <Form.Group controlId="formBasicEmail">
                            <Form.Label>Add your name:</Form.Label>
                            <Form.Control type="text" placeholder="Login" ref={this.login} required/>
                            <Form.Text className="text-muted">
                                Name that will be used in game.
                            </Form.Text>
                        </Form.Group>
                    </Form>

                    <Modal.Footer>
                        <Button variant="secondary" onClick={closeCreate}>
                            Close
                        </Button>
                        <Button variant="primary" onClick={this.createGame}>
                            Create
                        </Button>
                    </Modal.Footer>
                </Modal>


                <Modal show={this.state.joinShow} onHide={closeJoin}>
                    <Modal.Header closeButton>
                        <Modal.Title>Join Game</Modal.Title>
                    </Modal.Header>
                    <Form>
                        <Form.Group controlId="formBasicEmail">
                            <Form.Label>Add your name:</Form.Label>
                            <Form.Control type="text" placeholder="Login"/>
                            <Form.Text className="text-muted">
                                Name that will be used in game.
                            </Form.Text>
                            <Form.Control type="text" placeholder="Login"/>
                            <Form.Text className="text-muted">
                                Name of the game.
                            </Form.Text>
                        </Form.Group>
                    </Form>
                    <Modal.Footer>
                        <Button variant="secondary" onClick={closeJoin}>
                            Close
                        </Button>
                        <Button variant="primary" onClick={closeJoin}>
                            Save Changes
                        </Button>
                    </Modal.Footer>
                </Modal>
            </>
        );
    }
}

export default App;
