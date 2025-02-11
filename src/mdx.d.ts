declare module "*.mdx" {
  import type { ComponentType } from "react";

  interface MDXProps {
    components?: Record<string, ComponentType<any>>;
    [key: string]: any;
  }

  const component: ComponentType<MDXProps>;
  export default component;
}
