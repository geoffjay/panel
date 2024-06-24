import { useState } from "react";
import { isRouteErrorResponse, useRouteError } from "react-router-dom";

const ErrorPage: React.FC = () => {
  const error = useRouteError();
  const [errorMessage, setErrorMessage] = useState<string>("");

  if (isRouteErrorResponse(error)) {
    setErrorMessage(error.statusText);
  } else if (error instanceof Error) {
    setErrorMessage(error.message);
  } else if (typeof error === "string") {
    setErrorMessage(error);
  } else {
    console.error(error);
    setErrorMessage("Unknown error");
  }

  return (
    <div id='error-page' className='flex flex-col gap-8 justify-center items-center h-screen'>
      <h1 className='text-4xl font-bold'>Oops!</h1>
      <p>Sorry, an unexpected error has occurred.</p>
      <p className='text-slate-400'>
        <i>{errorMessage}</i>
      </p>
    </div>
  );
};

export default ErrorPage;
