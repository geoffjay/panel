import React from "react";

interface StackProps {
  direction: string;
  children?: React.ReactNode;
}

const Stack: React.FC<StackProps> = ({
  direction = "vertical",
  children,
}: StackProps) => {
  const classes = direction === "vertical" ? "flex flex-col" : "flex flex-row";

  return <div className={classes}>{children}</div>;
};

export default Stack;
