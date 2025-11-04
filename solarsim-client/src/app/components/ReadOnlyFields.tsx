import { Box, Flex, Skeleton, Text } from "@radix-ui/themes";
import type { ReactNode } from "react";
import { capitalize } from "../utils/strings";

interface Props {
  simulationFields: Record<string, SimulationField> | null;
}

export function ReadOnlyFields({ simulationFields }: Props): ReactNode {
  if (simulationFields === null) {
    return (
      <Skeleton>
        <Box></Box>
      </Skeleton>
    );
  }

  const entries = Object.entries(simulationFields);

  return entries
    .filter(([_, fieldValue]) => fieldValue.kind === "ReadOnly")
    .map(([key, fieldValue]) => (
      <Box key={key} width="100%">
        <Skeleton loading={entries.length === 0}>
          <Flex direction="column">
            <Text id={key} as="label">
              {capitalize(key)}
            </Text>
            <Text aria-labelledby={key}>{fieldValue.value}</Text>
          </Flex>
        </Skeleton>
      </Box>
    ));
}
