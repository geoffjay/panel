import { createContext, useContext, ReactNode } from "react";
import { createMachine, state, transition } from "robot3";
import { useMachine } from "react-robot";

export const EVENTS = {
  LOADING_STARTED: "LOADING_STARTED",
  LOADING_PROGRESS: "LOADING_PROGRESS",
  LOADING_FINISHED: "LOADING_FINISHED",
} as const;

export const STATES = {
  LAUNCHED: "launched",
  LOADING: "loading",
  READY: "ready",
} as const;

export type MachineState = (typeof STATES)[keyof typeof STATES];

export type MachineEvent = 
  | { type: typeof EVENTS.LOADING_STARTED }
  | { type: typeof EVENTS.LOADING_PROGRESS }
  | { type: typeof EVENTS.LOADING_FINISHED };

const appMachine = createMachine({
  [STATES.LAUNCHED]: state(transition(EVENTS.LOADING_STARTED, STATES.LOADING)),
  [STATES.LOADING]: state(transition(EVENTS.LOADING_FINISHED, STATES.READY)),
  [STATES.READY]: state(),
});

type MachineTuple = ReturnType<typeof useMachine<typeof appMachine>>;
const MachineContext = createContext<MachineTuple | undefined>(undefined);

interface MachineProviderProps {
  children: ReactNode;
}

export function MachineProvider({ children }: MachineProviderProps) {
  const machine = useMachine(appMachine);
  
  return (
    <MachineContext.Provider value={machine}>
      {children}
    </MachineContext.Provider>
  );
}

export function useMachineContext() {
  const context = useContext(MachineContext);
  if (context === undefined) {
    throw new Error("useMachineContext must be used within a MachineProvider");
  }
  return context;
}

