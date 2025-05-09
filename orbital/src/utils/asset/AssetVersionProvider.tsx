import { ReactNode, useCallback, useMemo, useState } from "react";
import {
    AssetVersionContext,
    AssetVersionId,
    AssetVersionKind,
    getResourcePath,
} from "./util";
import { Err, Ok } from "ts-results";
import * as fs from "@tauri-apps/plugin-fs";
import { FrontendError } from "../../types/error";
import { isArray } from "lodash";

export function AssetVersionProvider({
    children,
}: {
    children?: ReactNode | ReactNode[];
}) {
    const [version, setVersion] = useState<AssetVersionId | null>("1.0-stable");
    const [versionNumber, versionType] = useMemo(() => {
        if (version === null) {
            return [null, null];
        } else {
            const result = version.split("-", 2);
            return [result[0] ?? null, result[1] ?? null];
        }
    }, [version]);

    const trySetVersion = useCallback(
        async (version: [string, AssetVersionKind] | AssetVersionId | null) => {
            if (version === null) {
                setVersion(null);
                return Ok(null);
            }

            let parsed_version: AssetVersionId | null = null;
            if (isArray(version)) {
                parsed_version = version.join("-") as AssetVersionId;
            } else if (version === null) {
                parsed_version = null;
            } else {
                parsed_version = version;
            }

            const requestedFolder = await getResourcePath(
                `assets/${parsed_version}`
            );
            if (await fs.exists(requestedFolder)) {
                setVersion(parsed_version);
                return Ok(parsed_version);
            } else {
                return Err({
                    kind: "unknown_version",
                    version: parsed_version,
                } as FrontendError);
            }
        },
        [setVersion]
    );

    return (
        <AssetVersionContext.Provider
            value={{
                currentVersion: version,
                versionNumber,
                versionType: versionType as AssetVersionKind | null,
                setVersion: trySetVersion,
            }}
        >
            {children}
        </AssetVersionContext.Provider>
    );
}
