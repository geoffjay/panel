import { Link } from "react-router";

import { buttonVariants } from "~components/ui/button";

const Page = () => {
  return (
    <div className="grid place-content-center px-4">
      <div className="text-center">
        <h1 className="text-9xl font-black text-zinc-200">404</h1>
        <p className="text-2xl font-bold tracking-tight text-zinc-900 sm:text-4xl">Uh-oh!</p>
        <p className="my-4 text-zinc-500">We can&apos;t find that page.</p>
        <Link to="/" className={buttonVariants({ variant: "outline" })}>
          Go Back Home
        </Link>
      </div>
    </div>
  );
};

export default Page;
