import type { JSONValue } from "next/dist/server/config-shared";

type BRPMethod =
  | "world.get_components"
  | "world.query"
  | "world.spawn_entity"
  | "world.despawn_entity"
  | "world.remove_components"
  | "world.insert_components"
  | "world.mutate_components"
  | "world.reparent_entities"
  | "world.list_components"
  | "world.get_components+watch"
  | "world.list_components+watch"
  | "world.get_resources"
  | "world.insert_resources"
  | "world.remove_resources"
  | "world.mutate_resources"
  | "world.list_resources"
  | "registry.schema"
  | "rpc.discover";

interface BRPCommonBody {
  jsonrpc: "2.0";
  id: JSONValue;
}

interface BRPCommonRequestBody extends BRPCommonBody {
  jsonrpc: "2.0";
  method: BRPMethod;
  id: JSONValue;
}

// Requests

export interface BRPGetComponentsRequestBody extends BRPCommonRequestBody {
  method: "world.get_components";
  params: {
    entity: string;
    components: string[];
    strict?: boolean;
  };
}

export interface BRPGetResourcesRequestBody extends BRPCommonRequestBody {
  method: "world.get_resources";
  params: {
    resource: string;
  };
}

export type BRPRequestBody =
  | BRPGetResourcesRequestBody
  | BRPGetComponentsRequestBody;

// Responses

interface BRPGetResourcesResponse extends BRPCommonBody {
  result: {
    value: unknown;
  };
}

interface BRPGetComponentsResponse extends BRPCommonBody {
  result:
    | Record<string, JSONValue>
    | {
        // This will only be present if the "strict" request parameter is set to "true"
        components?: Record<string, JSONValue>;
        // This will only be present if the "strict" request parameter is set to "true"
        errors?: Record<string, string>;
        // This will only be present if the "strict" request parameter is set to "false"
      };
}

interface BRPRequestResponseMap {
  "world.get_resources": BRPGetResourcesResponse;
  "world.get_components": BRPGetComponentsResponse;
  "world.query": never;
  "world.spawn_entity": never;
  "world.despawn_entity": never;
  "world.remove_components": never;
  "world.insert_components": never;
  "world.mutate_components": never;
  "world.reparent_entities": never;
  "world.list_components": never;
  "world.get_components+watch": never;
  "world.list_components+watch": never;
  "world.insert_resources": never;
  "world.remove_resources": never;
  "world.mutate_resources": never;
  "world.list_resources": never;
  "registry.schema": never;
  "rpc.discover": never;
}

export type BRPResponseFor<R extends BRPRequestBody> = R extends {
  method: infer T;
}
  ? T extends keyof BRPRequestResponseMap
    ? BRPRequestResponseMap[T]
    : never
  : never;
