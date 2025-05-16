import React, { useState, useEffect } from 'react';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
  DialogClose,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Label } from '@/components/ui/label';
import { Input } from '@/components/ui/input';
import { AlertTriangle, Loader2 } from 'lucide-react'; // Added Loader2
import { cn } from '@/lib/utils';

interface DestructiveActionDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onConfirm: () => void;
  title: string;
  description: React.ReactNode;
  confirmationText: string;
  confirmButtonText?: string;
  isConfirming?: boolean; // Added prop to disable confirm button during action
}

const DestructiveActionDialog: React.FC<DestructiveActionDialogProps> = ({
  isOpen,
  onClose,
  onConfirm,
  title,
  description,
  confirmationText,
  confirmButtonText = 'confirm',
  isConfirming = false, // Default isConfirming to false
}) => {
  const [inputText, setInputText] = useState('');
  const isMatch = inputText === confirmationText;

  useEffect(() => {
    if (!isOpen) {
      // Reset input when dialog closes
      setTimeout(() => setInputText(''), 150);
    }
  }, [isOpen]);

  const handleConfirm = () => {
    // Only call confirm if not already confirming and text matches
    if (isMatch && !isConfirming) {
      onConfirm();
      // Don't close automatically here, let the parent component handle closing after the async action
      // onClose();
    }
  };

  const buttonFocusStyle =
    'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 dark:focus-visible:ring-offset-gray-950';

  return (
    <Dialog open={isOpen} onOpenChange={(open) => !open && onClose()}>
      <DialogContent className="sm:max-w-[480px]">
        <DialogHeader className="flex flex-row items-center gap-3">
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
            To confirm, type "
            <span className="text-destructive font-bold">
              {confirmationText}
            </span>
            " in the box below:
          </Label>
          <Input
            id="confirmation-input"
            value={inputText}
            onChange={(e) => setInputText(e.target.value)}
            placeholder={confirmationText}
            aria-invalid={!isMatch && inputText.length > 0}
            className={cn(
              isMatch && 'border-green-500 focus-visible:ring-green-500/50'
            )}
            disabled={isConfirming} // Disable input while confirming
          />
        </div>
        <DialogFooter>
          <DialogClose>
            {/* Disable Cancel button while confirming */}
            <Button
              type="button"
              variant="outline"
              className={cn(buttonFocusStyle)}
              disabled={isConfirming}
            >
              Cancel
            </Button>
          </DialogClose>
          <Button
            type="button"
            variant="destructive"
            onClick={handleConfirm}
            // Disable if text doesn't match OR if confirming is in progress
            disabled={!isMatch || isConfirming}
            className={cn(
              buttonFocusStyle,
              // Only apply green style if matched AND not confirming
              isMatch &&
                !isConfirming &&
                'bg-green-600 hover:bg-green-700 border-green-600'
            )}
          >
            {/* Show loader when confirming */}
            {isConfirming && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
            {confirmButtonText}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

export default DestructiveActionDialog;
