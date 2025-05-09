import { convertFileSrc } from "@tauri-apps/api/core";
import * as path from "@tauri-apps/api/path";
import { Res } from "../../types/error";
import { createContext } from "react";
import { Err } from "ts-results";

export async function getResourcePath(resource: string): Promise<string> {
    return convertFileSrc(
        await path.resolveResource(await path.join("resources", resource))
    );
}

export type AssetVersionKind = "stable" | "experimental" | "legacy";
export type AssetVersionId = `${string}-${AssetVersionKind}`;
export type AssetVersionSetter = (
    version: [string, AssetVersionKind] | AssetVersionId | null
) => Promise<Res<AssetVersionId | null>>;

export type AssetVersionContextType = {
    currentVersion: AssetVersionId | null;
    versionNumber: string | null;
    versionType: AssetVersionKind | null;
    setVersion: AssetVersionSetter;
};

export const AssetVersionContext = createContext<AssetVersionContextType>({
    currentVersion: null,
    versionNumber: null,
    versionType: null,
    setVersion: async (_version: any) =>
        Err({ kind: "uninitialized_context", context: "AssetVersionContext" }),
});
