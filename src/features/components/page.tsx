import React from "react";

import { StackExample, StatExample, TextExample } from "./examples/";

const Page: React.FC = () => (
  <div className="text-zinc-700 w-screen pl-12 pr-4 pt-2 pb-4 flex flex-col gap-4">
    <p className="text-2xl font-bold h-[32px] align-middle my-auto">
      Components
    </p>
    <StackExample />
    <StatExample />
    <TextExample />
  </div>
);

export default Page;
