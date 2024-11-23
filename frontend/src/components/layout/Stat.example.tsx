import React, { createContext, useContext, useState } from "react";

import Example from "./Example";
import { MDXComponents } from "./MDXComponents";

import Stat from "./Stat.mdx";

interface ExampleContextType {
  title: string;
  value: string;
  description: string;
};

const ExampleContext = createContext<ExampleContextType | null>(null);

const Controls = () => {
  const [values, setValues] = useState<ExampleContextType>({
    title: "Stat",
    value: "100",
    description: "Description",
  });

  const handleTitle = (event: React.FormEvent<HTMLInputElement>) => {
    setValues({ ...values, title: event.currentTarget.value });
  };

  const handleValue = (event: React.FormEvent<HTMLInputElement>) => {
    setValues({ ...values, value: event.currentTarget.value });
  };

  const handleDescription = (event: React.FormEvent<HTMLInputElement>) => {
    setValues({ ...values, description: event.currentTarget.value });
  };

  return (
    <ExampleContext.Provider value={values}>
      <div className="flex flex-col gap-4">
        <Example.TextInputControl key="input-title" label="Title" value={values.title} onChange={handleTitle} />
        <Example.TextInputControl key="input-value" label="Value" value={values.value} onChange={handleValue} />
        <Example.TextInputControl key="input-description" label="Description" value={values.description} onChange={handleDescription} />
      </div>
    </ExampleContext.Provider>
  );
};

const StatExample: React.FC = () => {
  const values = useContext(ExampleContext);

  return (
    <Example
      content={<Stat components={MDXComponents} title={values?.title} value={values?.value} description={values?.description} />}
      controls={<Controls />}
    />
  );
};

export default StatExample;
