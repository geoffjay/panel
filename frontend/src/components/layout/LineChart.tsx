import React from "react";
import { ResponsiveLine } from "@nivo/line";

const LineChart: React.FC = () => {
  const data = [
    {
      "id": "japan",
      "color": "hsl(152, 70%, 50%)",
      "data": [
        {
          "x": "plane",
          "y": 73,
        },
        {
          "x": "helicopter",
          "y": 99,
        },
        {
          "x": "boat",
          "y": 166,
        },
        {
          "x": "train",
          "y": 100,
        },
        {
          "x": "subway",
          "y": 137,
        },
        {
          "x": "bus",
          "y": 9,
        },
        {
          "x": "car",
          "y": 289,
        },
        {
          "x": "moto",
          "y": 68,
        },
        {
          "x": "bicycle",
          "y": 33,
        },
        {
          "x": "horse",
          "y": 37,
        },
        {
          "x": "skateboard",
          "y": 162,
        },
        {
          "x": "others",
          "y": 23,
        },
      ],
    },
    {
      "id": "france",
      "color": "hsl(177, 70%, 50%)",
      "data": [
        {
          "x": "plane",
          "y": 105,
        },
        {
          "x": "helicopter",
          "y": 276,
        },
        {
          "x": "boat",
          "y": 91,
        },
        {
          "x": "train",
          "y": 78,
        },
        {
          "x": "subway",
          "y": 65,
        },
        {
          "x": "bus",
          "y": 295,
        },
        {
          "x": "car",
          "y": 193,
        },
        {
          "x": "moto",
          "y": 173,
        },
        {
          "x": "bicycle",
          "y": 74,
        },
        {
          "x": "horse",
          "y": 160,
        },
        {
          "x": "skateboard",
          "y": 36,
        },
        {
          "x": "others",
          "y": 147,
        },
      ],
    },
    {
      "id": "us",
      "color": "hsl(318, 70%, 50%)",
      "data": [
        {
          "x": "plane",
          "y": 239,
        },
        {
          "x": "helicopter",
          "y": 99,
        },
        {
          "x": "boat",
          "y": 209,
        },
        {
          "x": "train",
          "y": 259,
        },
        {
          "x": "subway",
          "y": 55,
        },
        {
          "x": "bus",
          "y": 15,
        },
        {
          "x": "car",
          "y": 238,
        },
        {
          "x": "moto",
          "y": 299,
        },
        {
          "x": "bicycle",
          "y": 95,
        },
        {
          "x": "horse",
          "y": 233,
        },
        {
          "x": "skateboard",
          "y": 26,
        },
        {
          "x": "others",
          "y": 6,
        },
      ],
    },
    {
      "id": "germany",
      "color": "hsl(84, 70%, 50%)",
      "data": [
        {
          "x": "plane",
          "y": 134,
        },
        {
          "x": "helicopter",
          "y": 38,
        },
        {
          "x": "boat",
          "y": 92,
        },
        {
          "x": "train",
          "y": 192,
        },
        {
          "x": "subway",
          "y": 259,
        },
        {
          "x": "bus",
          "y": 190,
        },
        {
          "x": "car",
          "y": 33,
        },
        {
          "x": "moto",
          "y": 12,
        },
        {
          "x": "bicycle",
          "y": 45,
        },
        {
          "x": "horse",
          "y": 123,
        },
        {
          "x": "skateboard",
          "y": 234,
        },
        {
          "x": "others",
          "y": 148,
        },
      ],
    },
    {
      "id": "norway",
      "color": "hsl(353, 70%, 50%)",
      "data": [
        {
          "x": "plane",
          "y": 247,
        },
        {
          "x": "helicopter",
          "y": 48,
        },
        {
          "x": "boat",
          "y": 300,
        },
        {
          "x": "train",
          "y": 127,
        },
        {
          "x": "subway",
          "y": 23,
        },
        {
          "x": "bus",
          "y": 83,
        },
        {
          "x": "car",
          "y": 24,
        },
        {
          "x": "moto",
          "y": 234,
        },
        {
          "x": "bicycle",
          "y": 35,
        },
        {
          "x": "horse",
          "y": 186,
        },
        {
          "x": "skateboard",
          "y": 194,
        },
        {
          "x": "others",
          "y": 260,
        },
      ],
    },
  ];

  return (
    <ResponsiveLine
      data={data}
      margin={{ top: 50, right: 110, bottom: 50, left: 60 }}
      xScale={{ type: "point" }}
      yScale={{
        type: "linear",
        min: "auto",
        max: "auto",
        stacked: true,
        reverse: false,
      }}
      yFormat=" >-.2f"
      axisTop={null}
      axisRight={null}
      axisBottom={{
        tickSize: 5,
        tickPadding: 5,
        tickRotation: 0,
        legend: "transportation",
        legendOffset: 36,
        legendPosition: "middle",
        truncateTickAt: 0,
      }}
      axisLeft={{
        tickSize: 5,
        tickPadding: 5,
        tickRotation: 0,
        legend: "count",
        legendOffset: -40,
        legendPosition: "middle",
        truncateTickAt: 0,
      }}
      pointSize={10}
      pointColor={{ theme: "background" }}
      pointBorderWidth={2}
      pointBorderColor={{ from: "serieColor" }}
      pointLabel="data.yFormatted"
      pointLabelYOffset={-12}
      enableTouchCrosshair={true}
      useMesh={true}
      legends={[
        {
          anchor: "bottom-right",
          direction: "column",
          justify: false,
          translateX: 100,
          translateY: 0,
          itemsSpacing: 0,
          itemDirection: "left-to-right",
          itemWidth: 80,
          itemHeight: 20,
          itemOpacity: 0.75,
          symbolSize: 12,
          symbolShape: "circle",
          symbolBorderColor: "rgba(0, 0, 0, .5)",
          effects: [
            {
              on: "hover",
              style: {
                itemBackground: "rgba(0, 0, 0, .03)",
                itemOpacity: 1,
              },
            },
          ],
        },
      ]}
    />
  );
};

export default LineChart;
