import React from 'react';
import ReactDOM from 'react-dom';
import { Router } from "@reach/router";
import { ToastContainer } from "react-toastify";

import "./index.css";
import * as serviceWorker from './serviceWorker';
import Header from "./components/header";

ReactDOM.render(
  <React.StrictMode>
      <Header/>
      <Router>

      </Router>
      <ToastContainer autoClose={2500} pauseOnHover={false} draggable={false} position="top-center"/>
  </React.StrictMode>,
  document.getElementById('root')
);

// If you want your app to work offline and load faster, you can change
// unregister() to register() below. Note this comes with some pitfalls.
// Learn more about service workers: https://bit.ly/CRA-PWA
serviceWorker.unregister();
