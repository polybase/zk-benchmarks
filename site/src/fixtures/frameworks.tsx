import benchmarks from './benchmarks.json'
import { Box, HStack, Text, Tooltip } from '@chakra-ui/react'
import midenLogo from '@/img/frameworks/miden.png'
import noirLogo from '@/img/frameworks/noir.svg'
import polylangLogo from '@/img/frameworks/polylang.png'
import riscZeroLogo from '@/img/frameworks/risc-zero.png'
import leoLogo from '@/img/frameworks/leo.svg'
import leoLightLogo from '@/img/frameworks/leo-light.svg'

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
      src: {
        light: polylangLogo,
        dark: polylangLogo,
      }
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
      src: {
        light: midenLogo,
        dark: midenLogo,
      }
    },
    url: 'https://wiki.polygon.technology/docs/miden/',
    frontend: 'MASM (Assembly)',
    zk: 'STARK / zkVM',
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
      src: {
        light: riscZeroLogo,
        dark: riscZeroLogo,
      }
    },
    url: 'https://risczero.com',
    frontend: 'Rust, C, C++',
    zk: 'STARK / zkVM',
    unbounded: '✅',
    existingLibSupport: '✅',
    audit: '❌ Planned 2024',
    evmVerifier: '✅',
    gpu: ['Metal', 'CUDA'],
    optimisedHash: 'SHA-256',
    metrics: benchmarks.frameworks['risc-zero'],
  },
  {
    id: 'noir',
    name: 'Noir (Barretenberg)',
    logo: {
      height: 30,
      width: 80,
      src: {
        light: noirLogo,
        dark: noirLogo,
      },
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
  },
  {
    id: 'leo',
    name: 'Leo',
    logo: {
      height: 40,
      width: 46,
      src: {
        light: leoLightLogo,
        dark: leoLogo,
      },
    },
    url: 'https://leo-lang.org/',
    frontend: 'Leo (DSL)',
    zk: 'SNARK',
    unbounded: '❌',
    existingLibSupport: '⚠️',
    audit: '❌ Planned 2023',
    evmVerifier: '❌',
    optimisedHash: <MoreInfo count={3} more='SHA3-256, Keccak256, Poseidon, BHP'>Pedersen</MoreInfo>,
    metrics: benchmarks.frameworks.leo,
  }
]
