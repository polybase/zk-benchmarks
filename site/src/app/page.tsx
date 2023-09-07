"use client";

import Image from 'next/image'
import { Flex, Center, Heading, VStack, HStack, Box, Spacer, Stack, Link } from '@chakra-ui/react'
import logo from '@/img/logo.png'
import { ResultsTable } from '@/components/ResultsTable'
import { M_PLUS_Rounded_1c } from 'next/font/google'
import { ColorModeSwitcher } from '@/components/ColorModeSwitcher';
import Faq from '../mkd/faq.mdx'
import { PoweredBy } from '@/components/PoweredBy';


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
                <HStack>
                  <Image width={50} height={50} alt='zk-bench' src={logo} />
                  <Heading as='h1' fontWeight={700} fontFamily={rounded.style.fontFamily}>zk-bench</Heading>
                </HStack>
              </Center>
            </Box>
            <Center>
              <Box maxW='container.md'>
                <ResultsTable />
              </Box>
            </Center>
            <Box maxW='container.md' margin='0 auto' width='100%' p={2} pt={10}>
              <Faq />
            </Box>
          </Stack>
        </Box>
      </Box>
    </main >
  )
}
