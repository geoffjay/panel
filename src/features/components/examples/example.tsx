import React from "react";

import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "~components/ui/select";
import { Input } from "~components/ui/input";
import { Label } from "~components/ui/label";
import { Slider } from "~components/ui/slider";

type ExampleType = React.FC<ExampleProps> & {
  // Static subcomponents added to the Example component
  RangeControl: React.FC<RangeControlProps>;
  SelectControl: React.FC<SelectControlProps>;
  TextInputControl: React.FC<TextInputControlProps>;
};

// Props for the Example component, containing optional content and controls
interface ExampleProps {
  content?: React.ReactNode;
  controls?: React.ReactNode;
}

// Example component that renders a content area alongside a controls panel
const Example: ExampleType = ({ content, controls }: ExampleProps) => (
  <div className="bg-zinc-200 flex flex-row border border-zinc-700 rounded-md shadow-md">
    {/* Main content section */}
    <div className="flex-auto py-2 px-4">{content}</div>
    {/* Controls pane section */}
    <div className="flex-2 border-l border-l-zinc-700 bg-zinc-300 min-w-[400px] py-4 px-4 rounded-r-md">
      <h1 className="text-xl text-zinc-700 font-bold pb-4">Controls</h1>
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

const RangeControl: React.FC<RangeControlProps> = ({
  id,
  label,
  min = 0,
  max = 100,
  value,
  onChange,
}: RangeControlProps) => (
  <div className="flex flex-row gap-4">
    <Label htmlFor={id} className="text-zinc-700 my-auto min-w-[120px]">
      {label}
    </Label>
    <Slider
      id={id}
      className="bg-zinc-200 w-full max-w-xs"
      min={min}
      max={max}
      value={[value]}
      onChange={onChange}
    />
  </div>
);

type OptionType = {
  value: string;
  label: string;
  selected?: boolean;
};

interface SelectControlProps {
  label: string;
  options: Array<OptionType>;
  value: string;
  // eslint-disable-next-line no-unused-vars
  onChange: (value: string) => void;
}

const SelectControl: React.FC<SelectControlProps> = ({
  label,
  options,
  value,
  onChange,
}) => (
  <div className="flex flex-row gap-4">
    <Select onValueChange={onChange} defaultValue={value}>
      <SelectTrigger className="bg-zinc-200 border border-zinc-400">
        <SelectValue
          placeholder={label}
          className="text-zinc-700 my-auto min-w-[120px]"
        />
      </SelectTrigger>
      <SelectContent className="select select-bordered bg-zinc-200 w-full max-w-xs">
        <SelectGroup>
          {options.map((option, i) => (
            <SelectItem
              key={`option-${option.value}-${i}`}
              value={option.value}
            >
              {option.label}
            </SelectItem>
          ))}
        </SelectGroup>
      </SelectContent>
    </Select>
  </div>
);

interface TextInputControlProps {
  id: string;
  label: string;
  value: string;
  // eslint-disable-next-line no-unused-vars
  onChange: (event: React.FormEvent<HTMLInputElement>) => void;
}

const TextInputControl: React.FC<TextInputControlProps> = ({
  id,
  label,
  value,
  onChange,
}: TextInputControlProps) => {
  return (
    <div className="flex flex-row gap-4">
      <Label htmlFor={id} className="text-zinc-700 my-auto min-w-[120px]">
        {label}
      </Label>
      <Input
        id={id}
        className="input input-bordered bg-zinc-200 border border-zinc-400 w-full max-w-xs"
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
