import React from "react";

interface TextProps {
  children?: React.ReactNode;
}

const Text: React.FC<TextProps> = ({ children }: TextProps) => (
  <p className="text text-zinc-700">{children}</p>
);

export default Text;
