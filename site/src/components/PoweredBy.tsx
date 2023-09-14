import { HStack, Box, Text, Stack } from '@chakra-ui/react'
import Image from 'next/image'
// import logo from '@/img/polybase-logo-outline.svg'
import madeBy from '@/img/made-by.png'

export function PoweredBy() {
  return (
    <a href='https://polybaselabs.com' target='_blank'>
      <Image src={madeBy} alt='Made by Polybase Labs' height={37} width={135} />
    </a>
  )
}

// <HStack borderRadius={8} border='1px solid' borderColor='bw.200' backgroundColor='bw.100' p={2}>
//   <Box>
//     <Image src={logo} alt='logo' width={20} />
//   </Box>
//   <Box>
//     <Text fontSize='xs' color='bw.600'>made by</Text>
//     <Box fontWeight={600} fontSize='sm'>polybaselabs</Box>
//   </Box>
// </HStack>