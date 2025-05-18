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
import { RJSFSchema } from '@rjsf/utils';
import { Separator } from '@/components/ui/separator';
import useReactFlowStore, { RFState } from '@/store/use-react-flow-store';
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

const PipelineNode: React.FC<NodeProps<PipelineNodeType>> = (
  props: NodeProps<PipelineNodeType>
) => {
  const { updateParameters } = useReactFlowStore(selector(props.id), shallow);
  const parameters = useRef<object>({ ...props.data.parameters });
  const [isOpen, setIsOpen] = React.useState(false);
  const [isError, setIsError] = React.useState(false);
  return (
    <BaseNode
      selected={props.selected}
      data-state={isError ? 'invalid' : 'valid'}
      className="w-64 px-3 py-2 data-[state=invalid]:border-red-500 data-[state=invalid]:ring-red-500"
    >
      <NodeHeader className="-mx-3 -mt-2 border-b">
        <NodeHeaderIcon>
          <Cog />
        </NodeHeaderIcon>
        <NodeHeaderTitle>{props.data.stepDefinition.name}</NodeHeaderTitle>
        <NodeHeaderActions>
          <NodeHeaderDeleteAction />
        </NodeHeaderActions>
      </NodeHeader>
      <div className="-mx-3 flex justify-between py-5">
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
      {Object.keys(
        (props.data.stepDefinition.schema as RJSFSchema).properties ?? {}
      ).length > 0 && (
        <>
          <Separator />
          <div className="m-2">
            <Collapsible open={isOpen} onOpenChange={setIsOpen}>
              <CollapsibleTrigger className="justify-left flex w-full flex-nowrap">
                {isOpen ? <ChevronDown /> : <ChevronRight />}
                <span
                  className={cn(
                    'pl-1 transition-opacity',
                    isOpen && 'pb-4 opacity-60'
                  )}
                >
                  Input parameters
                </span>
              </CollapsibleTrigger>
              <CollapsibleContent>
                <Form
                  liveValidate
                  showErrorList={false}
                  className="space-y-2"
                  uiSchema={{
                    'ui:submitButtonOptions': {
                      norender: true,
                    },
                  }}
                  schema={props.data.stepDefinition.schema as RJSFSchema}
                  validator={validator}
                  formData={props.data.parameters}
                  onChange={(e) => {
                    parameters.current = e.formData ?? {};
                    if (
                      validator.isValid(
                        props.data.stepDefinition.schema as RJSFSchema,
                        parameters.current,
                        props.data.stepDefinition.schema as RJSFSchema
                      )
                    ) {
                      setIsError(false);
                      updateParameters(parameters.current);
                    } else {
                      setIsError(true);
                    }
                  }}
                  onSubmit={() => {
                    updateParameters(parameters.current);
                    toast.success('Updated node parameters');
                  }}
                  onError={() =>
                    toast.error('Failed to update node parameters!')
                  }
                />
              </CollapsibleContent>
            </Collapsible>
          </div>
        </>
      )}
    </BaseNode>
  );
};

export default memo(PipelineNode);
