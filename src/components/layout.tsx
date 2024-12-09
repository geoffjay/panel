import React, { PropsWithChildren } from "react";

import { ThemeProvider } from "~components/context/theme-provider";

interface Props extends PropsWithChildren {
  className?: string;
};

const Layout: React.FC<Props> = ({ className, children }: Props) => {
  const classes = `${className} h-screen bg-zinc-100 dark:bg-zinc-900`;
  return (
    <ThemeProvider defaultTheme="dark" storageKey="panel-theme">
      <main className={classes}>
        {children}
      </main>
    </ThemeProvider>
  );
};

export default Layout;
