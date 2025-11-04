import type {
  BRPGetResourcesRequestBody,
  BRPGetResourcesResponse,
} from "../brp";
import { displayFetchErrorType } from "../utils/fetch";

export const networkError = Error(displayFetchErrorType("NETWORK_ERROR"));

export const mockSimulationSliderField: SimulationField = {
  kind: "Slider",
  max: 0,
  min: 100,
  value: 50,
};

export const mockSimulationReadOnlyField: SimulationField = {
  kind: "ReadOnly",
  max: 0,
  min: 100,
  value: 25,
};

export const mockBrpRequestBody: BRPGetResourcesRequestBody = {
  id: 0,
  jsonrpc: "2.0",
  method: "world.get_resources",
  params: { resource: "solarsim_server::TestResource" },
};

export const mockBrpResponse: BRPGetResourcesResponse = {
  id: 0,
  jsonrpc: "2.0",
  result: { value: { test_field: 1 } },
};
