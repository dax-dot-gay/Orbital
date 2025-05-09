import { createContext, useContext } from "react";

export type ThemeMode = "dark" | "light";
export type ThemeContextType = [ThemeMode, (mode: ThemeMode) => void];

export const ThemeContext = createContext<ThemeContextType>(null as any);

export function useAppTheme(): [ThemeMode, (mode: ThemeMode) => void] {
    return useContext(ThemeContext);
}
