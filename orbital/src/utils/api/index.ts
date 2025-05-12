import { useContext } from "react";
import { ApiContext, ApiContextType } from "./util";

export type Api = ApiContextType;

export function useApi(): Api {
    return useContext(ApiContext);
}
