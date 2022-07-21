import Head from "next/head"
import {Box, Flex, Spacer, Container, SimpleGrid} from "@chakra-ui/react"
import StatsCard from "../components/statscard"
// import styles from "../styles/Home.module.css"

export default function Home() {
  return <div>
    <Head>
      <title>AeroDAQ UPM</title>
      <meta name="description" content="Aplikasi akuisisi data sensor dengan load cell" />
      <link rel="icon" href="/favicon.ico" />
    </Head>

    <Flex flexDirection="column" color='white' height="100vh">
      <Spacer />
        <Container maxW="9xl" py={"10"}>
          <SimpleGrid columns={{sm: 2, md: 3}} spacing="20px">
            <StatsCard title={'We serve'} stat={'50,000 people'} />
            <StatsCard title={'We serve'} stat={'50,000 people'} />
            <StatsCard title={'We serve'} stat={'50,000 people'} />
            <StatsCard title={'We serve'} stat={'50,000 people'} />
            <StatsCard title={'We serve'} stat={'50,000 people'} />
            <StatsCard title={'We serve'} stat={'50,000 people'} />
          </SimpleGrid>
        </Container>
      <Spacer />
    </Flex>
  </div>
}
