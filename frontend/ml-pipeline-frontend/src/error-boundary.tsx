import { isRouteErrorResponse, useNavigate, useRouteError } from 'react-router-dom';
import { isServiceError } from '@/types';

function ErrorBoundary() {
  const error = useRouteError();
  console.error(error);
  const navigate = useNavigate();

  if (isRouteErrorResponse(error)) {
    return (
      <div>
        <h1>Oops! Something went wrong.</h1>
        <p>{error.statusText}</p>
      </div>
    );
  } else if (error instanceof Error) {
    return (
      <div>
        <h1>Oops! Something went wrong.</h1>
        <p>{error.message}</p>
      </div>
    );
  } else if (isServiceError(error)) {
    if (error.statusCode === 401 && error.error === 'Invalid token') {
      navigate('/login');
    }
    return (
      <div>
        <h1>Oops! Something went wrong.</h1>
        <p>{error.error}</p>
      </div>
    );
  }

  return (
    <div>
      <h1>Oops! Something went wrong.</h1>
    </div>
  );
}

export default ErrorBoundary;
