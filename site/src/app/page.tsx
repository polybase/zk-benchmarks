"use client";

import Image from 'next/image'
import { Flex, Center, Text, Heading, Button, useColorModeValue, HStack, Box, Spacer, Stack, Link, Divider } from '@chakra-ui/react'
import logo from '@/img/logo.png'
import { ResultsTable } from '@/components/ResultsTable'
import { M_PLUS_Rounded_1c } from 'next/font/google'
import { ColorModeSwitcher } from '@/components/ColorModeSwitcher'
import Faq from '../mkd/faq.mdx'
import { PoweredBy } from '@/components/PoweredBy'
import x from '@/img/X.png'


const rounded = M_PLUS_Rounded_1c({ subsets: ['latin'], weight: ["700"], fallback: ['SF Rounded'] })

export default function Home() {
  return (
    <main>
      <Box mb={20}>
        <Flex>
          <Spacer />
          <HStack p={3}>
            <ColorModeSwitcher />
            <Box pr={2}>
              <Link as='a' href='https://github.com/polybase/zk-benchmarks' target='_blank' fontWeight='600'>
                github
              </Link>
            </Box>
            <Box>
              <PoweredBy />
            </Box>
          </HStack>
        </Flex>
        <Box p={4}>
          <Stack spacing={10}>
            <Box p={2}>
              <Center>
                <Stack spacing={4}>
                  <Center>
                    <HStack>
                      <Image width={50} height={50} alt='zk-bench' src={logo} />
                      <Heading as='h1' fontWeight={700} fontFamily={rounded.style.fontFamily}>zk-bench</Heading>
                    </HStack>
                  </Center>
                  <Stack>
                    <Heading as='h2' fontSize='lg' fontFamily={rounded.style.fontFamily} textAlign='center'>Impartial benchmarks for your favourite ZK frameworks</Heading>
                    <Heading as='h3' fontSize='md' textAlign='center' fontWeight='normal'>(if it’s not fair, <Link as='a' href='https://github.com/polybase/zk-benchmarks' target='_blank' color={useColorModeValue('blue.600', 'blue.200')}>raise a PR!</Link>)</Heading>
                  </Stack>
                </Stack>
              </Center>
            </Box>
            <Box maxW='container.lg' margin='0 auto'>
              <ResultsTable />
            </Box>
            <Box maxW='container.md' width='100%' margin='0 auto' p={2}>
              <Stack bg='bw.50' borderRadius={10} textAlign='center' padding={10} spacing={4} mt={10}>
                <Stack>
                  <Heading fontFamily={rounded.style.fontFamily}>Follow us on <Text as='span' textDecoration='line-through'>Twitter</Text> X</Heading>
                  <Heading fontSize='xl' color='bw.700'>To get updates on zk-bench follow the Polybase Labs team.</Heading>
                </Stack>
                <Center>
                  <Button as='a' href='https://twitter.com/intent/user?screen_name=polybase_xyz' target='_blank' size='lg' background='blackAlpha.900' color='white' _hover={{ background: 'blackAlpha.800' }} _active={{ background: 'blackAlpha.700' }} colorScheme='white'>
                    <HStack>
                      <Image width={22} src={x} alt='X Logo' />
                      <Text>@polybase_xyz</Text>
                    </HStack>
                  </Button>
                </Center>
              </Stack>
            </Box>
            <Box maxW='container.md' margin='0 auto' width='100%' p={2}>
              <Faq />
            </Box>
          </Stack>
        </Box>
      </Box>
    </main >
  )
}
