import React from "react";

interface PanelProps {
  children?: React.ReactNode;
}

const Panel: React.FC<PanelProps> = ({ children }: PanelProps) => {
  return <div className="flex m-2">{children}</div>;
};

export default Panel;
