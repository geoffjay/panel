import React, { useState } from "react";

import Example from "./Example";
import { MDXComponents } from "./MDXComponents";

import Stack from "./Stack.mdx";

const StackExample: React.FC = () => {
  const [direction, setDirection] = useState("vertical");

  const handleDirection = (event: React.FormEvent<HTMLSelectElement>) => {
    setDirection(event.currentTarget.value);
  };

  const Controls = () => (
    <div>
      <div className="flex flex-row gap-4">
        <label className="text-neutral-800 my-auto">Direction:</label>
        <select className="select select-bordered bg-neutral-200 w-full max-w-xs" onChange={handleDirection} value={direction}>
          <option selected value="vertical">Vertical</option>
          <option value="horizontal">Horizontal</option>
        </select>
      </div>
    </div >
  );

  return (
    <Example
      content={<Stack components={MDXComponents} direction={direction} />}
      controls={<Controls />}
    />
  );
};

export default StackExample;
