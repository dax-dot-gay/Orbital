import {
    createTheme,
    DefaultMantineColor,
    MantineColorsTuple,
    MantineThemeOverride,
} from "@mantine/core";

const PrimaryColor: MantineColorsTuple = [
    "#fff0e4",
    "#ffe0cd",
    "#ffc09c",
    "#fe9d66",
    "#fe8038",
    "#fe6e1c",
    "#fe640c",
    "#f05800",
    "#cb4900",
    "#b13c00",
];

type ExtendedCustomColors = "primary" | DefaultMantineColor;

declare module "@mantine/core" {
    export interface MantineThemeColorsOverride {
        colors: Record<ExtendedCustomColors, MantineColorsTuple>;
    }
}

const themeDefaults: MantineThemeOverride = {
    fontFamily: "Fira Sans",
    fontFamilyMonospace: "Fira Code",
    colors: {
        primary: PrimaryColor,
    },
    primaryColor: "primary",
    luminanceThreshold: 0.4,
    autoContrast: true,
    defaultRadius: "sm",
};

export const ThemeDark = createTheme({
    primaryShade: 7,

    ...themeDefaults,
});

export const ThemeLight = createTheme({
    primaryShade: 3,
    ...themeDefaults,
});
