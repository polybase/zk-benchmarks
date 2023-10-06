import './globals.css'
import { Suspense } from 'react'
import type { Metadata } from 'next'
import Head from 'next/head'
import { Inter } from 'next/font/google'
import { Providers } from './providers'
import { PostHogPageview } from '@/components/PostHogPageview'

const inter = Inter({ subsets: ['latin'] })

export const metadata: Metadata = {
  title: 'ZK Bench',
  description: 'Benchmarks for your favourite ZK frameworks',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang='en'>
      <Head>
        <meta name='viewport' content='width=device-width, initial-scale=1.0' />
      </Head>
      <Suspense>
        <PostHogPageview />
      </Suspense>
      <body className={inter.className}>
        <Providers>
          {children}
        </Providers>
      </body>
    </html>
  )
}
