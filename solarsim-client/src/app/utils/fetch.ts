type FetchErrorType =
  | "NETWORK_ERROR"
  | "TIMEOUT"
  | "BAD_STATUS"
  | "INVALID_CONTENT"
  | "UNKNOWN";

interface FetchError {
  type: FetchErrorType;
  message: string;
  status?: number;
  details?: unknown;
}

interface FetchSuccess<T> {
  ok: true;
  status: number;
  data: T;
}

interface FetchFailure {
  ok: false;
  error: FetchError;
}

export type FetchResult<T> = FetchSuccess<T> | FetchFailure;

export function displayFetchErrorType(error_type: FetchErrorType): string {
  switch (error_type) {
    case "NETWORK_ERROR":
    case "BAD_STATUS":
      return "Server is unavailable :(";
    case "TIMEOUT":
      return "Server timed out :(";
    case "INVALID_CONTENT":
      return "Unable to retrieve content :(";
    case "UNKNOWN":
    default:
      return "Unknown Server error :(";
  }
}
