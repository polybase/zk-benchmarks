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

if (typeof window !== 'undefined' && NEXT_PUBLIC_POSTHOG_KEY) {
  posthog.init(NEXT_PUBLIC_POSTHOG_KEY, {
    api_host: NEXT_PUBLIC_POSTHOG_HOST,
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

