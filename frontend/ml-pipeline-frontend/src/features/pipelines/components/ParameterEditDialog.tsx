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
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"; // Import Select
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

  // Use string for select value compatibility
  const handleValueChange = (paramName: string, value: string) => {
    const paramDef = stepDefinition?.parameters.find(p => p.name === paramName);
    let finalValue: string | number | boolean = value;

    if (paramDef?.type === 'number') {
      finalValue = parseFloat(value) || 0;
    } else if (paramDef?.type === 'boolean') {
      finalValue = value === 'true'; // Convert string "true" to boolean
    }
    // String/enum/s3_path can remain strings for now

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
  const buttonFocusStyle = "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 dark:focus-visible:ring-offset-gray-950";

  const renderInput = (param: ParameterDefinition) => {
    const currentValue = currentParams[param.name];

    switch (param.type) {
      case 'boolean':
        return (
          <Select
            value={currentValue === true ? 'true' : 'false'} // Control with string value
            onValueChange={(value) => handleValueChange(param.name, value)}
          >
            <SelectTrigger id={param.name} className="col-span-3">
              <SelectValue placeholder="Select true or false" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="true">True</SelectItem>
              <SelectItem value="false">False</SelectItem>
            </SelectContent>
          </Select>
        );
      case 'enum':
        return (
           <Select
            value={currentValue ?? ''}
            onValueChange={(value) => handleValueChange(param.name, value)}
          >
            <SelectTrigger id={param.name} className="col-span-3">
              <SelectValue placeholder={`Select ${param.label || param.name}`} />
            </SelectTrigger>
            <SelectContent>
              {(param.options || []).map(option => (
                 <SelectItem key={option} value={option}>{option}</SelectItem>
              ))}
            </SelectContent>
          </Select>
        );
      case 'number':
         return (
            <Input
              id={param.name}
              value={currentValue ?? ''}
              onChange={(e) => handleValueChange(param.name, e.target.value)}
              placeholder={param.description || `Enter ${param.label || param.name}`}
              type="number"
              className="col-span-3"
              required={param.required}
            />
         );
      case 'string':
      case 's3_path': // Treat s3_path as string for now
      default:
        return (
          <Input
            id={param.name}
            value={currentValue ?? ''}
            onChange={(e) => handleValueChange(param.name, e.target.value)}
            placeholder={param.description || `Enter ${param.label || param.name}`}
            type="text"
            className="col-span-3"
            required={param.required}
          />
        );
    }
  };


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
            stepDefinition.parameters.map((param) => (
              <div key={param.name} className="grid grid-cols-4 items-center gap-4">
                <Label htmlFor={param.name} className="text-right col-span-1">
                  {param.label || param.name}
                  {param.required && <span className="text-red-500 ml-1">*</span>}
                </Label>
                {renderInput(param)}
              </div>
            ))
          )}
        </div>
        <DialogFooter>
          <DialogClose asChild>
            <Button type="button" variant="outline" className={cn(buttonFocusStyle)}>Cancel</Button>
          </DialogClose>
          <Button
             type="button"
             onClick={handleSave}
             disabled={!hasParameters}
             className={cn(
                buttonFocusStyle,
                // Override background only when enabled and matched (for delete dialog consistency)
                // This might not be needed here if default variant looks okay with theme
                // isMatch && "bg-green-600 hover:bg-green-700 border-green-600"
             )}
           >
             Save Parameters
           </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

export default ParameterEditDialog;