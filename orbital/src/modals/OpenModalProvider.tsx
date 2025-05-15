import { ReactNode, useCallback } from "react";
import { useTranslation } from "react-i18next";
import { modals, OpenModalContext, OpenModalContextType } from "./context";
import { ModalOptions } from "./util";
import { CreateProjectFactory } from "./components/CreateProjectModal";
import { ModalsProvider, ModalsProviderProps } from "@mantine/modals";

export function OpenModalProvider({
    children,
    ...managerProps
}: { children?: ReactNode | ReactNode[] } & Partial<
    Omit<ModalsProviderProps, "children" | "modals">
>) {
    const { t } = useTranslation();
    const modalOpener: OpenModalContextType = useCallback(
        <T extends keyof typeof modals>(
            id: T,
            props?: Parameters<(typeof modals)[T]>["0"]["innerProps"],
            overrides?: ModalOptions,
        ) => {
            const openers: {
                [key in keyof typeof modals]: (
                    props: Parameters<(typeof modals)[key]>["0"]["innerProps"],
                    overrides?: ModalOptions,
                ) => string;
            } = {
                createProject: CreateProjectFactory(t),
            };

            return openers[id](props ?? {}, overrides);
        },
        [t],
    );

    return (
        <ModalsProvider modals={modals} {...managerProps}>
            <OpenModalContext.Provider value={modalOpener}>
                {children}
            </OpenModalContext.Provider>
        </ModalsProvider>
    );
}
