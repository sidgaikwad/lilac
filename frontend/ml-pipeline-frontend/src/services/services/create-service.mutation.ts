import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ApiError } from '@/types';
import { postHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';

export interface CreateServiceRequest {
  service: 'mlflow';
  service_name: string;
}

export interface CreateServiceResponse {
  service_id: string;
}

async function createService(
  projectId: string,
  payload: CreateServiceRequest
): Promise<CreateServiceResponse> {
  return postHttp<Sn<CreateServiceRequest>, Sn<CreateServiceResponse>>(
    `/projects/${projectId}/services`,
    payload
  );
}

export interface UseCreateServiceProps {
  onSuccess?: (data: CreateServiceResponse) => void;
  onError?: (error: ApiError) => void;
}

export function useCreateService(projectId: string, props: UseCreateServiceProps) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationKey: [QueryKeys.CREATE_SERVICE],
    mutationFn: (payload: CreateServiceRequest) => createService(projectId, payload),
    onSuccess: (data) => {
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.LIST_SERVICES, projectId],
      });
      if (props.onSuccess !== undefined) {
        props.onSuccess(data);
      }
    },
    onError: props.onError,
  });
}