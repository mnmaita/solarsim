import "./palette/radix-accents-dark.css";
import "./palette/radix-background-dark.css";
import "./palette/radix-gray-dark.css";
import "./palette/radix-accents-light.css";
import "./palette/radix-background-light.css";
import "./palette/radix-gray-light.css";
import "@radix-ui/themes/styles.css";

import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import { ThemeProvider } from "next-themes";
import { Flex, Text, Theme } from "@radix-ui/themes";
import "./globals.css";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "Solarsim Client",
  description: "Powered by Next.js",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body className={`${geistSans.variable} ${geistMono.variable}`}>
        <ThemeProvider attribute="class" defaultTheme="system" enableSystem>
          <Theme appearance="inherit">{children}</Theme>
        </ThemeProvider>
      </body>
    </html>
  );
}
