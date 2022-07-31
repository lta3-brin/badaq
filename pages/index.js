import Head from "next/head"
import { IconSun, IconMoonStars } from "@tabler/icons"
import {
  Text,
  Title,
  Affix,
  ActionIcon,
  useMantineColorScheme,
} from "@mantine/core"


export default function Home() {
  const { colorScheme, toggleColorScheme } = useMantineColorScheme()
  const dark = colorScheme === "dark"

  return <div>
    <Head>
      <title>AeroDAQ UPM</title>
      <meta name="description" content="Aplikasi akuisisi data sensor dengan load cell" />
      <link rel="icon" href="/favicon.ico" />
    </Head>

    <Title order={3}>
      <Text color={dark ? "yellow" : "blue"} inherit component="span">
        Highlight something in title {colorScheme}.
      </Text>
    </Title>

    <Affix position={{ bottom: 20, right: 20 }}>
      <ActionIcon
        variant="outline"
        color={dark ? "yellow" : "blue"}
        onClick={() => toggleColorScheme()}
        title="Toggle color scheme"
      >
        {dark ? <IconSun size={18} /> : <IconMoonStars size={18} />}
      </ActionIcon>
    </Affix>
  </div>
}
