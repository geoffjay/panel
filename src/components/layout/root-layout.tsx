import React from "react";
import { Outlet } from "react-router";

import { ThemeProvider } from "~components/context/theme-provider";

interface Props {
  className?: string;
}

const Layout: React.FC<Props> = ({ className }: Props) => {
  const classes = `${className || ""}h-screen bg-zinc-100 dark:bg-zinc-900`;
  return (
    <ThemeProvider defaultTheme="light" storageKey="panel-theme">
      <main id="root-layout" className={classes}>
        <Outlet />
      </main>
    </ThemeProvider>
  );
};

export default Layout;
