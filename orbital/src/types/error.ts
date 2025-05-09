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
      };

export type ApplicationError = FrontendError;

export type Res<T> = Result<T, ApplicationError>;
