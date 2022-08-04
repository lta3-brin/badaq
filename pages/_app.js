import Head from "next/head"
import {
  AppShell,
  MantineProvider,
  useMantineTheme,
  ColorSchemeProvider,
} from "@mantine/core"
import { useLocalStorage } from "@mantine/hooks"

import DefaultTheme from "../themes/default"

export default function AeroDAQ(props) {
  const { Component, pageProps } = props
  const theme = useMantineTheme()

  const [colorScheme, setColorScheme] = useLocalStorage({
    key: "mantine-color-scheme",
    defaultValue: DefaultTheme.colorScheme,
    getInitialValueInEffect: true,
  })

  const toggleColorScheme = (value) =>
    setColorScheme(value || (colorScheme === "dark" ? "light" : "dark"))

  return (
    <>
      <Head>
        <title>AeroDAQ</title>
        <meta name="viewport" content="minimum-scale=1, initial-scale=1, width=device-width" />
      </Head>

      <ColorSchemeProvider colorScheme={colorScheme} toggleColorScheme={toggleColorScheme}>
        <MantineProvider
          withGlobalStyles
          withNormalizeCSS
          theme={DefaultTheme}
        >
          <AppShell padding={0}
            navbarOffsetBreakpoint="sm"
            asideOffsetBreakpoint="sm"
            styles={{
              main: {
                background: colorScheme === "dark" ?
                  theme.fn.gradient({
                    from: theme.colors.dark[6],
                    to: theme.colors.dark[4],
                    deg: -60
                  }) :
                  theme.fn.gradient({
                    from: theme.colors.gray[6],
                    to: theme.colors.gray[3],
                    deg: -60
                  }),
              },
            }}
          >
            <Component {...pageProps} />
          </AppShell>
        </MantineProvider>
      </ColorSchemeProvider>
    </>
  );
}
