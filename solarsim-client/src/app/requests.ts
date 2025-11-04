import type { FetchResult } from "./utils/fetch";
import type { BRPRequestBody, BRPResponseFor } from "./utils/brp";

const host = process.env.NEXT_PUBLIC_SOLARSIM_SERVER_HOST;
const port = process.env.NEXT_PUBLIC_SOLARSIM_SERVER_PORT;
const url = URL.parse(`${host}:${port}`) ?? `${host}:${port}`;
const requestInit: RequestInit = {
  headers: { "Content-Type": "application/json" },
  method: "post",
};
const timeoutMs = 10_000;

/**
 * Get the value of a Resource
 *
 * @param {string} name - Type name of the Resource (without its full path).
 * @param {string} [path] - Fully-qualified type path of the resource to get (without its name).
 * Defaults to "solarsim_server".
 * @param {JSONValue} [id] - Arbitrary JSON data. The server completely ignores its contents,
 * and the client may use it for any purpose. It will be copied via
 * serialization and deserialization (so object property order, etc. can’t be
 * relied upon to be identical) and sent back to the client as part of the
 * response. Defaults to 0.
 * */
export async function brpRequest<R extends BRPRequestBody>(
  request: R
): Promise<FetchResult<BRPResponseFor<R>>> {
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);

  try {
    const response = await fetch(url, {
      ...requestInit,
      body: JSON.stringify(request),
      signal: controller.signal,
    });

    clearTimeout(timeout);

    if (!response.ok) {
      const text = await response.text().catch(() => "");

      return {
        ok: false,
        error: {
          type: "BAD_STATUS",
          message: `HTTP error ${response.status}: ${response.statusText}`,
          status: response.status,
          details: text,
        },
      };
    }

    const data = await response.json();

    return { ok: true, status: response.status, data };
  } catch (error) {
    clearTimeout(timeout);

    if (error instanceof DOMException && error.name === "AbortError") {
      return {
        ok: false,
        error: {
          type: "TIMEOUT",
          message: `Request timed out after ${timeoutMs} ms`,
        },
      };
    }

    return {
      ok: false,
      error: {
        type: "NETWORK_ERROR",
        message: (error as Error)?.message ?? "Network request failed",
        details: error,
      },
    };
  }
}
