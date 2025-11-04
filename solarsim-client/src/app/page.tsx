import { brpRequest } from "./requests";
import { displayFetchErrorType } from "./utils/fetch";
import { capitalize } from "./utils/strings";
import {
  Box,
  Container,
  Flex,
  Section,
  Skeleton,
  Slider,
  Text,
} from "@radix-ui/themes";
import { ReactNode } from "react";

export default async function Home() {
  const simulationFields = await brpRequest({
    id: 0,
    jsonrpc: "2.0",
    method: "world.get_resources",
    params: { resource: "solarsim_server::SimulationConfig" },
  })
    .then((body) => {
      if (body.ok === false) {
        return Error(displayFetchErrorType(body.error.type));
      } else {
        console.log(body.data.result.value);
        return Object.entries(
          body.data.result.value as Record<string, SimulationField>
        );
      }
    })
    .catch((err) => {
      console.log(err);
      return Error("Unknown error :(");
    });

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
                {sliderFields(simulationFields)}
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
                {readOnlyFields(simulationFields)}
              </Flex>
            </>
          )}
        </Flex>
      </Container>
    </Section>
  );
}

function sliderFields(fields: Array<[string, SimulationField]>): ReactNode {
  return fields
    .filter(([_, value]) => value.kind === "Slider")
    .map(([key, value]) => (
      <Box key={key} width="100%">
        <Flex direction="column" gapY="2">
          <Skeleton loading={fields.length === 0}>
            <Text id={key} as="label">
              {capitalize(key)}
            </Text>
            <Slider
              aria-labelledby={key}
              defaultValue={[value.value]}
              min={value.min}
              max={value.max}
            />
          </Skeleton>
        </Flex>
      </Box>
    ));
}

function readOnlyFields(fields: Array<[string, SimulationField]>): ReactNode {
  return fields
    .filter(([_, value]) => value.kind === "ReadOnly")
    .map(([key, value]) => (
      <Box key={key} width="100%">
        <Skeleton loading={fields.length === 0}>
          <Flex direction="column">
            <Text id={key} as="label">
              {capitalize(key)}
            </Text>
            <Text aria-labelledby={key}>{value.value}</Text>
          </Flex>
        </Skeleton>
      </Box>
    ));
}
