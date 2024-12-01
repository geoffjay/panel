import { createContext, useContext, ReactNode } from "react";
import { createMachine, state, transition } from "robot3";
import { useMachine } from "react-robot";

export const EVENTS = {
  START_LOADING: "START_LOADING",
  COMPLETE_LOADING: "COMPLETE_LOADING"
} as const;

export const STATES = {
  LAUNCHED: "launched",
  LOADING: "loading",
  READY: "ready"
} as const;

type MachineState = (typeof STATES)[keyof typeof STATES];

type MachineEvent = 
  | { type: typeof EVENTS.START_LOADING }
  | { type: typeof EVENTS.COMPLETE_LOADING };

const appMachine = createMachine({
  [STATES.LAUNCHED]: state(transition(EVENTS.START_LOADING, STATES.LOADING)),
  [STATES.LOADING]: state(transition(EVENTS.COMPLETE_LOADING, STATES.READY)),
  [STATES.READY]: state()
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

