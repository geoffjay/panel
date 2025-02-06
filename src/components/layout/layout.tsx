import React, { useEffect, useState } from "react";
import { useNavigate, Outlet } from "react-router";
import { Menu } from "lucide-react";

import { getCurrentWindow } from "@tauri-apps/api/window";

import { ThemeProvider } from "~components/context/theme-provider";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuShortcut,
  DropdownMenuTrigger,
} from "~components/ui/dropdown-menu";
import { Button } from "~components/ui/button";

interface Props {
  className?: string;
}

const Layout: React.FC<Props> = ({ className }: Props) => {
  const [isFullscreen, setIsFullscreen] = useState(false);
  const navigate = useNavigate();

  const classes = `${className || ""}h-screen bg-zinc-100 dark:bg-zinc-900`;

  useEffect(() => {
    const handleKeyDown = async (event: KeyboardEvent) => {
      if (event.key.toLowerCase() === "f" && event.shiftKey && event.metaKey) {
        await getCurrentWindow()
          .setFullscreen(!isFullscreen)
          .catch(console.error);
        setIsFullscreen(!isFullscreen);
      } else if (event.key.toLowerCase() === "c" && event.metaKey) {
        navigate("/");
      } else if (event.key.toLowerCase() === "escape") {
        await getCurrentWindow().setFullscreen(false).catch(console.error);
        setIsFullscreen(false);
      }
    };

    window.addEventListener("keydown", handleKeyDown);

    return () => {
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, []);

  const handleToggleFullscreen = async () => {
    await getCurrentWindow().setFullscreen(!isFullscreen).catch(console.error);
    setIsFullscreen(!isFullscreen);
  };

  const handleClickAbout = () => {
    navigate("/about");
  };

  const handleClickDocumentation = () => {
    navigate("/documentation");
  };

  const handleClickComponents = () => {
    navigate("/documentation/components");
  };

  const handleClose = () => {
    navigate("/");
  };

  return (
    <ThemeProvider defaultTheme="light" storageKey="panel-theme">
      <main id="layout" className={classes}>
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button
              variant="ghost"
              className="fixed border-none focus:outline-none top-2 left-2 z-50 p-2 hover:bg-gray-100 dark:hover:bg-zinc-700 transition-colors"
            >
              <Menu className="w-4 h-4 text-zinc-600 dark:text-white" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent className="ml-2">
            <DropdownMenuLabel>Panel</DropdownMenuLabel>
            <DropdownMenuGroup>
              <DropdownMenuSeparator />
              <DropdownMenuItem onClick={handleToggleFullscreen}>
                Toggle Fullscreen
                <DropdownMenuShortcut>⇧⌘F</DropdownMenuShortcut>
              </DropdownMenuItem>
            </DropdownMenuGroup>
            <DropdownMenuSeparator />
            <DropdownMenuGroup>
              <DropdownMenuLabel>Help</DropdownMenuLabel>
              <DropdownMenuItem onClick={handleClickAbout}>
                About
              </DropdownMenuItem>
              <DropdownMenuItem onClick={handleClickDocumentation}>
                Documentation
              </DropdownMenuItem>
              <DropdownMenuItem onClick={handleClickComponents}>
                Components
              </DropdownMenuItem>
            </DropdownMenuGroup>
            <DropdownMenuSeparator />
            <DropdownMenuItem onClick={handleClose}>
              Close
              <DropdownMenuShortcut>⌘C</DropdownMenuShortcut>
            </DropdownMenuItem>
            <DropdownMenuItem>
              Quit
              <DropdownMenuShortcut>⌘W</DropdownMenuShortcut>
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>

        <Outlet />
      </main>
    </ThemeProvider>
  );
};

export default Layout;
