import * as React from 'react';
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { TriangleAlert } from 'lucide-react';
import { Spinner } from './spinner/spinner';

interface DestructiveActionConfirmationModalProps {
  dialogTrigger: React.ReactNode;
  onConfirm: () => void;
  itemName: string;
  itemType: string;
  confirmationText?: string;
  isLoading?: boolean;
}

function DestructiveActionConfirmationModal({
  dialogTrigger,
  onConfirm,
  itemName,
  itemType,
  confirmationText = 'delete',
  isLoading = false,
}: DestructiveActionConfirmationModalProps) {
  const [typedInput, setTypedInput] = React.useState('');

  const handleConfirm = () => {
    if (typedInput === confirmationText) {
      onConfirm();
    }
  };

  const handleCancel = () => {
    setTypedInput('');
  };

  return (
    <Dialog>
      <DialogTrigger asChild onClick={(event) => event.stopPropagation()}>
        {dialogTrigger}
      </DialogTrigger>
      <DialogContent className='border-destructive border-4 sm:max-w-[425px]'>
        <DialogHeader>
          <DialogTitle className='flex flex-row items-center gap-2'>
            <TriangleAlert className='text-destructive' />
            Are you absolutely sure?
          </DialogTitle>
          <DialogDescription>
            This action will permanently delete the {itemType} "{itemName}".
          </DialogDescription>
        </DialogHeader>
        <div className='grid gap-4 py-4'>
          <div className='grid grid-cols-1 items-center gap-2'>
            <Label htmlFor='confirmation-input'>
              <span>
                To confirm, please type "<strong>{confirmationText}</strong>" in
                the box below.
              </span>
            </Label>
            <Input
              id='confirmation-input'
              value={typedInput}
              onChange={(e) => setTypedInput(e.target.value)}
              placeholder={confirmationText}
              className={
                typedInput !== '' && typedInput !== confirmationText
                  ? 'border-destructive focus-visible:ring-destructive/50'
                  : ''
              }
              disabled={isLoading}
            />
          </div>
        </div>
        <DialogFooter>
          <DialogClose asChild onClick={(event) => event.stopPropagation()}>
            <div className='space-x-4'>
              <Button
                variant='outline'
                onClick={handleCancel}
                disabled={isLoading}
              >
                Cancel
              </Button>
              <Button
                variant='destructive'
                onClick={(event) => {
                  event.stopPropagation();
                  handleConfirm();
                }}
                disabled={typedInput !== confirmationText || isLoading}
              >
                {isLoading ? (
                  <>
                    <Spinner />
                    Deleting...
                  </>
                ) : (
                  'Confirm'
                )}
              </Button>
            </div>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

export default DestructiveActionConfirmationModal;
