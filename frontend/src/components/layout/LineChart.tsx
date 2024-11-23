import React from "react";
import { ResponsiveLine } from "@nivo/line";

interface LineChartProps {
  className?: string;
  marginTop?: number;
  marginRight?: number;
  marginBottom?: number;
  marginLeft?: number;
};

const LineChart: React.FC<LineChartProps> = (props: LineChartProps) => {
  const {
    className,
    marginTop = 20,
    marginRight = 20,
    marginBottom = 40,
    marginLeft = 40,
  } = props;

  return (
    <div className={`${className} h-[320px]`}>
      <ResponsiveLine
        curve="monotoneX"
        data={[
          {
            id: "fake corp. A",
            data: [
              { x: 0, y: 7 },
              { x: 1, y: 5 },
              { x: 2, y: 11 },
              { x: 3, y: 9 },
              { x: 4, y: 13 },
              { x: 7, y: 16 },
              { x: 9, y: 12 },
            ],
          },
        ]}
        xScale={{
          type: "linear",
          min: 0,
          max: "auto",
        }}
        axisLeft={{
          legend: "linear scale",
          legendOffset: 12,
        }}
        axisBottom={{
          legend: "linear scale",
          legendOffset: -12,
        }}
        margin={{ top: marginTop, right: marginRight, bottom: marginBottom, left: marginLeft }}
        animate={true}
        enableTouchCrosshair={true}
        enableSlices="x"
      />
    </div >
  );
};

export default LineChart;
