import { describe, expect, test } from 'vitest';
import { camelCaseObject, snakeCaseObject } from './utils';

describe('snakeCaseObjects', () => {
  test('handles objects successfully', () => {
    const obj = {
      key1: 'value1',
      anotherKey: 'anotherValue',
      someNumberValue: 12345,
    };
    const snakeCaseObj = snakeCaseObject(obj);
    expect(Object.keys(snakeCaseObj)).toEqual([
      'key_1',
      'another_key',
      'some_number_value',
    ]);
    expect(snakeCaseObj.key_1).toEqual(obj.key1);
    expect(snakeCaseObj.another_key).toEqual(obj.anotherKey);
    expect(snakeCaseObj.some_number_value).toEqual(obj.someNumberValue);
  });

  test('handles nested objects successfully', () => {
    const obj = {
      arr: [
        {
          myKey: 'myValue',
        },
      ],
      nestedObj: {
        nestedKey: {
          anotherNestedKey: 42,
        },
      },
    };
    const snakeCaseObj = snakeCaseObject(obj);
    expect(Object.keys(snakeCaseObj)).toEqual(['arr', 'nested_obj']);
    expect(Object.keys(snakeCaseObj.arr[0])).toEqual(['my_key']);
    expect(Object.keys(snakeCaseObj.nested_obj)).toEqual(['nested_key']);
    expect(Object.keys(snakeCaseObj.nested_obj.nested_key)).toEqual([
      'another_nested_key',
    ]);
    expect(snakeCaseObj.arr[0].my_key).toEqual(obj.arr[0].myKey);
    expect(snakeCaseObj.nested_obj.nested_key.another_nested_key).toEqual(
      obj.nestedObj.nestedKey.anotherNestedKey
    );
  });
});

describe('camelCaseObjects', () => {
  test('handles objects successfully', () => {
    const obj = {
      key_1: 'value1',
      another_key: 'anotherValue',
      some_number_value: 12345,
    };
    const camelCaseObj = camelCaseObject(obj);
    expect(Object.keys(camelCaseObj)).toEqual([
      'key1',
      'anotherKey',
      'someNumberValue',
    ]);
    expect(camelCaseObj.key1).toEqual(obj.key_1);
    expect(camelCaseObj.anotherKey).toEqual(obj.another_key);
    expect(camelCaseObj.someNumberValue).toEqual(obj.some_number_value);
  });

  test('handles nested objects successfully', () => {
    const obj = {
      arr: [
        {
          my_key: 'my_value',
        },
      ],
      nested_obj: {
        nested_key: {
          another_nested_key: 42,
        },
      },
    };
    const camelCaseObj = camelCaseObject(obj);
    expect(Object.keys(camelCaseObj)).toEqual(['arr', 'nestedObj']);
    expect(Object.keys(camelCaseObj.arr[0])).toEqual(['myKey']);
    expect(Object.keys(camelCaseObj.nestedObj)).toEqual(['nestedKey']);
    expect(Object.keys(camelCaseObj.nestedObj.nestedKey)).toEqual([
      'anotherNestedKey',
    ]);
    expect(camelCaseObj.arr[0].myKey).toEqual(obj.arr[0].my_key);
    expect(camelCaseObj.nestedObj.nestedKey.anotherNestedKey).toEqual(
      obj.nested_obj.nested_key.another_nested_key
    );
  });
});
