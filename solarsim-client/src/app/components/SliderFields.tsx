"use client";

import { Box, Flex, Skeleton, Slider, Text } from "@radix-ui/themes";
import type { Dispatch, ReactNode, RefObject, SetStateAction } from "react";
import { capitalize } from "../utils/strings";
import { brpRequest } from "../requests";

interface Props {
  simulationFields: Record<string, SimulationField> | null;
  setSimulationFields: Dispatch<
    SetStateAction<Record<string, SimulationField> | Error | null>
  >;
  userInteracting: RefObject<boolean>;
}

export function SliderFields({
  simulationFields,
  setSimulationFields,
  userInteracting,
}: Readonly<Props>): ReactNode {
  if (simulationFields === null) {
    return (
      <Skeleton>
        <Box></Box>
      </Skeleton>
    );
  }

  const entries = Object.entries(simulationFields);

  return entries
    .filter(([_, fieldValue]) => fieldValue.kind === "Slider")
    .map(([fieldName, fieldValue]) => (
      <Box key={fieldName} width="100%">
        <Flex direction="column" gapY="2">
          <Skeleton loading={entries.length === 0}>
            <Flex justify="between">
              <Text id={fieldName} as="label">
                {capitalize(fieldName)}
              </Text>
              <Text id={fieldName} as="label">
                {fieldValue.value.toFixed(2)}
              </Text>
            </Flex>
            <Slider
              aria-labelledby={fieldName}
              color="amber"
              value={[fieldValue.value]}
              min={fieldValue.min}
              max={fieldValue.max}
              step={0.1}
              onPointerDown={() => {
                userInteracting.current = true;
              }}
              onPointerUp={() => {
                userInteracting.current = false;
              }}
              onValueChange={([val]) => {
                setSimulationFields((prev) => {
                  if (prev && !Error.isError(prev)) {
                    const fields = { ...prev };
                    fields[fieldName].value = val;
                    return fields;
                  }

                  return prev;
                });
              }}
              onValueCommit={([val]) => {
                brpRequest({
                  id: 0,
                  jsonrpc: "2.0",
                  method: "world.mutate_resources",
                  params: {
                    resource:
                      "solarsim_server::simulation::simulation_config::SimulationConfig",
                    path: `.${fieldName}`,
                    value: {
                      ...fieldValue,
                      value: val,
                    } as SimulationField,
                  },
                })
                  .then((value) => {
                    // eslint-disable-next-line @typescript-eslint/no-explicit-any
                    if ((value as any)?.data?.error) {
                      console.error(
                        "Error ",
                        // eslint-disable-next-line @typescript-eslint/no-explicit-any
                        (value as any)?.data?.error.code,
                        ": ",
                        // eslint-disable-next-line @typescript-eslint/no-explicit-any
                        (value as any)?.data?.error.message
                      );
                    }
                  })
                  .catch((err) => {
                    console.error(err);
                  });
              }}
            />
          </Skeleton>
        </Flex>
      </Box>
    ));
}
