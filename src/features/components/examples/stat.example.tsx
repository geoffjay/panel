import React, { createContext, useContext, useState } from "react";

import Example from "./example";
import { MDXComponents } from "./mdx-components";

import Stat from "./stat.mdx";

interface ExampleContextType {
  title: string;
  value: string;
  description: string;
  updateTitle?: (title: string) => void;
  updateValue?: (value: string) => void;
  updateDescription?: (description: string) => void;
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
  const {
    title,
    value,
    description,
    updateTitle,
    updateValue,
    updateDescription,
  } = exampleContext!;

  if (!updateTitle || !updateValue || !updateDescription) {
    throw new Error(
      "updateTitle, updateValue and updateDescription are required",
    );
  }

  const handleTitle = (event: React.FormEvent<HTMLInputElement>) => {
    updateTitle(event.currentTarget.value);
  };

  const handleValue = (event: React.FormEvent<HTMLInputElement>) => {
    updateValue(event.currentTarget.value);
  };

  const handleDescription = (event: React.FormEvent<HTMLInputElement>) => {
    updateDescription(event.currentTarget.value);
  };

  return (
    <div className="flex flex-col gap-2">
      <Example.TextInputControl
        id="input-title"
        label="Title"
        value={title}
        onChange={handleTitle}
      />
      <Example.TextInputControl
        id="input-value"
        label="Value"
        value={value}
        onChange={handleValue}
      />
      <Example.TextInputControl
        id="input-description"
        label="Description"
        value={description}
        onChange={handleDescription}
      />
    </div>
  );
};

const StatExample: React.FC = () => {
  const exampleContext = useExampleControls();
  const { title, value, description } = exampleContext!;

  return (
    <Example
      content={
        <Stat
          {...{
            components: MDXComponents,
            title: title,
            value: value,
            description: description,
          }}
        />
      }
      controls={<Controls />}
    />
  );
};

const StatExampleWrapper: React.FC = () => {
  const [values, setValues] = useState<ExampleContextType>({
    title: "Stat",
    value: "100",
    description: "Description",
  });

  const handleTitle = (title: string) => {
    setValues((prev) => ({ ...prev, title }));
  };

  const handleValue = (value: string) => {
    setValues((prev) => ({ ...prev, value }));
  };

  const handleDescription = (description: string) => {
    setValues((prev) => ({ ...prev, description }));
  };

  return (
    <ExampleContext.Provider
      value={{
        ...values,
        updateTitle: handleTitle,
        updateValue: handleValue,
        updateDescription: handleDescription,
      }}
    >
      <StatExample />
    </ExampleContext.Provider>
  );
};

export default StatExampleWrapper;
