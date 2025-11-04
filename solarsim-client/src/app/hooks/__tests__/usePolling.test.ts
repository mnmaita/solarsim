import { renderHook, act } from "@testing-library/react";
import { usePolling } from "../usePolling";

jest.useFakeTimers();

describe("usePolling", () => {
  let mockFn: jest.Mock<Promise<number>, []>;

  beforeEach(() => {
    jest.clearAllTimers();
    mockFn = jest.fn().mockResolvedValue(42);
  });

  it("calls the polling function immediately on mount when immediate is true", async () => {
    renderHook(() => usePolling(mockFn, { intervalMs: 1000, immediate: true }));

    await act(async () => {
      await Promise.resolve();
    });

    expect(mockFn).toHaveBeenCalledTimes(1);
  });

  it("does not call the polling function immediately when immediate is false", async () => {
    renderHook(() =>
      usePolling(mockFn, { intervalMs: 1000, immediate: false })
    );

    await act(async () => {
      await Promise.resolve();
    });

    expect(mockFn).not.toHaveBeenCalled();
  });

  it("polls repeatedly at the given interval", async () => {
    renderHook(() => usePolling(mockFn, { intervalMs: 500 }));

    await act(async () => {
      await Promise.resolve();
    });

    expect(mockFn).toHaveBeenCalledTimes(1);

    await act(async () => {
      jest.advanceTimersByTime(1500);
      await Promise.resolve();
    });

    expect(mockFn).toHaveBeenCalledTimes(4);
  });

  it("updates data and clears error on success", async () => {
    const { result } = renderHook(() =>
      usePolling(mockFn, { intervalMs: 500 })
    );

    await act(async () => {
      await Promise.resolve();
    });

    expect(result.current.data).toBe(42);
    expect(result.current.error).toBeNull();
    expect(result.current.loading).toBe(false);
  });

  it("handles errors and calls onError callback", async () => {
    const error = new Error("Network failure");
    const onError = jest.fn();
    mockFn.mockRejectedValueOnce(error);

    const { result } = renderHook(() =>
      usePolling(mockFn, { intervalMs: 500, onError })
    );

    await act(async () => {
      await Promise.resolve();
    });

    expect(result.current.error).toBe(error);
    expect(onError).toHaveBeenCalledWith(error);
    expect(result.current.loading).toBe(false);
  });

  it("pause() stops polling and resume() restarts it", async () => {
    const { result } = renderHook(() =>
      usePolling(mockFn, { intervalMs: 500 })
    );

    await act(async () => {
      await Promise.resolve();
    });

    expect(mockFn).toHaveBeenCalledTimes(1);

    act(() => {
      result.current.pause();
    });

    await act(async () => {
      jest.advanceTimersByTime(2000);
      await Promise.resolve();
    });

    expect(mockFn).toHaveBeenCalledTimes(1);

    act(() => {
      result.current.resume();
    });

    await act(async () => {
      jest.advanceTimersByTime(1000);
      await Promise.resolve();
    });

    expect(mockFn).toHaveBeenCalledTimes(3);
  });

  it("clears interval and stops polling on unmount", async () => {
    const { unmount } = renderHook(() =>
      usePolling(mockFn, { intervalMs: 500 })
    );

    await act(async () => {
      await Promise.resolve();
    });

    expect(mockFn).toHaveBeenCalledTimes(1);

    unmount();

    await act(async () => {
      jest.advanceTimersByTime(2000);
      await Promise.resolve();
    });

    expect(mockFn).toHaveBeenCalledTimes(1);
  });

  it("toggles loading state correctly", async () => {
    const { result } = renderHook(() =>
      usePolling(mockFn, { intervalMs: 500 })
    );

    expect(result.current.loading).toBe(true);

    await act(async () => {
      await Promise.resolve();
    });

    expect(result.current.loading).toBe(false);
  });
});
