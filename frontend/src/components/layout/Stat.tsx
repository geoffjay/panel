import React from "react";

interface StatProps {
  title?: string;
  value: string;
  description?: string;
}

const Stat: React.FC<StatProps> = ({ title, value, description }: StatProps) => {
  return (
    <div className="stats shadow bg-neutral-100 text-neutral-700">
      <div className="stat">
        {title ?? <div className="stat-title">{title}</div>}
        <div className="stat-value">{value}</div>
        {description ?? <div className="stat-desc">{description}</div>}
      </div>
    </div>
  );
};

export default Stat;
