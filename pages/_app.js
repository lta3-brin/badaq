import { ChakraProvider, extendTheme } from "@chakra-ui/react"

import DefaultTheme from "../themes/default"


const defaultTheme = extendTheme(DefaultTheme)

function AeroDAQ({ Component, pageProps }) {
  return <ChakraProvider theme={defaultTheme}>
    <Component {...pageProps} />
  </ChakraProvider>
}

export default AeroDAQ
