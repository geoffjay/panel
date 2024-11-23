import React from "react";

import { StackExample, StatExample, LineChartExample } from "~components/layout";

const Page: React.FC = () => (
  <div className="text-neutral-700 w-screen px-8 py-2 flex flex-col gap-4">
    <StackExample />
    <StatExample />
    <LineChartExample />
  </div>
);

export default Page;
