import React, { useEffect } from "react";

import { invoke, Channel } from "@tauri-apps/api/core";

import { UserTable } from "~components";
import { columns as userColumns } from "~components/user-table";
import { Button } from "~components/ui/button";
import {
  useMachineContext,
  MachineState,
  EVENTS,
  STATES,
} from "~components/context/machine-provider";
import { useGetUsersQuery } from "~lib/services";

type AppEvent =
  | {
      event: "loadingStarted";
      data: {
        id: number;
      };
    }
  | {
      event: "loadingProgress";
      data: {
        id: number;
        percent: number;
        message: string;
      };
    }
  | {
      event: "loadingFinished";
      data: {
        id: number;
      };
    };

const Page: React.FC = () => {
  const [current, send] = useMachineContext();

  const { data: users } = useGetUsersQuery();

  useEffect(() => {
    console.table(users);
  }, [users]);

  const onEvent = new Channel<AppEvent>();

  onEvent.onmessage = (message) => {
    console.log(`got app event ${message.event}`);

    switch (message.event) {
      case "loadingStarted":
        send({ type: EVENTS.LOADING_STARTED, data: message.data });
        break;
      case "loadingProgress":
        send({ type: EVENTS.LOADING_PROGRESS, data: message.data });
        break;
      case "loadingFinished":
        send({ type: EVENTS.LOADING_FINISHED, data: message.data });
        break;
    }
  };

  const handleInitialize = async () => {
    await invoke("initialize", { onEvent });
  };

  const renderComponent = (state: MachineState) => {
    switch (state) {
      case STATES.LAUNCHED:
        return <Button onClick={handleInitialize}>Initialize</Button>;
      case STATES.LOADING:
        return <p>Loading...</p>;
      case STATES.READY:
        return (
          users && (
            <div className="mx-8">
              <UserTable columns={userColumns} data={users} />
            </div>
          )
        );
      default:
        return null;
    }
  };

  return <>{current?.name && renderComponent(current.name as MachineState)}</>;
};

export default Page;
