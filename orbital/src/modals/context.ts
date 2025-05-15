import { createContext } from "react";
import { CreateProjectModal } from "./components/CreateProjectModal";
import { ModalOptions } from "./util";

export const modals = {
    createProject: CreateProjectModal,
};

declare module "@mantine/modals" {
    export interface MantineModalsOverride {
        modals: typeof modals;
    }
}

export type OpenModalContextType = <T extends keyof typeof modals>(
    id: T,
    props?: Parameters<(typeof modals)[T]>["0"]["innerProps"],
    overrides?: ModalOptions,
) => string;

export const OpenModalContext = createContext<OpenModalContextType>(
    null as any,
);
