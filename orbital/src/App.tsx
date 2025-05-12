import { MantineProvider } from "@mantine/core";
import { Notifications } from "@mantine/notifications";
import { ModalsProvider } from "@mantine/modals";
import { useState } from "react";
import { ThemeContext, ThemeMode } from "./utils/theme/themeContext";

import "./styles/index.scss";

// DARK THEME
import { shadcnCssVariableResolver as darkVariableResolver } from "./utils/theme/dark/cssVariableResolver";
import { shadcnTheme as darkTheme } from "./utils/theme/dark/theme";

// LIGHT THEME
import { shadcnCssVariableResolver as lightVariableResolver } from "./utils/theme/light/cssVariableResolver";
import { shadcnTheme as lightTheme } from "./utils/theme/light/theme";

import "./utils/theme/style.css";
import { Routing } from "./views/routes";
import { LocalizationProvider } from "./utils/Localization";
import { AssetVersionProvider } from "./utils/asset/AssetVersionProvider";
import { ApiProvider } from "./utils/api/ApiProvider";

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
                                themeMode === "dark" ? darkTheme : lightTheme
                            }
                            cssVariablesResolver={
                                themeMode === "dark"
                                    ? darkVariableResolver
                                    : lightVariableResolver
                            }
                        >
                            <ModalsProvider>
                                <Notifications />
                                <Routing />
                            </ModalsProvider>
                        </MantineProvider>
                    </AssetVersionProvider>
                </LocalizationProvider>
            </ThemeContext.Provider>
        </ApiProvider>
    );
}
