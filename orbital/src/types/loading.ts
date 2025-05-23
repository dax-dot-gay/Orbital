import { Err, Ok } from "ts-results";
import { ApplicationError, isResult, Res } from "./error";
import { useCallback, useEffect, useMemo, useState } from "react";
import { isEqual, isObject, isString } from "lodash";

type LoadingType<T> =
    | {
          state: "ready";
          value: T;
      }
    | {
          state: "error";
          error: ApplicationError;
      }
    | {
          state: "loading";
      };

export class Loading<T> {
    public constructor(protected data: LoadingType<T>) {}

    public static auto<T = any>(
        val: T | ApplicationError | null | Loading<T>
    ): Loading<T> {
        if (val === null) {
            return new Loading<T>({ state: "loading" });
        } else if (["loading", "error", "ready"].includes((val as any).state)) {
            return new Loading<T>((val as Loading<T>).data);
        } else if (
            isObject(val) &&
            Object.keys(val).includes("kind") &&
            isString((val as ApplicationError).kind)
        ) {
            return new Loading<T>({
                state: "error",
                error: val as ApplicationError,
            });
        } else {
            return new Loading<T>({ state: "ready", value: val as T });
        }
    }

    public get state(): LoadingType<T>["state"] {
        return this.data.state;
    }

    public result(): Res<T | null> {
        switch (this.state) {
            case "loading":
                return Ok(null);
            case "error":
                return Err((this.data as any).error);
            case "ready":
                return Ok((this.data as any).value);
        }
    }

    public get ready(): boolean {
        return this.state === "ready";
    }

    public get failed(): boolean {
        return this.state === "error";
    }

    public get loading(): boolean {
        return this.state === "loading";
    }

    public get value(): T | null {
        if (this.ready) {
            return (this.data as any).value;
        } else {
            return null;
        }
    }

    public get err(): ApplicationError | null {
        if (this.failed) {
            return (this.data as any).error;
        } else {
            return null;
        }
    }

    public unwrap(): T {
        return this.value ?? (undefined as unknown as T);
    }

    public unwrap_err(): ApplicationError {
        return this.err ?? (undefined as unknown as ApplicationError);
    }
}

export function Ready<T>(value: T): Loading<T> {
    return new Loading<T>({ state: "ready", value });
}

export function Waiting<T = any>(): Loading<T> {
    return new Loading<T>({ state: "loading" });
}

export function Failed<T = any>(error: ApplicationError): Loading<T> {
    return new Loading<T>({ state: "error", error });
}

export function resolveLoading<T, F = T | null>(
    inner: Loading<T>,
    if_loading?: F,
    if_error?: (error: ApplicationError) => F
): T | F {
    if (inner.ready) {
        return inner.unwrap();
    } else if (inner.failed) {
        if (if_error) {
            return if_error(inner.unwrap_err());
        } else {
            return null as F;
        }
    } else {
        return if_loading ?? (null as F);
    }
}

export function useFlattenedLoading<T, F = T | null>(
    inner: Loading<T>,
    if_loading?: F,
    if_error?: (error: ApplicationError) => F
): T | F {
    const [result, setResult] = useState<T | F>(
        resolveLoading<T, F>(inner, if_loading, if_error)
    );

    useEffect(() => {
        const new_result = resolveLoading<T, F>(inner, if_loading, if_error);
        if (!isEqual(result, new_result)) {
            setResult(new_result);
        }
    }, [inner, if_loading, if_error, setResult]);

    return result;
}

export function useLoading<T>(inner: Loading<T>): {
    value: T | null;
    error: ApplicationError | null;
    loading: boolean;
} {
    const [extracted, setExtracted] = useState<{
        value: T | null;
        error: ApplicationError | null;
        loading: boolean;
    }>({ value: inner.value, error: inner.err, loading: inner.loading });

    useEffect(() => {
        if (
            !isEqual(inner.value, extracted.value) ||
            !isEqual(inner.err, extracted.error) ||
            inner.loading !== extracted.loading
        ) {
            setExtracted({
                value: inner.value,
                error: inner.err,
                loading: inner.loading,
            });
        }
    }, [inner, setExtracted, extracted]);

    return extracted;
}

export function useLoadingState<T = object>(
    initialState?: T | ApplicationError | null | Loading<T>
): [Loading<T>, (state: T | ApplicationError | null | Loading<T>) => void] {
    const [state, setState] = useState<Loading<T>>(
        Loading.auto<T>(initialState ?? null)
    );

    const autoSet = useCallback(
        (state: T | ApplicationError | null | Loading<T>) => {
            setState(Loading.auto<T>(state));
        },
        [setState]
    );

    return [state, autoSet];
}

export function useLoadingPromise<A extends any[], R>(
    callable: (...args: A) => Promise<Res<R> | R | null | ApplicationError>,
    args: A,
    _default?: R,
    onError?: (error: ApplicationError, args: A) => R
): Loading<R> {
    const [result, setResult] = useLoadingState<R>(_default);

    useEffect(() => {
        callable(...args).then((val) =>
            setResult(isResult<R>(val) ? val.val : val)
        );
    }, [setResult, callable, ...args]);

    const { value, error, loading } = useLoading<R>(result);

    const output = useMemo(() => {
        if (value !== null) {
            return Ready(value);
        } else if (error !== null) {
            return onError ? Ready(onError(error, args)) : Failed<R>(error);
        } else {
            return _default === undefined ? Waiting<R>() : Ready(_default);
        }
    }, [value, error, loading, _default, onError, ...args]);

    return output;
}
