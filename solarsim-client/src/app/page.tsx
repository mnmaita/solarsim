"use client";

import { brpRequest } from "./requests";
import { displayFetchErrorType } from "./utils/fetch";
import { ReadOnlyFields } from "./components/ReadOnlyFields";
import { SliderFields } from "./components/SliderFields";
import { Container, Flex, Section, Text } from "@radix-ui/themes";
import { useEffect, useRef, useState } from "react";
import { usePolling } from "./utils/usePolling";

export default function Home() {
  const { data } = usePolling(() =>
    brpRequest({
      id: 0,
      jsonrpc: "2.0",
      method: "world.get_resources",
      params: { resource: "solarsim_server::SimulationConfig" },
    })
      .then((body) => {
        if (body.ok === false) {
          return Error(displayFetchErrorType(body.error.type));
        } else {
          return body.data.result.value as Record<string, SimulationField>;
        }
      })
      .catch((err) => {
        console.error(err);
        return Error("Unknown error :(");
      })
  );

  const [simulationFields, setSimulationFields] = useState<
    Record<string, SimulationField> | Error | null
  >({});
  const userInteracting = useRef(false);

  // When polling updates come in, apply only if user isn't dragging
  useEffect(() => {
    if (!userInteracting.current) {
      // eslint-disable-next-line react-hooks/set-state-in-effect
      setSimulationFields(data);
    }
  }, [data]);

  return (
    <Section>
      <Container align="center" size={{ initial: "1", md: "3", lg: "4" }}>
        <Flex
          direction={{ initial: "column", md: "row" }}
          gap={{ initial: "8", md: "6" }}
        >
          {Error.isError(simulationFields) ? (
            <Text size="8" align="center" aria-label={simulationFields.message}>
              {simulationFields.message}
            </Text>
          ) : (
            <>
              <Flex
                direction="column"
                gap="4"
                width={{ initial: "100%", md: "50%" }}
                align={{ initial: "center", md: "start" }}
              >
                <Text size={"8"} aria-label="Simulation Parameters">
                  Simulation Parameters
                </Text>
                <SliderFields
                  setSimulationFields={setSimulationFields}
                  simulationFields={simulationFields}
                  userInteracting={userInteracting}
                />
              </Flex>
              <Flex
                direction="column"
                gap="4"
                width={{ initial: "100%", md: "50%" }}
                align={{ initial: "center", md: "start" }}
              >
                <Text size={"8"} aria-label="Simulation Results">
                  Simulation Results
                </Text>
                <ReadOnlyFields simulationFields={simulationFields} />
              </Flex>
            </>
          )}
        </Flex>
      </Container>
    </Section>
  );
}
