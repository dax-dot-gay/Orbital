import { ReactNode, useMemo } from "react";
import { createTauRPCProxy } from "../../bindings";
import { ApiContext } from "./util";

export function ApiProvider({
    children,
}: {
    children?: ReactNode | ReactNode[];
}) {
    const api = useMemo(() => createTauRPCProxy(), []);

    return <ApiContext.Provider value={api}>{children}</ApiContext.Provider>;
}
