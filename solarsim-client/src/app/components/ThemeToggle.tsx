"use client";

import { useTheme } from "next-themes";
import { CSSProperties, useEffect, useLayoutEffect, useState } from "react";
import { IconButton } from "@radix-ui/themes";
import { MoonIcon, SunIcon } from "@radix-ui/react-icons";

interface Props {
  style?: CSSProperties;
}

export function ThemeToggle({ style }: Props) {
  const { theme, setTheme, resolvedTheme } = useTheme();
  const [mounted, setMounted] = useState(false);

  // Prevent hydration mismatch by ensuring we only render after mount.
  // This is not needed in production builds but we add it to get rid
  // of some warnings during development.
  const useSafeEffect =
    typeof window === "undefined" ? useEffect : useLayoutEffect;
  useSafeEffect(() => setMounted(true), []);

  if (!mounted) {
    return (
      <IconButton style={style} variant="ghost" color="yellow" disabled>
        <SunIcon width="24" height="24" />
      </IconButton>
    );
  }

  const isDark = resolvedTheme === "dark";

  return (
    <IconButton
      style={style}
      variant="ghost"
      color="yellow"
      onClick={() => setTheme(isDark ? "light" : "dark")}
      aria-label="Toggle dark mode"
    >
      {isDark ? (
        <SunIcon width="24" height="24" />
      ) : (
        <MoonIcon width="24" height="24" />
      )}
    </IconButton>
  );
}
