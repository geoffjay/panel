import React, { useState } from "react";

import Example from "./example";
import { MDXComponents } from "./mdx-components";

import Stack from "./stack.mdx";

const StackExample: React.FC = () => {
  const [direction, setDirection] = useState("vertical");
  const directionOptions = [
    { value: "vertical", label: "Vertical", selected: true },
    { value: "horizontal", label: "Horizontal" },
  ];

  const handleDirection = (value: string) => {
    setDirection(value);
  };

  const Controls = () => (
    <div className="flex flex-col gap-4">
      <Example.SelectControl
        label="Direction"
        options={directionOptions}
        value={direction}
        onChange={handleDirection}
      />
    </div>
  );

  return (
    <Example
      content={
        <Stack {...{ components: MDXComponents, direction: direction }} />
      }
      controls={<Controls />}
    />
  );
};

export default StackExample;
