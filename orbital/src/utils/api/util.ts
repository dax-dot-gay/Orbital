import { createContext } from "react";
import { createTauRPCProxy } from "../../bindings";

export type ApiContextType = ReturnType<typeof createTauRPCProxy>;
export const ApiContext = createContext<ApiContextType>(null as any);
