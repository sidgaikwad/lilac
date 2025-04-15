    // src/components/ParameterEditDialog.tsx
    import React from 'react';
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
    import { Input } from "@/components/ui/input";
    import { Label } from "@/components/ui/label";
    import { Node } from 'reactflow';
    import { cn } from '@/lib/utils'; // Import cn

    interface ParameterEditDialogProps {
        isOpen: boolean;
        node: Node | null;
        paramValues: Record<string, any>;
        onClose: () => void;
        onSave: () => void;
        onChange: (paramName: string, value: string | number) => void;
    }

    const ParameterEditDialog: React.FC<ParameterEditDialogProps> = ({
        isOpen,
        node,
        paramValues,
        onClose,
        onSave,
        onChange
    }) => {
        return (
            <Dialog open={isOpen} onOpenChange={(open) => { if (!open) onClose(); }}>
                <DialogContent className="sm:max-w-[425px]">
                    {node && (
                        <>
                            <DialogHeader>
                                <DialogTitle>Configure: {node.data?.label}</DialogTitle>
                                <DialogDescription>
                                    Adjust the parameters for this pipeline step.
                                </DialogDescription>
                            </DialogHeader>
                            <div className="grid gap-4 py-4">
                                {Object.entries(paramValues).map(([key, value]) => (
                                    <div key={key} className="grid grid-cols-4 items-center gap-4">
                                        <Label htmlFor={key} className="text-right">
                                            {key.charAt(0).toUpperCase() + key.slice(1)}
                                        </Label>
                                        <Input
                                            id={key}
                                            name={key}
                                            value={value ?? ''}
                                            onChange={(e) => onChange(key, e.target.value)}
                                            className="col-span-3"
                                            type={typeof node.data?.parameters?.[key] === 'number' ? 'number' : 'text'}
                                            step={typeof node.data?.parameters?.[key] === 'number' && !Number.isInteger(node.data?.parameters?.[key]) ? "any" : undefined}
                                        />
                                    </div>
                                ))}
                                {Object.keys(paramValues).length === 0 && (
                                    <p className="text-sm text-muted-foreground col-span-4 text-center">No parameters defined for this node.</p>
                                )}
                            </div>
                            <DialogFooter>
                                <DialogClose asChild>
                                    {/* Explicit hover style for secondary button */}
                                    <Button
                                        type="button"
                                        variant="secondary"
                                        className="hover:bg-gray-200" 
                                        >
                                        Cancel
                                    </Button>
                                </DialogClose>
                                {/* Explicit hover style for primary button */}
                                <Button
                                    type="button"
                                    onClick={onSave}
                                    className="hover:bg-gray-200" 
                                >
                                    Save changes
                                </Button>
                            </DialogFooter>
                        </>
                    )}
                </DialogContent>
            </Dialog>
        );
    };

    export default ParameterEditDialog;
    