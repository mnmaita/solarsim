import "./palette/radix-accents-dark.css";
import "./palette/radix-background-dark.css";
import "./palette/radix-gray-dark.css";
import "./palette/radix-accents-light.css";
import "./palette/radix-background-light.css";
import "./palette/radix-gray-light.css";
import "@radix-ui/themes/styles.css";

import type { Metadata } from "next";
import { ThemeProvider } from "next-themes";
import { Container, Flex, Section, Text, Theme } from "@radix-ui/themes";
import "./globals.css";
import { ThemeToggle } from "./components/ThemeToggle";

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
      <body>
        <ThemeProvider attribute="class" defaultTheme="system" enableSystem>
          <Theme appearance="inherit">
            <header>
              <Section size="1">
                <Container size="1">
                  <Flex
                    direction="row"
                    justify="center"
                    width="100%"
                    position="sticky"
                  >
                    <Text size="9" mx="2" aria-label="Solarsim">
                      Solarsim
                    </Text>
                    <ThemeToggle style={{ justifySelf: "end" }} />
                  </Flex>
                </Container>
              </Section>
            </header>
            {children}
          </Theme>
        </ThemeProvider>
      </body>
    </html>
  );
}
