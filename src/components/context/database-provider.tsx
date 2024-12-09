import React, { createContext, useContext, useEffect, useState } from "react";

import Database from "@tauri-apps/plugin-sql";

type DatabaseContextType = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  execute: (query: string, bindValues?: unknown[]) => Promise<any>;
  isLoading: boolean;
  error: Error | null;
};

const DatabaseContext = createContext<DatabaseContextType | undefined>(undefined);

export function DatabaseProvider({ children }: { children: React.ReactNode }) {
  const [db, setDb] = useState<Database | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    const initDatabase = async () => {
      try {
        const database = await Database.load("sqlite:panel.db");
        setDb(database);
      } catch (err) {
        setError(err instanceof Error ? err : new Error("Failed to load database"));
      } finally {
        setIsLoading(false);
      }
    };

    initDatabase();
  }, []);

  useEffect(() => {
    if (error) {
      console.error(error);
    }
  }, [error]);

  const execute = async (query: string, bindValues?: unknown[]) => {
    if (!db) {
      throw new Error("Database not initialized");
    }
    try {
      const result = await db.execute(query, bindValues);
      return result;
    } catch(err) {
      setError(err instanceof Error ? err : new Error("Failed to read users"));
    }
  };

  const value = {
    execute,
    isLoading,
    error,
  };

  return (
    <DatabaseContext.Provider value={value}>
      {children}
    </DatabaseContext.Provider>
  );
}

export function useDatabase() {
  const context = useContext(DatabaseContext);
  
  if (context === undefined) {
    throw new Error("useDatabase must be used within a DatabaseProvider");
  }
  
  return context;
}
