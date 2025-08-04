import { Routes } from '@/constants';
import { clsx, type ClassValue } from 'clsx';
import { camelCase, snakeCase as lodashSnakeCase } from 'lodash';
import { generatePath, PathParam } from 'react-router-dom';
import { twMerge } from 'tailwind-merge';
import { CamelCasedPropertiesDeep, SnakeCasedPropertiesDeep } from 'type-fest';
import { WordsOptions } from 'type-fest/source/words';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function mapObject<T extends object, R>(
  obj: T,
  fn: (v: string) => string
): R {
  if (Array.isArray(obj)) {
    return obj.map((x) => mapObject(x, fn)) as R;
  } else if (typeof obj === 'object') {
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
  return obj;
}

export function snakeCaseObject<T extends object, Options extends WordsOptions>(
  obj: T,
  splitNumbers?: boolean
): SnakeCasedPropertiesDeep<T, Options>;
export function snakeCaseObject<
  T extends object,
  Options extends WordsOptions = { splitOnNumbers: true },
>(obj: T, splitNumbers: true): SnakeCasedPropertiesDeep<T, Options>;
export function snakeCaseObject<
  T extends object,
  Options extends WordsOptions = { splitOnNumbers: false },
>(obj: T, splitNumbers: false): SnakeCasedPropertiesDeep<T, Options>;
export function snakeCaseObject<T extends object, Options extends WordsOptions>(
  obj: T,
  splitNumbers: boolean = true
): SnakeCasedPropertiesDeep<T, Options> {
  return mapObject(obj, (s) => snakeCase(s, splitNumbers));
}

export function camelCaseObject<T extends object>(
  obj: T
): CamelCasedPropertiesDeep<T> {
  return mapObject(obj, camelCase) as CamelCasedPropertiesDeep<T>;
}

export function snakeCase(s: string, splitNumbers: boolean = true): string {
  let result = lodashSnakeCase(s);
  if (!splitNumbers) {
    result = result.replace(/_(\d+)/, '$1');
  }
  return result;
}

export function route<Path extends Routes>(route: Path, params: {
  [key in PathParam<Path>]: string | null;
}) {
  return generatePath(route, params);
}