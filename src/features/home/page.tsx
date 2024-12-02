import { useEffect } from "react";

import { invoke, Channel } from "@tauri-apps/api/core";

import { Layout } from "~components";
import { Button } from "~components/ui/button";
import { useMachineContext, EVENTS, STATES } from "~components/context/machine-provider";

type AppEvent =
  | {
      event: "loading-started";
      data: {
        id: number;
      };
    }
  | {
      event: "loading-progress";
      data: {
        id: number;
        percent: number;
        message: string;
      };
    }
  | {
      event: "loading-finished";
      data: {
        id: number;
      };
    };

const Page: React.FC = () => {
  const [current, send] = useMachineContext();

  const onEvent = new Channel<AppEvent>();

  onEvent.onmessage = (message) => {
    console.log(`got app event ${message.event}`);
  };

  const isLoading = current.name === STATES.LOADING;

  const handleStartLoading = async () => {
    await invoke("initialize", { onEvent });
    send({ type: EVENTS.START_LOADING });
  };

  return (
    <Layout>
      <p>Current state: {current.name}</p>
      
      {current.name === "launched" && (
        <Button onClick={handleStartLoading}>Start Loading</Button>
      )}
      
      {isLoading && <p>Loading...</p>}
      
      {current.name === "ready" && <p>Ready!</p>}
    </Layout>
  );
}

export default Page;
