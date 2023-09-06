"use client";

import Image from 'next/image'
import { Center, Heading, VStack, HStack, Box } from '@chakra-ui/react'
import logo from '@/img/logo.png'
import { ResultsTable } from '@/components/ResultsTable'
import meta from '@/fixtures/meta.json'

export default function Home() {
  return (
    <main>
      <Center p={10}>
        <VStack spacing={10}>
          <Box p={2}>
            <Center>
              <HStack>
                <Image width={50} height={50} alt='zk-bench' src={logo} />
                <Heading as='h1' className='display'>zk-bench</Heading>
              </HStack>
            </Center>
          </Box>
          <ResultsTable />
        </VStack>
      </Center>
    </main >
  )
}
