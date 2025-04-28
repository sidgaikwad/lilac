import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
  Button,
} from '@/components/ui';

interface EmptyCardSectionProps {
  title: string;
  description?: string;
  buttonText: string;
  buttonDisabled?: boolean;
  onClick: () => void;
}

const EmptyCardSection: React.FC<EmptyCardSectionProps> = (
  props: EmptyCardSectionProps
) => {
  return (
    <Card
      className="w-full basis-full flex-1 border-dashed border-input text-center"
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
          variant="outline"
        >
          {props.buttonText}
        </Button>
      </CardContent>
    </Card>
  );
};

export default EmptyCardSection;
