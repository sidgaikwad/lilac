import useAuthStore from '@/store/use-auth-store';
import { BASE_URL } from '@/services/constants';

function handleUnauthorized() {
  useAuthStore.getState().clearToken();
  window.location.href = '/login';
}

export async function postHttp<Req, Resp>(
  path: string,
  request: Req
): Promise<Resp> {
  if (path[0] !== '/') {
    path = `/${path}`;
  }
  const token = useAuthStore.getState().token;
  const headers: HeadersInit = {
    'Content-Type': 'application/json',
  };
  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }
  const resp = await fetch(`${BASE_URL}${path}`, {
    method: 'POST',
    headers,
    body: JSON.stringify(request),
  });
  if (!resp.ok) {
    if (resp.status === 401) {
      handleUnauthorized();
    }
    return Promise.reject({
      statusCode: resp.status,
      ...(await resp.json()),
    });
  }
  if (resp.headers.get('Content-Type') === 'application/json') {
    return await resp.json();
  }
  return {} as Resp;
}

export async function getHttp<Resp>(
  path: string,
  params?: Record<string, string>
): Promise<Resp>;
export async function getHttp<Req extends Record<string, string>, Resp>(
  path: string,
  params?: Req
): Promise<Resp> {
  if (path[0] !== '/') {
    path = `/${path}`;
  }
  const searchParams = new URLSearchParams();
  Object.entries(params ?? {}).forEach(([key, value]) =>
    searchParams.append(key, value)
  );
  const token = useAuthStore.getState().token;
  const headers: HeadersInit = {};
  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }
  const resp = await fetch(`${BASE_URL}${path}?${searchParams.toString()}`, {
    method: 'GET',
    headers,
  });
  if (!resp.ok) {
    if (resp.status === 401) {
      handleUnauthorized();
    }
    return Promise.reject({
      statusCode: resp.status,
      ...(await resp.json()),
    });
  }
  return await resp.json();
}

export async function deleteHttp<Resp = void>(path: string): Promise<Resp> {
  if (path[0] !== '/') {
    path = `/${path}`;
  }
  const token = useAuthStore.getState().token;
  const headers: HeadersInit = {};
  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }
  const resp = await fetch(`${BASE_URL}${path}`, {
    method: 'DELETE',
    headers,
  });

  if (!resp.ok) {
    if (resp.status === 401) {
      handleUnauthorized();
    }
    const errorBody = await resp.json().catch(() => ({
      error: resp.statusText || 'Delete operation failed',
    }));
    return Promise.reject({
      statusCode: resp.status,
      error: errorBody.error,
    });
  }

  if (resp.status === 204 || resp.headers.get('Content-Length') === '0') {
    return Promise.resolve({} as Resp);
  }

  if (resp.headers.get('Content-Type')?.includes('application/json')) {
    return await resp.json();
  }

  return Promise.resolve({} as Resp);
}
