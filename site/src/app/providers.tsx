'use client'

import { CacheProvider } from '@chakra-ui/next-js'
import { ChakraProvider, ColorModeScript } from '@chakra-ui/react'
import { theme } from './theme'
import posthog from 'posthog-js'
import { PostHogProvider } from 'posthog-js/react'

const {
  NEXT_PUBLIC_POSTHOG_KEY,
  NEXT_PUBLIC_POSTHOG_HOST
} = process?.env ?? {}

console.log(process?.env)

if (typeof window !== 'undefined') {
  posthog.init('phc_7qOsZoc2928qXJa4LlQQS8qj7pVdqtrv7PCk5wdLYp7', {
    api_host: 'https://app.posthog.com',
    capture_pageview: false // Disable automatic pageview capture, as we capture manually
  })
}

export function Providers({
  children
}: {
  children: React.ReactNode
}) {
  return (
    <CacheProvider>
      <PostHogProvider client={posthog}>
        <ColorModeScript initialColorMode='system' />
        <ChakraProvider theme={theme}>
          {children}
        </ChakraProvider>
      </PostHogProvider>
    </CacheProvider>
  )
}

