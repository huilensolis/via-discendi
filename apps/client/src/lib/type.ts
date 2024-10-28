export type Some<T> = {
  _tag: "Some";
  value: T;
};

export type None = {
  _tag: "None";
};

export type Ok<T> = {
  _tag: "Ok";
  value: T;
};

export type Err<T> = {
  _tag: "Err";
  value: T;
};

export type Option<T> = Some<T> | None;
export type Result<T, K> = Ok<T> | Err<K>;

export const some = <T>(value: T): Some<T> => {
  return {
    _tag: "Some",
    value,
  };
};

export const ok = <T>(value: T): Ok<T> => {
  return {
    _tag: "Ok",
    value,
  };
};

export const err = <T>(value: T): Err<T> => {
  return {
    _tag: "Err",
    value,
  };
};

export const none = (): None => {
  return {
    _tag: "None",
  };
};

export function isSome<T>(option: Option<T>): option is Some<T> {
  return option._tag === "Some";
}
export function isOk<T, K>(result: Result<T, K>): result is Ok<T> {
  return result._tag === "Ok";
}

export function isErr<T, K>(result: Result<T, K>): result is Err<K> {
  return result._tag === "Err";
}
