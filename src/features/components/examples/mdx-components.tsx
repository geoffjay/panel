import React from "react";
import { JSX } from "react/jsx-runtime";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import { nord } from "react-syntax-highlighter/dist/esm/styles/prism";

type PropType = JSX.IntrinsicAttributes &
  React.ClassAttributes<HTMLHeadingElement> &
  React.HTMLAttributes<HTMLHeadingElement>;

const MDXComponents = {
  h1: (props: PropType) => (
    <h1 className="text-2xl text-zinc-700 font-bold my-2" {...props} />
  ),
  h2: (props: PropType) => (
    <h2 className="text-xl text-zinc-700 font-bold my-2" {...props} />
  ),
  h3: (props: PropType) => (
    <h3 className="text-lg text-zinc-700 font-bold my-2" {...props} />
  ),
  h4: (props: PropType) => (
    <h3 className="text-lg text-zinc-600 font-bold my-2" {...props} />
  ),
  p: (props: PropType) => <p className="text-zinc-700" {...props} />,
  code({
    className,
    children,
    ...props
  }: {
    className?: string;
    children?: React.ReactNode;
  }) {
    const match = /language-(\w+)/.exec(className || "");
    return match ? (
      <SyntaxHighlighter
        style={nord}
        language={match[1]}
        showLineNumbers={true}
        PreTag="div"
        {...props}
      >
        {String(children).replace(/\n$/, "")}
      </SyntaxHighlighter>
    ) : (
      <code className={className} {...props}>
        {children}
      </code>
    );
  },
};

export { MDXComponents };
