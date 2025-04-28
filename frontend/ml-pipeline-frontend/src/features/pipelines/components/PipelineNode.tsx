import React, { memo, useRef } from 'react';
import { Node, Position, NodeProps } from '@xyflow/react';
import { StepDefinition } from '@/types';
import { LabeledHandle } from '@/components/labeled-handle';
import {
  NodeHeader,
  NodeHeaderIcon,
  NodeHeaderTitle,
  NodeHeaderActions,
  NodeHeaderDeleteAction,
} from '@/components/node-header';
import { ChevronDown, ChevronRight, Cog } from 'lucide-react';
import { BaseNode } from '@/components/base-node';
import Form from '@rjsf/shadcn';
import validator from '@rjsf/validator-ajv8';
import {
  getSubmitButtonOptions,
  RJSFSchema,
  SubmitButtonProps,
} from '@rjsf/utils';
import { Button, Separator } from '@/components/ui';
import useReactFlowStore, { RFState } from '@/store/useReactFlowStore';
import { shallow } from 'zustand/shallow';
import { toast } from 'sonner';
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from '@/components/ui/collapsible';
import { cn } from '@/lib/utils';

type PipelineNodeData = {
  parameters: Record<string, string | number | boolean | object>;
  stepDefinition: StepDefinition;
};

export type PipelineNodeType = Node<PipelineNodeData, 'pipelineNode'>;

const selector = (nodeId: string) => (store: RFState) => ({
  updateParameters: (parameters: object) =>
    store.updateNode(nodeId, parameters),
});

function SubmitButton(props: SubmitButtonProps) {
  const { uiSchema } = props;
  const { norender } = getSubmitButtonOptions(uiSchema);
  if (norender) {
    return null;
  }
  return (
    <div className="flex justify-end pt-2">
      <Button>Save</Button>
    </div>
  );
}

const PipelineNode: React.FC<NodeProps<PipelineNodeType>> = (
  props: NodeProps<PipelineNodeType>
) => {
  const { updateParameters } = useReactFlowStore(selector(props.id), shallow);
  const parameters = useRef<object>({ ...props.data.parameters });
  const [isOpen, setIsOpen] = React.useState(false);
  return (
    // Use theme variables for card background and border
    // Use primary color for selection border/ring
    <BaseNode selected={props.selected} className="w-64 px-3 py-2">
      <NodeHeader className="-mx-3 -mt-2 border-b">
        <NodeHeaderIcon>
          <Cog />
        </NodeHeaderIcon>
        <NodeHeaderTitle>{props.data.stepDefinition.name}</NodeHeaderTitle>
        <NodeHeaderActions>
          {/* <NodeHeaderMenuAction label="Expand account options">
            <DropdownMenuLabel>Edit Node</DropdownMenuLabel>
            <DropdownMenuSeparator />
            <DropdownMenuItem>Profile</DropdownMenuItem>
            <DropdownMenuItem>Billing</DropdownMenuItem>
            <DropdownMenuItem>Team</DropdownMenuItem>
            <DropdownMenuItem>Subscription</DropdownMenuItem>
          </NodeHeaderMenuAction> */}
          <NodeHeaderDeleteAction />
        </NodeHeaderActions>
      </NodeHeader>
      <div className="flex justify-between -mx-3 py-5">
        {props.data.stepDefinition.inputs.map((input) => (
          <LabeledHandle
            id={input}
            key={`${props.id}-${input}`}
            title={input}
            type="target"
            position={Position.Left}
            handleClassName="bg-red-500"
          />
        ))}

        {props.data.stepDefinition.outputs.map((output) => (
          <LabeledHandle
            id={output}
            key={`${props.id}-${output}`}
            title={output}
            type="source"
            position={Position.Right}
          />
        ))}
      </div>
      <Separator />
      <div className="m-2">
        <Collapsible open={isOpen} onOpenChange={setIsOpen}>
          <CollapsibleTrigger className="w-full flex flex-nowrap justify-left">
            {isOpen ? <ChevronDown /> : <ChevronRight />}
            <span
              className={cn(
                'pl-1 transition-opacity',
                isOpen && 'pb-4 opacity-60 '
              )}
            >
              Node Parameters
            </span>
          </CollapsibleTrigger>
          <CollapsibleContent>
            <Form
              className="space-y-2"
              templates={{ ButtonTemplates: { SubmitButton } }}
              schema={props.data.stepDefinition.schema as RJSFSchema}
              validator={validator}
              formData={props.data.parameters}
              onChange={(e) => (parameters.current = e.formData ?? {})}
              onSubmit={() => {
                updateParameters(parameters.current);
                toast.success('Updated node parameters');
              }}
              onError={() => toast.error('Failed to update node parameters!')}
            />
          </CollapsibleContent>
        </Collapsible>
      </div>
    </BaseNode>
  );
};

export default memo(PipelineNode);
