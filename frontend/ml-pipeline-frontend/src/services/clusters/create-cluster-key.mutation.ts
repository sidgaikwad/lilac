import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ServiceError } from '@/types';
import { postHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';
import { camelCaseObject } from '@/lib/utils';

export interface CreateClusterKeyRequest {
  clusterId: string;
}

export interface CreateClusterKeyResponse {
  id: string;
  prefix: string;
  key: string;
  createdAt: string;
}

async function createClusterKey(
  payload: CreateClusterKeyRequest
): Promise<CreateClusterKeyResponse> {
  const resp = await postHttp<object, Sn<CreateClusterKeyResponse>>(
    `/clusters/${payload.clusterId}/api-keys`,
    {}
  );
  return camelCaseObject(resp);
}

export interface UseCreateClusterKeyProps {
  onSuccess?: (data: CreateClusterKeyResponse) => void;
  onError?: (error: ServiceError) => void;
}

export function useCreateClusterKey(props?: UseCreateClusterKeyProps) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationKey: [QueryKeys.CREATE_CLUSTER_KEY],
    mutationFn: createClusterKey,
    onSuccess: (data) => {
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.GET_CLUSTER_KEYS],
      });
      if (props?.onSuccess !== undefined) {
        props.onSuccess(data);
      }
    },
    onError: props?.onError,
  });
}
