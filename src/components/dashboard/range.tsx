import React from "react";

import { cn } from "~lib/utils";
import { Slider } from "~components/ui/slider";

interface RangeProps {
  className?: string;
  defaultValue?: number;
  min?: number;
  max?: number;
}

const Range: React.FC<RangeProps> = ({
  className,
  defaultValue = 0,
  min = 0,
  max = 100,
}: RangeProps) => {
  return (
    <Slider
      defaultValue={[defaultValue]}
      max={max}
      min={min}
      step={1}
      className={cn("w-[60%]", className)}
    />
  );
};

export default Range;
