import { useCallback, useContext } from "react";
import { OpenModalProvider } from "./OpenModalProvider";
import { modals, OpenModalContext } from "./context";
import { modals as mantine_modals } from "@mantine/modals";
import { ModalOptions } from "./util";

export function useModal<T extends keyof typeof modals>(
    id: T,
): {
    open: (
        props?: Parameters<(typeof modals)[T]>["0"]["innerProps"],
        overrides?: ModalOptions,
    ) => string;
    close: () => void;
} {
    const getModal = useContext(OpenModalContext);

    const openModal = useCallback(
        (
            props?: Parameters<(typeof modals)[T]>["0"]["innerProps"],
            overrides?: ModalOptions,
        ) => getModal(id, props, overrides),
        [id, getModal],
    );

    const closeModal = useCallback(() => mantine_modals.close(id), [id]);

    return { open: openModal, close: closeModal };
}

export { OpenModalProvider };
