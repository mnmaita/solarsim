import { mockBrpRequestBody, mockBrpResponse } from "../__mocks__/constants";
import { brpRequest } from "../requests";
import { FetchResult } from "../utils/fetch";

global.fetch = jest.fn();

jest.mock("../requests", () => {
  const actual = jest.requireActual("../requests");
  return {
    ...actual,
    url: URL.parse("https://test.com/api"),
    timeoutMs: 1000,
    requestInit: {
      method: "POST",
      headers: { "Content-Type": "application/json" },
    },
  };
});

describe("brpRequest", () => {
  const mockFetch = global.fetch as jest.Mock;

  beforeEach(() => {
    jest.useFakeTimers();
  });

  afterEach(() => {
    jest.useRealTimers();
  });

  it("returns ok=true on a successful JSON response", async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      status: 200,
      json: async () => mockBrpResponse,
    });

    const result = (await brpRequest(mockBrpRequestBody)) as FetchResult<
      typeof mockBrpResponse
    >;

    expect(result.ok).toBe(true);
    if (result.ok) {
      expect(result.status).toBe(200);
      expect(result.data).toEqual(mockBrpResponse);
    }
  });

  it("handles non-OK (bad status) responses correctly", async () => {
    mockFetch.mockResolvedValueOnce({
      ok: false,
      status: 404,
      statusText: "Not Found",
      text: async () => "Not Found Error",
    });

    const result = await brpRequest(mockBrpRequestBody);

    expect(result.ok).toBe(false);
    if (!result.ok) {
      expect(result.error.type).toBe("BAD_STATUS");
      expect(result.error.status).toBe(404);
      expect(result.error.message).toBe("HTTP error 404: Not Found");
      expect(result.error.details).toBe("Not Found Error");
    }
  });

  it("handles network errors", async () => {
    const networkErr = new Error("Network down");
    mockFetch.mockRejectedValueOnce(networkErr);

    const result = await brpRequest(mockBrpRequestBody);

    expect(result.ok).toBe(false);

    if (!result.ok) {
      expect(result.error.type).toBe("NETWORK_ERROR");
      expect(result.error.message).toContain("Network down");
    }
  });

  it("handles timeout when request is aborted", async () => {
    mockFetch.mockImplementationOnce(() =>
      Promise.reject(new DOMException("Aborted", "AbortError"))
    );

    const result = await brpRequest(mockBrpRequestBody);

    expect(result.ok).toBe(false);
    if (!result.ok) {
      expect(result.error.type).toBe("TIMEOUT");
      expect(result.error.message).toContain("timed out");
    }
  });

  it("handles unexpected thrown values gracefully", async () => {
    mockFetch.mockRejectedValueOnce("Unexpected Error");

    const result = await brpRequest(mockBrpRequestBody);

    expect(result.ok).toBe(false);
    if (!result.ok) {
      expect(result.error.type).toBe("NETWORK_ERROR");
      expect(result.error.message).toContain("Network request failed");
      expect(result.error.details).toBe("Unexpected Error");
    }
  });
});
