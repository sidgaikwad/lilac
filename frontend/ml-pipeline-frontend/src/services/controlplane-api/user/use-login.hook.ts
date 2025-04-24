import { useMutation } from "@tanstack/react-query";
import { QueryKeys } from "../constants";
import { AuthToken } from "@/types/api/auth";
import { useNavigate } from "react-router-dom";
import { toast } from "sonner";

export interface UseAuthProps {
    redirectTo?: string;
}

export function useLogin(props: UseAuthProps) {
    const navigate = useNavigate();
    return useMutation({
        mutationKey: [QueryKeys.LOGIN],
        mutationFn: async (input: {email: string, password: string}): Promise<AuthToken> => {
            const resp = await fetch(`http://localhost:3000/auth/login`, {
                method: 'POST',
                body: JSON.stringify({
                    email: input.email,
                    password: input.password,
                }),
                headers: {
                    'Content-Type': 'application/json',
                }
            });
            if (resp.status >= 300) {
                const body = await resp.json();
                console.log(body);
                return Promise.reject({
                    error: resp.status,
                    message: body.error,
                });
            }
            return await resp.json();
        },
        onSuccess: (data) => {
            localStorage.setItem('token', data.access_token);
            if (props.redirectTo) {
                navigate(props.redirectTo);
            }
        },
        onError: (error) => {
            toast.error("Login Failed", { description: `${error.statusCode} ${error.message}` });
        }
    });
}