import React, { useEffect, useState } from "react";

import { invoke, Channel } from "@tauri-apps/api/core";

import { Layout, UserTable } from "~components";
import { User, columns as userColumns } from "~components/user-table";
import { Button } from "~components/ui/button";
import { useMachineContext, EVENTS, STATES } from "~components/context/machine-provider";
import { useUsers } from "~lib/hooks";

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
  const [users, setUsers] = useState<User[]>([]);
  const [current, send] = useMachineContext();
  const { users: getUsers, error: userError } = useUsers();

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

  const isLoading = current.name === STATES.LOADING;

  useEffect(() => {
    const fetchUsers = async () => {
      const fetchedUsers = await getUsers();
      if(fetchedUsers) {
        setUsers(fetchedUsers);
      }
    };

    if(current.name === "ready") {
      fetchUsers();
    }
  }, [current]);

  const handleInitialize = async () => {
    await invoke("initialize", { onEvent });
  };

  return (
    <Layout>
      <p>Current state: {current.name}</p>

      {current.name === "launched" && (
        <Button onClick={handleInitialize}>Initialize</Button>
      )}

      {isLoading && <p>Loading...</p>}

      {(current.name === "ready" && users) &&
        <UserTable columns={userColumns} data={users} />
      }
    </Layout>
  );
};

export default Page;
