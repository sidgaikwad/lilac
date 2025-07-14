import * as React from 'react';

import { Button } from '@/components/ui/button';
import { Separator } from '@/components/ui/separator';
import { defineStepper } from '@stepperize/react';
import { Form, useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import z from 'zod';
import { createWithEqualityFn } from 'zustand/traditional';

export interface FormWizardStep {
  id: string;
  title: string;
  description: string;
  schema: z.AnyZodObject;
  formContent: React.ReactNode;
}

export interface FormWizardProps {
  steps: FormWizardStep[];
  onSubmit: (values: object) => void;
}

function FormWizard(props: FormWizardProps) {
  const { useStepper, steps, utils } = defineStepper(...props.steps);
  const stepper = useStepper();

  type StepSchemaTypes = {
    [K in (typeof props.steps)[number]['id']]: z.infer<
      (typeof props.steps)[number]['schema']
    >;
  };
  const useValueStore = createWithEqualityFn<
    StepSchemaTypes & {
      setValue: (
        id: keyof StepSchemaTypes,
        values: StepSchemaTypes[keyof StepSchemaTypes]
      ) => void;
    }
  >((set) => ({
    setValue: (id, values) => set({ [id]: values }),
  }));
  const { setValue } = useValueStore();

  const form = useForm({
    mode: 'onTouched',
    resolver: zodResolver(stepper.current.schema),
  });

  const onSubmit = (values: z.infer<typeof stepper.current.schema>) => {
    if (stepper.isLast) {
      Object.entries(values).map(([k, v]) => setValue(k, v));
      props.onSubmit(values);
    } else {
      Object.entries(values).map(([k, v]) => setValue(k, v));
      stepper.next();
    }
  };

  const currentIndex = utils.getIndex(stepper.current.id);
  return (
    <Form {...form}>
      <form
        onSubmit={form.handleSubmit(onSubmit)}
        className='w-[450px] space-y-6 rounded-lg border p-6'
      >
        <div className='flex justify-between'>
          <h2 className='text-lg font-medium'>Checkout</h2>
          <div className='flex items-center gap-2'>
            <span className='text-gray-text-muted text-sm'>
              Step {currentIndex + 1} of {steps.length}
            </span>
          </div>
        </div>
        <nav aria-label='Checkout Steps' className='group my-4'>
          <ol
            className='flex items-center justify-between gap-2'
            aria-orientation='horizontal'
          >
            {stepper.all.map((step, index, array) => (
              <React.Fragment key={step.id}>
                <li className='flex flex-shrink-0 items-center gap-4'>
                  <Button
                    type='button'
                    role='tab'
                    variant={index <= currentIndex ? 'default' : 'secondary'}
                    aria-current={
                      stepper.current.id === step.id ? 'step' : undefined
                    }
                    aria-posinset={index + 1}
                    aria-setsize={steps.length}
                    aria-selected={stepper.current.id === step.id}
                    className='flex size-10 items-center justify-center rounded-full'
                    onClick={async () => {
                      const valid = await form.trigger();
                      //must be validated
                      if (!valid) return;
                      //can't skip steps forwards but can go back anywhere if validated
                      if (index - currentIndex > 1) return;
                      stepper.goTo(step.id);
                    }}
                  >
                    {index + 1}
                  </Button>
                  <span className='text-sm font-medium'>{step.title}</span>
                </li>
                {index < array.length - 1 && (
                  <Separator
                    className={`flex-1 ${
                      index < currentIndex ? 'bg-primary' : 'bg-muted'
                    }`}
                  />
                )}
              </React.Fragment>
            ))}
          </ol>
        </nav>
        <div className='space-y-4'>
          {stepper.switch(
            Object.values(props.steps).reduce(
              (obj, step) => ({
                [step.id]: step.formContent,
                ...obj,
              }),
              {}
            )
          )}
          {!stepper.isLast ? (
            <div className='flex justify-end gap-4'>
              <Button
                variant='secondary'
                onClick={stepper.prev}
                disabled={stepper.isFirst}
              >
                Back
              </Button>
              <Button type='submit'>
                {stepper.isLast ? 'Complete' : 'Next'}
              </Button>
            </div>
          ) : (
            <Button onClick={stepper.reset}>Reset</Button>
          )}
        </div>
      </form>
    </Form>
  );
}

export default FormWizard;
