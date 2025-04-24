// import { useQuery } from "@tanstack/react-query";
// import { QueryKeys } from "./constants";
// import apiClient from "../apiClient";

// export interface UsePipelinesProps {
//     pipelineId: string,
// }

// export function usePipelines(props: UsePipelinesProps) {
//     const query = useQuery({
//         queryKey: [QueryKeys.GET_PIPELINE, props.pipelineId],
//         queryFn: () => {
//             apiClient.get<Pipeline>(`/pipeline/${props.pipelineId}`);
//         }
//     });

    
//     return {
//         pipelines:
//     }
// }