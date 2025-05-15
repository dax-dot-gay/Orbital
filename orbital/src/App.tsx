import { MantineProvider } from "@mantine/core";
import { Notifications } from "@mantine/notifications";
import { useState } from "react";
import { ThemeContext, ThemeMode } from "./utils/theme/themeContext";

import "./styles/index.scss";
import { Routing } from "./views/routes";
import { LocalizationProvider } from "./utils/Localization";
import { AssetVersionProvider } from "./utils/asset/AssetVersionProvider";
import { ApiProvider } from "./utils/api/ApiProvider";
import { OpenModalProvider } from "./modals";
import { ThemeDark, ThemeLight } from "./utils/theme/themes";

export function App() {
    const [themeMode, setThemeMode] = useState<ThemeMode>("dark");

    return (
        <ApiProvider>
            <ThemeContext.Provider value={[themeMode, setThemeMode]}>
                <LocalizationProvider>
                    <AssetVersionProvider>
                        <MantineProvider
                            forceColorScheme={themeMode}
                            theme={
                                themeMode === "dark" ? ThemeDark : ThemeLight
                            }
                        >
                            <OpenModalProvider>
                                <Notifications />
                                <Routing />
                            </OpenModalProvider>
                        </MantineProvider>
                    </AssetVersionProvider>
                </LocalizationProvider>
            </ThemeContext.Provider>
        </ApiProvider>
    );
}
