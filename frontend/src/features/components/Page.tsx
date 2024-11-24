import React from "react";

import { StackExample, StatExample, LineChartExample, TextExample } from "~components/layout";

const Page: React.FC = () => (
  <div className="text-neutral-700 w-screen px-8 py-2 flex flex-col gap-4">
    <StackExample />
    <StatExample />
    <TextExample />
    <LineChartExample />
  </div>
);

export default Page;
