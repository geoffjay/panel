import React from "react";

import Example from "./example";
import { MDXComponents } from "./mdx-components";

import Range from "./range.mdx";

const RangeExample: React.FC = () => {
  // const [direction, setDirection] = useState("vertical");

  // const handleDirection = (value: string) => {
  //   setDirection(value);
  // };

  const Controls = () => (
    <div className="flex flex-col gap-4">
      {/* <Example.TextInputControl
        label="Direction"
        options={directionOptions}
        value={direction}
        onChange={handleDirection}
      /> */}
    </div>
  );

  return (
    <Example
      content={<Range components={MDXComponents} defaultValue={50} />}
      controls={<Controls />}
    />
  );
};

export default RangeExample;
