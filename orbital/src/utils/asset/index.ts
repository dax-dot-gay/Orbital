import { useContext, useEffect } from "react";
import {
    AssetVersionContext,
    AssetVersionContextType,
    AssetVersionId,
    AssetVersionKind,
    AssetVersionSetter,
    getResourcePath,
} from "./util";
import { Loading, useLoadingState } from "../../types/loading";
import * as path_api from "@tauri-apps/api/path";

export function useAssetVersionCtx(): AssetVersionContextType {
    return useContext(AssetVersionContext);
}

export function useAssetVersion(): [AssetVersionId | null, AssetVersionSetter] {
    const context = useAssetVersionCtx();
    return [context.currentVersion, context.setVersion];
}

export function useAssetVersionTuple(): [
    [string | null, AssetVersionKind | null],
    AssetVersionSetter
] {
    const context = useAssetVersionCtx();
    return [[context.versionNumber, context.versionType], context.setVersion];
}

export function useAssetPath(path: string): Loading<string> {
    const [version] = useAssetVersion();
    const [result, setResult] = useLoadingState<string>(null);

    useEffect(() => {
        if (version) {
            path_api.join("assets", version, path).then((joined) => {
                getResourcePath(joined).then((resource) => setResult(resource));
            });
        } else {
            setResult({ kind: "unset_version" });
        }
    }, [setResult, path, version]);

    return result;
}
