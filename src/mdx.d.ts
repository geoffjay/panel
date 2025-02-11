declare module "*.mdx" {
  // import type { ComponentProps, ComponentType } from "react";
  // const component: ComponentType<ComponentProps<"div">>;
  // export default component;

  // import React from "react";
  // declare const component: React.ComponentType;
  // export default component;

  import type { ComponentType } from "react";

  interface MDXProps {
    components?: Record<string, ComponentType<any>>;
    [key: string]: any;
  }

  const component: ComponentType<MDXProps>;
  export default component;
}
