import remarkGfm from 'remark-gfm'
import createMDX from '@next/mdx'
import rehypeHighlight from 'rehype-highlight'
import rehypeSlug from 'rehype-slug'

import langJs from 'highlight.js/lib/languages/javascript'

/** @type {import('next').NextConfig} */
const nextConfig = {}

const withMDX = createMDX({
  options: {
    extension: /\.mdx?$/,
    remarkPlugins: [remarkGfm],
    rehypePlugins: [rehypeSlug, rehypeHighlight, { languages: { javascript: langJs } }]
    // If you use `MDXProvider`, uncomment the following line.
    // providerImportSource: "@mdx-js/react",
  },
})
export default withMDX(nextConfig)

