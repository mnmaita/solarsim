import { useEffect, useRef, useState, useCallback } from "react";

interface UsePollingOptions {
  intervalMs?: number;
  // Set as true to call the function immediately on mount
  immediate?: boolean;
  onError?: (error: unknown) => void;
}

/**
 * Generic polling hook.
 * @param fn - async function to poll
 * @param options - polling options
 */
export function usePolling<T>(
  fn: () => Promise<T>,
  options: UsePollingOptions = {}
) {
  const { intervalMs = 400, immediate = true, onError } = options;
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<unknown>(null);
  const intervalRef = useRef<NodeJS.Timeout | null>(null);
  const isActive = useRef<boolean>(true);

  const execute = useCallback(async () => {
    if (!isActive.current) return;
    setLoading(true);
    try {
      const result = await fn();
      if (!isActive.current) return;
      setData(result);
      setError(null);
    } catch (err) {
      setError(err);
      onError?.(err);
    } finally {
      if (isActive.current) setLoading(false);
    }
  }, [fn, onError]);

  useEffect(() => {
    isActive.current = true;

    if (immediate) {
      execute();
    }

    intervalRef.current = setInterval(execute, intervalMs);

    return () => {
      isActive.current = false;
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, [execute, immediate, intervalMs]);

  const pause = () => {
    if (intervalRef.current) clearInterval(intervalRef.current);
    intervalRef.current = null;
  };

  const resume = () => {
    if (intervalRef.current == null) {
      intervalRef.current = setInterval(execute, intervalMs);
    }
  };

  return { data, loading, error, pause, resume };
}
