import React, { useState, useEffect } from 'react';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
  DialogClose,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { StepDefinition, ParameterDefinition } from '@/types';
import { cn } from '@/lib/utils';

interface ParameterEditDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onSave: (updatedParams: Record<string, any>) => void;
  nodeLabel: string | undefined;
  stepDefinition: StepDefinition | undefined;
  initialParamValues: Record<string, any>;
}

const ParameterEditDialog: React.FC<ParameterEditDialogProps> = ({
  isOpen,
  onClose,
  onSave,
  nodeLabel,
  stepDefinition,
  initialParamValues,
}) => {
  const [currentParams, setCurrentParams] = useState(initialParamValues);

  useEffect(() => {
    if (isOpen) {
      setCurrentParams(initialParamValues);
    }
  }, [isOpen, initialParamValues]);

  const handleInputChange = (paramName: string, value: string | number | boolean) => {
    const paramDef = stepDefinition?.parameters.find(p => p.name === paramName);
    let finalValue = value;

    if (paramDef?.type === 'number') {
      finalValue = parseFloat(value as string) || 0;
    } else if (paramDef?.type === 'boolean') {
      // TODO: Implement proper boolean input (e.g., Checkbox, Switch)
      finalValue = value === 'true' || value === true;
    }
    // TODO: Handle 'enum', 's3_path' types

    setCurrentParams(prev => ({ ...prev, [paramName]: finalValue }));
  };

  const handleSave = () => {
    onSave(currentParams);
    onClose();
  };

  if (!stepDefinition) {
    return null;
  }

  const hasParameters = stepDefinition.parameters.length > 0;

  // Standard focus visible styling
  const buttonFocusStyle = "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 dark:focus-visible:ring-offset-gray-950";

  return (
    <Dialog open={isOpen} onOpenChange={(open) => !open && onClose()}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Configure: {nodeLabel || 'Pipeline Step'}</DialogTitle>
          <DialogDescription>
            {stepDefinition.description || 'Set the parameters for this step.'}
          </DialogDescription>
        </DialogHeader>
        <div className="grid gap-4 py-4">
          {!hasParameters ? (
            <p className="text-sm text-muted-foreground">This step has no configurable parameters.</p>
          ) : (
            stepDefinition.parameters.map((param: ParameterDefinition) => (
              <div key={param.name} className="grid grid-cols-4 items-center gap-4">
                <Label htmlFor={param.name} className="text-right col-span-1">
                  {param.label || param.name}
                  {param.required && <span className="text-red-500 ml-1">*</span>}
                </Label>
                {/* TODO: Render different input types based on param.type */}
                <Input
                  id={param.name}
                  value={currentParams[param.name] ?? ''}
                  onChange={(e) => handleInputChange(param.name, e.target.value)}
                  placeholder={param.description || `Enter ${param.label || param.name}`}
                  type={param.type === 'number' ? 'number' : 'text'}
                  className="col-span-3"
                  required={param.required}
                />
              </div>
            ))
          )}
        </div>
        <DialogFooter>
          <DialogClose asChild>
            <Button type="button" className={cn(buttonFocusStyle)}>Cancel</Button>
          </DialogClose>
           {/* Revert to default variant, rely on theme + focus style */}
          <Button
             type="button"
             onClick={handleSave}
             disabled={!hasParameters}
             className={cn(buttonFocusStyle)} // Only apply consistent focus
           >
             Save Parameters
           </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

export default ParameterEditDialog;