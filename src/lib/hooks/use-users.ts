/* eslint-disable @typescript-eslint/no-explicit-any */

import { useState } from "react";

import { debug } from "@tauri-apps/plugin-log";

import { User } from "~components/user-table";
import { useDatabase } from "~components/context/database-provider";

const useUsers = () => {
  const [error, setError] = useState<string | undefined>(undefined);
  const { execute, isLoading } = useDatabase();

  if (isLoading) {
    return {
      userById: () => Promise.resolve(undefined),
      users: () => Promise.resolve(undefined),
      addUser: () => Promise.resolve(undefined),
      updateUser: () => Promise.resolve(undefined),
      deleteUser: () => Promise.resolve(undefined),
      error: "Database connection must be loaded",
    };
  }

  const userById = async (id: number): Promise<User | undefined> => {
    try {
      const result = await execute("SELECT * FROM users WHERE id = ?", [id]);
      debug(result);
      return result ? { id: result.id, name: result.name } : undefined;
    } catch (err: any | unknown) {
      setError(err.message);
      return undefined;
    }
  };

  const users = async (): Promise<[User] | undefined> => {
    console.log("fetch users");
    try {
      const result = await execute("SELECT * FROM users");
      console.log(result);
      debug(result);
      const res = result ? result.map((user: any) => ({ id: user.id, name: user.name })) : undefined;
      console.log(res);
    } catch (err: any | unknown) {
      setError(err.message);
      return undefined;
    }
  };

  const addUser = async (user: User): Promise<User | undefined> => {
    try {
      const result = await execute("INSERT INTO users (name) VALUES (?)", [user.name]);
      debug(result);
      return { ...user, id: result?.id };
    } catch (err: any | unknown) {
      setError(err.message);
      return undefined;
    }
  };

  const updateUser = async (user: User): Promise<User | undefined> => {
    try {
      const result = await execute("UPDATE users SET name = ? WHERE id = ?", [user.name, user.id]);
      debug(result);
      return { ...user, id: result?.id };
    } catch (err: any | unknown) {
      setError(err.message);
      return undefined;
    }
  };

  const deleteUser = async (user: User): Promise<User | undefined> => {
    try {
      const result = await execute("DELETE FROM users WHERE id = ?", [user.id]);
      debug(result);
      return { ...user, id: result?.id };
    } catch (err: any | unknown) {
      setError(err.message);
      return undefined;
    }
  };

  return { userById, users, addUser, updateUser, deleteUser, error };
};

export default useUsers;
