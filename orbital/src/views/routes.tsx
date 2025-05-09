import { BrowserRouter, Route, Routes } from "react-router";
import { AppLayout } from "./layout/Layout";
import { StartupView } from "./startup/Startup";

export function Routing() {
    return (
        <BrowserRouter>
            <Routes>
                <Route element={<AppLayout />}>
                    <Route index element={<StartupView />} />
                </Route>
            </Routes>
        </BrowserRouter>
    );
}
