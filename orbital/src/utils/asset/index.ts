import { useContext, useEffect } from "react";
import {
    AssetVersionContext,
    AssetVersionContextType,
    AssetVersionId,
    AssetVersionKind,
    AssetVersionSetter,
    getResourcePath,
} from "./util";
import {
    Loading,
    useLoading,
    useLoadingPromise,
    useLoadingState,
} from "../../types/loading";
import * as path_api from "@tauri-apps/api/path";
import * as fs_api from "@tauri-apps/plugin-fs";
import { FrontendError } from "../../types/error";
import { Err, Ok } from "ts-results";

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

export function useAssetJson<T extends object = object>(
    path: string,
    default_value?: T
): Loading<T> {
    const assetPath = useLoading(useAssetPath(path));
    const result: Loading<T> = useLoadingPromise(
        async (
            value: string | null,
            error: FrontendError | null,
            _: boolean
        ) => {
            if (value) {
                try {
                    const fileContents = await fs_api.readTextFile(value);

                    try {
                        return Ok(JSON.parse(fileContents) as T);
                    } catch (_) {
                        return Err({
                            kind: "json",
                            operation: "decode",
                            data: fileContents,
                        });
                    }
                } catch (_) {
                    return Err({
                        kind: "file_operation",
                        operation: "open",
                        path: value,
                    });
                }
            } else {
                return error ? Err(error) : null;
            }
        },
        [assetPath.value, assetPath.error, assetPath.loading],
        default_value
    );

    return result;
}

export function useAssetText(
    path: string,
    default_value?: string
): Loading<string> {
    const assetPath = useLoading(useAssetPath(path));
    const result: Loading<string> = useLoadingPromise(
        async (
            value: string | null,
            error: FrontendError | null,
            _: boolean
        ) => {
            if (value) {
                try {
                    const fileContents = await fs_api.readTextFile(value);

                    return Ok(fileContents);
                } catch (_) {
                    return Err({
                        kind: "file_operation",
                        operation: "open",
                        path: value,
                    });
                }
            } else {
                return error ? Err(error) : null;
            }
        },
        [assetPath.value, assetPath.error, assetPath.loading],
        default_value
    );

    return result;
}