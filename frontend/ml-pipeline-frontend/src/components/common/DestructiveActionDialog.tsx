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

  return (
    <Dialog open={isOpen} onOpenChange={(open) => !open && onClose()}>
      <DialogContent className="sm:max-w-[480px]">
        <DialogHeader className="flex flex-row items-center gap-3">
          <div className="p-2 bg-red-100 dark:bg-red-900/30 rounded-full"> {/* Adjusted icon background */}
             <AlertTriangle className="h-6 w-6 text-red-600 dark:text-red-400" /> {/* Adjusted icon color */}
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
             To confirm, type "<span className="text-red-600 dark:text-red-400 font-bold">{confirmationText}</span>" in the box below:
           </Label>
           <Input
             id="confirmation-input"
             value={inputText}
             onChange={(e) => setInputText(e.target.value)}
             placeholder={confirmationText}
             className={cn(
                "focus-visible:ring-offset-2",
                isMatch
                 ? "border-green-500 focus-visible:ring-green-500/50"
                 // Use explicit red border color
                 : "border-red-500 dark:border-red-600 focus-visible:ring-red-500/50 dark:focus-visible:ring-red-600/50"
             )}
           />
        </div>
        <DialogFooter>
          <DialogClose asChild>
            <Button type="button" variant="outline">Cancel</Button>
          </DialogClose>
          <Button
            type="button"
            onClick={handleConfirm}
            disabled={!isMatch}
            className={cn(
                "text-white",
                !isMatch
                 // Use solid red background when disabled, ensure opacity is 1
                 ? "bg-red-600/70 dark:bg-red-700/70 opacity-100 cursor-not-allowed" // Slightly dimmed solid red
                 : "bg-green-600 hover:bg-green-700" // Green when enabled
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