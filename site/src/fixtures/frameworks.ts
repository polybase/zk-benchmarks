import benchmarks from './benchmarks.json'

export const frameworks = [
  {
    name: 'Polylang',
    url: 'https://polylang.xyz',
    frontend: 'Typescript-like',
    zk: 'STARK',
    existingLibSupport: false,
    gpu: ['Metal'],
    metrics: benchmarks.frameworks.miden,
  },
  {
    name: 'Miden',
    url: 'https://wiki.polygon.technology/docs/miden/',
    frontend: 'MASM (Assembly)',
    zk: 'STARK',
    existingLibSupport: false,
    gpu: ['Metal'],
    metrics: benchmarks.frameworks.miden,
  },
  {
    name: 'Risc Zero',
    url: 'https://risczero.com',
    frontend: 'Rust, C, C++',
    zk: 'STARK',
    existingLibSupport: true,
    gpu: ['Metal', 'CUDA'],
    metrics: benchmarks.frameworks.risc_zero,
  },
  {
    name: 'Noir',
    url: 'https://noir-lang.org',
    frontend: 'Rust-like',
    zk: 'SNARK',
    existingLibSupport: false,
    metrics: benchmarks.frameworks.noir,
  }
]
