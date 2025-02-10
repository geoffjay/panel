import React from "react";

import { cn } from "~lib/utils";

import {
  RangeExample,
  StackExample,
  StatExample,
  TextExample,
} from "./examples/";

const Page: React.FC = () => (
  <div
    className={cn(
      "w-screen pl-12 pr-4 pt-2 pb-4 flex flex-col gap-4",
      "text-zinc-700 dark:text-zinc-200",
    )}
  >
    <p className="text-2xl font-bold h-[32px] align-middle my-auto">
      Components
    </p>
    <RangeExample />
    <StackExample />
    <StatExample />
    <TextExample />
  </div>
);

export default Page;
