import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./main.css";
import { defaultTheme, Provider } from "@adobe/react-spectrum";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Provider theme={defaultTheme}>
      <App />
    </Provider>
  </React.StrictMode>,
);
