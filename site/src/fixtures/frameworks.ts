import benchmarks from './benchmarks.json'

export const frameworks = [
  {
    id: 'polylang',
    name: 'Polylang',
    url: 'https://polylang.xyz',
    frontend: 'Typescript-like',
    zk: 'STARK',
    unbounded: '✅',
    existingLibSupport: '❌',
    gpu: ['Metal'],
    optimisedHash: 'RPO',
    metrics: benchmarks.frameworks.polylang,
  },
  {
    id: 'miden',
    name: 'Miden',
    url: 'https://wiki.polygon.technology/docs/miden/',
    frontend: 'MASM (Assembly)',
    zk: 'STARK',
    unbounded: '✅',
    existingLibSupport: '⚠️',
    gpu: ['Metal'],
    optimisedHash: 'RPO, SHA-256',
    metrics: benchmarks.frameworks.miden,
  },
  {
    id: 'risc_zero',
    name: 'Risc Zero',
    url: 'https://risczero.com',
    frontend: 'Rust, C, C++',
    zk: 'STARK',
    unbounded: '✅',
    existingLibSupport: '✅',
    gpu: ['Metal', 'CUDA'],
    optimisedHash: 'SHA-256',
    metrics: benchmarks.frameworks.risc_zero,
  },
  {
    id: 'noir',
    name: 'Noir',
    url: 'https://noir-lang.org',
    frontend: 'Rust-like',
    zk: 'SNARK',
    unbounded: '❌',
    existingLibSupport: '⚠️',
    optimisedHash: 'SHA-256, Blake2, Pedersen',
    metrics: benchmarks.frameworks.noir,
  }
]
