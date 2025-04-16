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
import { AlertTriangle } from 'lucide-react';
import { cn } from '@/lib/utils';

interface DestructiveActionDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onConfirm: () => void;
  title: string;
  description: React.ReactNode;
  confirmationText: string;
  confirmButtonText?: string;
}

const DestructiveActionDialog: React.FC<DestructiveActionDialogProps> = ({
  isOpen,
  onClose,
  onConfirm,
  title,
  description,
  confirmationText,
  confirmButtonText = "Confirm",
}) => {
  const [inputText, setInputText] = useState("");
  const isMatch = inputText === confirmationText;

  useEffect(() => {
    if (!isOpen) {
      setTimeout(() => setInputText(""), 150);
    }
  }, [isOpen]);

  const handleConfirm = () => {
    if (isMatch) {
      onConfirm();
      onClose();
    }
  };

  const buttonFocusStyle = "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 dark:focus-visible:ring-offset-gray-950";

  return (
    <Dialog open={isOpen} onOpenChange={(open) => !open && onClose()}>
      <DialogContent className="sm:max-w-[480px]">
        <DialogHeader className="flex flex-row items-center gap-3">
          {/* Use theme destructive color for icon and background */}
          <div className="p-2 bg-destructive/10 rounded-full">
             <AlertTriangle className="h-6 w-6 text-destructive" />
          </div>
          <div>
            <DialogTitle className="text-lg">{title}</DialogTitle>
            <DialogDescription className="text-sm text-muted-foreground">
              {description}
            </DialogDescription>
          </div>
        </DialogHeader>
        <div className="py-4 space-y-2">
           <Label htmlFor="confirmation-input" className="font-semibold">
             To confirm, type "<span className="text-destructive font-bold">{confirmationText}</span>" in the box below:
           </Label>
           <Input
             id="confirmation-input"
             value={inputText}
             onChange={(e) => setInputText(e.target.value)}
             placeholder={confirmationText}
             // Use aria-invalid which applies border-destructive via theme/input component styles
             aria-invalid={!isMatch && inputText.length > 0}
             className={cn(
                // Only add green border manually when matched
                isMatch && "border-green-500 focus-visible:ring-green-500/50"
             )}
           />
        </div>
        <DialogFooter>
          <DialogClose asChild>
            <Button type="button" variant="outline" className={cn(buttonFocusStyle)}>Cancel</Button>
          </DialogClose>
          <Button
            type="button"
            variant="destructive" // Use destructive variant from theme
            onClick={handleConfirm}
            disabled={!isMatch}
            className={cn(
                buttonFocusStyle,
                // Override background only when enabled and matched
                isMatch && "bg-green-600 hover:bg-green-700 border-green-600"
            )}
          >
            {confirmButtonText}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

export default DestructiveActionDialog;