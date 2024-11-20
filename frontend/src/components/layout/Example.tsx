import React from "react";

interface ExampleProps {
  content?: React.ReactNode;
  controls?: React.ReactNode;
};

const Example: React.FC<ExampleProps> = ({ content, controls }: ExampleProps) => (
  <div className="bg-neutral-200 flex flex-row border border-neutral-700 rounded-md shadow-md">
    <div className="flex-auto py-2 px-4">
      {content}
    </div>
    <div className="flex-2 border-l border-l-neutral-700 min-w-[480px] py-4 px-4">
      <h1 className="text-2xl text-neutral-800 font-bold">Controls</h1>
      {controls}
    </div>
  </div>
);

export default Example;
