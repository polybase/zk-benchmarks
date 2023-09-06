"use client";

import Image from 'next/image'
import { Center, Heading, VStack, HStack, Box } from '@chakra-ui/react'
import logo from '@/img/logo.png'
import { ResultsTable } from '@/components/ResultsTable'
import { M_PLUS_Rounded_1c } from 'next/font/google'

const rounded = M_PLUS_Rounded_1c({ subsets: ['latin'], weight: ["700"], fallback: ['SF Rounded'] })

export default function Home() {
  return (
    <main>
      <Center p={10}>
        <VStack spacing={10}>
          <Box p={2}>
            <Center>
              <HStack>
                <Image width={50} height={50} alt='zk-bench' src={logo} />
                <Heading as='h1' fontWeight={700} fontFamily={rounded.style.fontFamily}>zk-bench</Heading>
              </HStack>
            </Center>
          </Box>
          <ResultsTable />
        </VStack>
      </Center>
    </main >
  )
}
