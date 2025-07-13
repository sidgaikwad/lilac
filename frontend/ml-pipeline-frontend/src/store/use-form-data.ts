import { createWithEqualityFn } from 'zustand/traditional';

type RecursivePartial<T> = {
  [P in keyof T]?: T[P] extends (infer U)[]
    ? RecursivePartial<U>[]
    : T[P] extends object | undefined
      ? RecursivePartial<T[P]>
      : T[P];
};

interface FormDataState<T> {
  formValues: RecursivePartial<T>;
  setFormValues: (values: RecursivePartial<T>) => void;
}

function createFormStore<T>() {
  const useFormStore = createWithEqualityFn<FormDataState<T>>((set, get) => ({
    formValues: {},
    setFormValues: (values) =>
      set({ formValues: { ...get().formValues, ...values } }),
  }));
  return useFormStore;
}

export default createFormStore;
