import React from "react";

type ExampleType = React.FC<ExampleProps> & {
  RangeControl: React.FC<RangeControlProps>;
  SelectControl: React.FC<SelectControlProps>;
  TextInputControl: React.FC<TextInputControlProps>;
};

interface ExampleProps {
  content?: React.ReactNode;
  controls?: React.ReactNode;
};

const Example: ExampleType = ({ content, controls }: ExampleProps) => (
  <div className="bg-neutral-200 flex flex-row border border-neutral-700 rounded-md shadow-md">
    <div className="flex-auto py-2 px-4">
      {content}
    </div>
    <div className="flex-2 border-l border-l-neutral-700 min-w-[480px] py-4 px-4">
      <h1 className="text-2xl text-neutral-800 font-bold pb-4">Controls</h1>
      {controls}
    </div>
  </div>
);

interface RangeControlProps {
  id: string;
  label: string;
  min?: number;
  max?: number;
  value: number;
  // eslint-disable-next-line no-unused-vars
  onChange: (event: React.FormEvent<HTMLInputElement>) => void;
}

const RangeControl: React.FC<RangeControlProps> = ({ id, label, min = 0, max = 100, value, onChange }: RangeControlProps) => (
  <div className="flex flex-row gap-4">
    <label htmlFor={id} className="text-neutral-800 my-auto min-w-[120px]">{label}</label>
    <input
      id={id}
      className="range bg-neutral-200 w-full max-w-xs"
      type="range"
      min={min}
      max={max}
      value={value}
      onChange={onChange}
    />
  </div>
);

type OptionType = {
  value: string;
  label: string;
  selected?: boolean;
}

interface SelectControlProps {
  label: string;
  options: Array<OptionType>;
  value: string;
  // eslint-disable-next-line no-unused-vars
  onChange: (event: React.FormEvent<HTMLSelectElement>) => void;
}

const SelectControl: React.FC<SelectControlProps> = ({ label, options, value, onChange }) => (
  <div className="flex flex-row gap-4">
    <label className="text-neutral-800 my-auto min-w-[120px]">{label}</label>
    <select className="select select-bordered bg-neutral-200 w-full max-w-xs" onChange={onChange} value={value}>
      {options.map((option, i) => (
        <option selected={option.selected ?? false} key={`option-${option.value}-${i}`} value={option.value}>{option.label}</option>
      ))}
    </select>
  </div>
);

interface TextInputControlProps {
  id: string;
  label: string;
  value: string;
  // eslint-disable-next-line no-unused-vars
  onChange: (event: React.FormEvent<HTMLInputElement>) => void;
}

const TextInputControl: React.FC<TextInputControlProps> = ({ id, label, value, onChange }: TextInputControlProps) => {
  return (
    <div className="flex flex-row gap-4">
      <label htmlFor={id} className="text-neutral-800 my-auto min-w-[120px]">
        {label}
      </label>
      <input
        id={id}
        className="input input-bordered bg-neutral-200 w-full max-w-xs"
        type="text"
        value={value}
        onChange={onChange}
      />
    </div>
  );
};

Example.RangeControl = RangeControl;
Example.SelectControl = SelectControl;
Example.TextInputControl = TextInputControl;

export default Example;
