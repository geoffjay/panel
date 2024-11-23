import React, { useState } from "react";

import Example from "./Example";
import { MDXComponents } from "./MDXComponents";

import LineChart from "./LineChart.mdx";

const LineChartExample: React.FC = () => {
  const [marginTop, setMarginTop] = useState(20);
  const [marginRight, setMarginRight] = useState(20);
  const [marginBottom, setMarginBottom] = useState(40);
  const [marginLeft, setMarginLeft] = useState(40);

  const handleMarginTop = (event: React.FormEvent<HTMLInputElement>) => {
    setMarginTop(Number(event.currentTarget.value));
  };

  const handleMarginRight = (event: React.FormEvent<HTMLInputElement>) => {
    setMarginRight(Number(event.currentTarget.value));
  };

  const handleMarginBottom = (event: React.FormEvent<HTMLInputElement>) => {
    setMarginBottom(Number(event.currentTarget.value));
  };

  const handleMarginLeft = (event: React.FormEvent<HTMLInputElement>) => {
    setMarginLeft(Number(event.currentTarget.value));
  };

  const Controls = () => (
    <div className="flex flex-col gap-4">
      <Example.RangeControl id="margin-top" label="Margin Top" onChange={handleMarginTop} value={marginTop} />
      <Example.RangeControl id="margin-right" label="Margin Right" onChange={handleMarginRight} value={marginRight} />
      <Example.RangeControl id="margin-bottom" label="Margin Bottom" onChange={handleMarginBottom} value={marginBottom} />
      <Example.RangeControl id="margin-left" label="Margin Left" onChange={handleMarginLeft} value={marginLeft} />
    </div>
  );

  const lineChartProps = {
    marginTop,
    marginRight,
    marginBottom,
    marginLeft,
  };

  return (
    <Example
      content={<LineChart components={MDXComponents} {...lineChartProps} />}
      controls={< Controls />}
    />
  );
};

export default LineChartExample;
