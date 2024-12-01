import React from "react";
import { createRoot } from "react-dom/client";
import { createBrowserRouter } from "react-router";
import { RouterProvider } from "react-router/dom";

import { Page as Home } from "~features/home";
import { Page as Error } from "~features/error";
import { Page as Documentation } from "~features/documentation";

import { MachineProvider } from "~components/context/machine-provider";

import "./style.css";

const routes = [
  {
    path: "/",
    element: <Home />,
    errorElement: <Error />,
    children: [
      {
        path: "/documentation",
        element: <Documentation />,
      },
    ],
  },
];

const router = createBrowserRouter(routes, {
  future: {
    v7_relativeSplatPath: true,
  },
});

const container = document.getElementById("root");
const root = createRoot(container!);

root.render(
  <React.StrictMode>
    <MachineProvider>
      <RouterProvider router={router} />
    </MachineProvider>
  </React.StrictMode>,
);
