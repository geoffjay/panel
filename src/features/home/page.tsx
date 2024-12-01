import { Layout } from "~components";
import { Button } from "~components/ui/button";
import { useMachineContext, EVENTS, STATES } from "~components/context/machine-provider";

const Page: React.FC = () => {
  const [current, send] = useMachineContext();

  const isLoading = current.name === STATES.LOADING;

  const handleStartLoading = () => {
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
