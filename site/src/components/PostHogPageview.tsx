'use client'

import { useEffect } from 'react'
import posthog from 'posthog-js'
import { usePathname, useSearchParams } from 'next/navigation'

export function PostHogPageview() {
    const pathname = usePathname()
    const searchParams = useSearchParams()
    // Track pageviews
    useEffect(() => {
        if (pathname) {
            let url = window.origin + pathname
            if (searchParams.toString()) {
                url = url + `?${searchParams.toString()}`
            }
            posthog.capture(
                '$pageview',
                {
                    '$current_url': url,
                }
            )
        }
    }, [pathname, searchParams])

    return null
}