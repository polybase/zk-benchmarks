import benchmarks from './benchmarks.json'
import { Box, HStack, Text, Tooltip } from '@chakra-ui/react'
import midenLogo from '@/img/frameworks/miden.png'
import noirLogo from '@/img/frameworks/noir.svg'
import polylangLogo from '@/img/frameworks/polylang.png'
import riscZeroLogo from '@/img/frameworks/risc-zero.png'

const MoreInfo = ({ children, count, more }: any) => (
  <HStack><Text>{children}</Text><Tooltip label={more}><Text color='blue.700' cursor='pointer'>+{count} more</Text></Tooltip></HStack>
)

export const frameworks = [
  {
    id: 'polylang',
    name: 'Polylang',
    logo: {
      height: 30,
      width: 30,
      src: polylangLogo,
    },
    url: 'https://polylang.dev',
    frontend: 'Typescript-like',
    zk: 'STARK',
    unbounded: '✅',
    existingLibSupport: '❌',
    audit: '❌ Planned 2024',
    evmVerifier: '⚠️',
    gpu: ['Metal'],
    optimisedHash: <MoreInfo count={2} more='Blake3, SHA-256'>RPO</MoreInfo>,
    metrics: benchmarks.frameworks.polylang,
  },
  {
    id: 'miden',
    name: 'Miden',
    logo: {
      height: 30,
      width: 30,
      src: midenLogo,
    },
    url: 'https://wiki.polygon.technology/docs/miden/',
    frontend: 'MASM (Assembly)',
    zk: 'STARK',
    unbounded: '✅',
    existingLibSupport: '⚠️',
    audit: '❌ Planned 2024',
    evmVerifier: '⚠️',
    gpu: ['Metal'],
    optimisedHash: <MoreInfo count={2} more='Blake3, SHA-256'>RPO</MoreInfo>,
    metrics: benchmarks.frameworks.miden,
  },
  {
    id: 'risc_zero',
    name: 'Risc Zero',
    logo: {
      height: 30,
      width: 30,
      src: riscZeroLogo,
    },
    url: 'https://risczero.com',
    frontend: 'Rust, C, C++',
    zk: 'STARK',
    unbounded: '✅',
    existingLibSupport: '✅',
    audit: '❌ Planned 2024',
    evmVerifier: '✅',
    gpu: ['Metal', 'CUDA'],
    optimisedHash: 'SHA-256',
    metrics: benchmarks.frameworks.risc_zero,
  },
  {
    id: 'noir',
    name: 'Noir',
    logo: {
      height: 30,
      width: 80,
      src: noirLogo,
    },
    url: 'https://noir-lang.org',
    frontend: 'Rust-like',
    zk: 'SNARK',
    unbounded: '❌',
    existingLibSupport: '⚠️',
    audit: '❌ Planned 2024',
    evmVerifier: '✅',
    optimisedHash: <MoreInfo count={2} more='SHA-256, Blake2'>Pedersen</MoreInfo>,
    metrics: benchmarks.frameworks.noir,
  }
]
