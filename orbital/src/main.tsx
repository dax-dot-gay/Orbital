import ReactDOM from "react-dom/client";
import { App } from "./App";
import "@mantine/core/styles.css";
import "@mantine/dropzone/styles.css";
import "@mantine/notifications/styles.css";
import "@mantine/charts/styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <App />,
);
