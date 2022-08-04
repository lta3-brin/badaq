import Head from "next/head"
import { IconSun, IconMoonStars } from "@tabler/icons"
import {
  Grid,
  Affix,
  ActionIcon,
  useMantineColorScheme,
} from "@mantine/core"
import Hero from "../components/hero"


export default function Home() {
  const { colorScheme, toggleColorScheme } = useMantineColorScheme()
  const dark = colorScheme === "dark"

  return <div>
    <Head>
      <title>AeroDAQ UPM</title>
      <meta name="description" content="Aplikasi akuisisi data sensor dengan load cell" />
      <link rel="icon" href="/favicon.ico" />
    </Head>

    <Grid gutter={0}>
      <Grid.Col sm={12} md={3} lg={2}>
        <Hero />
      </Grid.Col>

      <Grid.Col sm={12} md={9} lg={10}>

      </Grid.Col>
    </Grid>

    <Affix position={{ bottom: 20, right: 20 }}>
      <ActionIcon
        variant="outline"
        color={dark ? "yellow" : "dark"}
        onClick={() => toggleColorScheme()}
        title="Toggle color scheme"
      >
        {dark ? <IconSun size={18} /> : <IconMoonStars size={18} />}
      </ActionIcon>
    </Affix>
  </div>
}
