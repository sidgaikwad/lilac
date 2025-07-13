import { clsx, type ClassValue } from 'clsx';
import { camelCase, snakeCase } from 'lodash';
import { twMerge } from 'tailwind-merge';
import { CamelCasedPropertiesDeep, SnakeCasedPropertiesDeep } from 'type-fest';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function mapObject<T extends object, R>(
  obj: T,
  fn: (v: string) => string
): R {
  return Object.fromEntries(
    Object.entries(obj).map(([k, v]) => [
      fn(k),
      Array.isArray(v)
        ? v.map((x) => mapObject(x, fn))
        : Object(v) === v
          ? mapObject(v, fn)
          : v,
    ])
  ) as R;
}

export function snakeCaseObject<T extends object>(
  obj: T
): SnakeCasedPropertiesDeep<T, { splitOnNumbers: true }> {
  return mapObject(obj, snakeCase);
}

export function camelCaseObject<T extends object>(
  obj: T
): CamelCasedPropertiesDeep<T> {
  return mapObject(obj, camelCase) as CamelCasedPropertiesDeep<T>;
}

