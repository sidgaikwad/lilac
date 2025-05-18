import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from '@/components/ui/card';
import { Button } from '@/components/ui/button';

interface EmptyCardSectionProps {
  title: string;
  description?: string;
  buttonText: string;
  buttonDisabled?: boolean;
  onClick: () => void;
}

function EmptyCardSection(props: EmptyCardSectionProps) {
  return (
    <Card
      className="border-input w-full flex-1 basis-full border-dashed text-center"
      key={'empty-card-section'}
    >
      <CardHeader>
        <CardTitle>{props.title}</CardTitle>
        <CardDescription>{props.description}</CardDescription>
      </CardHeader>
      <CardContent>
        <Button
          onClick={props.onClick}
          disabled={props.buttonDisabled}
          variant="default"
        >
          {props.buttonText}
        </Button>
      </CardContent>
    </Card>
  );
}

export default EmptyCardSection;
