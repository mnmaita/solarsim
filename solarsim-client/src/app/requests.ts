import type { FetchResult } from "./utils/fetch";
import type { BRPRequestBody, BRPResponseFor } from "./brp";

const host = process.env.NEXT_PUBLIC_SOLARSIM_SERVER_HOST;
const port = process.env.NEXT_PUBLIC_SOLARSIM_SERVER_PORT;
const url = URL.parse(`${host}:${port}`) ?? `${host}:${port}`;
const requestInit: RequestInit = {
  headers: { "Content-Type": "application/json" },
  method: "post",
};
const timeoutMs = 10_000;

/**
 * Perform a Bevy Remote Protocol request against the Server.
 *
 * @param {R} request - The body of the request to be sent.
 * Check `brp.ts` to review the valid fields for each request.
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
