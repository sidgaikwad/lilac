import { isRouteErrorResponse, useRouteError } from 'react-router-dom';
import { isApiError } from '@/types';

function ErrorBoundary() {
  const error = useRouteError();
  console.error(error);

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
  } else if (isApiError(error)) {
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
