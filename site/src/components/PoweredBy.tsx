import { useColorMode } from '@chakra-ui/react'
import Image from 'next/image'
// import logo from '@/img/polybase-logo-outline.svg'
import dark from '@/img/made-by/dark.png'
import light from '@/img/made-by/light.png'

export function PoweredBy() {
  const { colorMode } = useColorMode()
  return (
    <a href='https://polybaselabs.com' target='_blank'>
      <Image src={colorMode == 'dark' ? dark : light} alt='Made by Polybase Labs' height={37} width={135} />
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