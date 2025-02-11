import React, { createContext, useContext, useState } from "react";

import Example from "./example";
import { MDXComponents } from "./mdx-components";

import Text from "./text.mdx";

interface ExampleContextType {
  content: string;
  updateContent?: (value: string) => void;
}

const ExampleContext = createContext<ExampleContextType | null>(null);

const useExampleControls = () => {
  const exampleContext = useContext(ExampleContext);

  if (!exampleContext) {
    throw new Error(
      "useExampleControls has to be used within <ExampleContext.Provider>",
    );
  }

  return exampleContext;
};

const Controls = () => {
  const exampleContext = useExampleControls();
  const { content, updateContent } = exampleContext!;

  if (!updateContent) {
    throw new Error("updateContent is required");
  }

  const handleContent = (event: React.FormEvent<HTMLInputElement>) => {
    updateContent(event.currentTarget.value);
  };

  return (
    <div className="flex flex-col gap-2">
      <Example.TextInputControl
        id="input-content"
        label="Title"
        value={content}
        onChange={handleContent}
      />
    </div>
  );
};

const TextExample: React.FC = () => {
  const exampleContext = useExampleControls();
  const { content } = exampleContext!;

  return (
    <Example
      content={<Text {...{ components: MDXComponents, content: content }} />}
      controls={<Controls />}
    />
  );
};

const TextExampleWrapper: React.FC = () => {
  const [values, setValues] = useState<ExampleContextType>({
    content: "Sample content",
  });

  const handleContent = (content: string) => {
    setValues((prev) => ({ ...prev, content }));
  };

  return (
    <ExampleContext.Provider
      value={{
        ...values,
        updateContent: handleContent,
      }}
    >
      <TextExample />
    </ExampleContext.Provider>
  );
};

export default TextExampleWrapper;
