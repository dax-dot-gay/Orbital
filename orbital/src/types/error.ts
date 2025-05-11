import { isFunction } from "lodash";
import { Result } from "ts-results";

export type FrontendError =
    | {
          kind: "unknown_version";
          version: string;
      }
    | {
          kind: "unset_version";
      }
    | {
          kind: "uninitialized_context";
          context: string;
      }
    | {
          kind: "unexpected";
          reason?: string;
      }
    | {
          kind: "invalid_asset_path";
          version: string;
          path: string;
          reason?: string;
      }
    | {
          kind: "file_operation";
          operation: "open" | "read" | "write" | "other";
          path: string;
      }
    | {
          kind: "json";
          operation: "decode";
          data: string;
      }
    | {
          kind: "json";
          operation: "encode";
          data: object;
      };

export type ApplicationError = FrontendError;

export type Res<T> = Result<T, ApplicationError>;

export function isResult<T = any>(obj: any): obj is Res<T> {
    if (
        isFunction(obj.expect) &&
        isFunction(obj.unwrap) &&
        isFunction(obj.else) &&
        isFunction(obj.unwrapOr) &&
        isFunction(obj.andThen) &&
        isFunction(obj.map) &&
        isFunction(obj.mapErr) &&
        isFunction(obj.toOption)
    ) {
        return true;
    } else {
        return false;
    }
}
