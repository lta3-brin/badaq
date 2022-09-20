import { useState } from 'react'
import {
  Box,
  Text,
  Stack,
  Modal,
  Button,
  Center,
  MediaQuery,
  BackgroundImage,
  useMantineTheme,
  useMantineColorScheme
} from "@mantine/core"

export default function Hero(props) {
  const { colorScheme } = useMantineColorScheme()
  const dark = colorScheme === "dark"

  const theme = useMantineTheme()
  const colorLight = theme.colors.gray[5]
  const colorDark = theme.colors.yellow[7]
  const [opened, setOpened] = useState(false)

  return <>
    <MediaQuery smallerThan={"md"} styles={{ display: "none" }}>
      <Box>
        <BackgroundImage
          radius={0}
          src="/balance.jpg"
        >
          <Stack justify="space-between" sx={{ minHeight: "100vh" }}>
            <Center p="md">
              <div>
                <Text
                  size={"md"}
                  weight={700}
                  component="span"
                  color={dark ? colorDark : colorLight}
                >
                  Six Component Balance
                </Text><br />

                <Text
                  size={"xs"}
                  weight={500}
                  component="span"
                  color={dark ? theme.fn.lighten(colorDark, 0.15) :
                    theme.fn.lighten(colorLight, 0.15)}
                >
                  The Department of Aerospace Engineering UPM
                </Text>
              </div>
            </Center>

            <Center p={"md"}>
              <Button
                fullWidth
                uppercase
                variant="gradient"
                gradient={{ from: dark ? "lime" : "indigo", to: dark ? "yellow" : "cyan" }}
                onClick={() => setOpened(true)}
              >
                About
              </Button>

              <Modal
                opened={opened}
                onClose={() => setOpened(false)}
                title="📌 About BADAQ"
              >
                <Text weight={700} component="span">Balance Data Acquisition and Processing System (BADAQ)</Text> is a six-component balance
                data acquisition and processing system specially designed for wind tunnel
                experiment needs. This system accommodates data acquisition and processing
                for axial force, side force, normal force, lift moment, yaw moment, and
                rolling moment.
              </Modal>
            </Center>
          </Stack>
        </BackgroundImage>
      </Box>
    </MediaQuery>

    <MediaQuery largerThan={"md"} styles={{ display: "none" }}>
      <Box>
        <BackgroundImage
          radius={0}
          src="/balance.jpg"
        >
          <Center p="md">
            <div>
              <Text
                size={"md"}
                weight={700}
                component="span"
                color={dark ? colorDark : colorLight}
              >
                Six Component Balance
              </Text><br />

              <Text
                size={"xs"}
                weight={500}
                component="span"
                color={dark ? theme.fn.lighten(colorDark, 0.15) :
                  theme.fn.lighten(colorLight, 0.15)}
              >
                The Department of Aerospace Engineering UPM
              </Text>
            </div>
          </Center>
        </BackgroundImage>
      </Box>
    </MediaQuery>
  </>
}
