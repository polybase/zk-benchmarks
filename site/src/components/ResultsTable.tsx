import { TableContainer, Box, Table, Thead, Tbody, Th, Tr, Td } from '@chakra-ui/react'

import midenSingleCPU from '@/fixtures/miden-single-cpu.json'
import midenMultiCPU from '@/fixtures/miden-multi-cpu.json'
import midenMetal from '@/fixtures/miden-metal.json'
import riscZeroMultiCPU from '@/fixtures/risc_zero-multi-cpu.json'
import riscZeroMetal from '@/fixtures/risc_zero-metal.json'

interface Duration {
  secs: number;
  nanos: number;
}

const properties = [{
  name: 'Frontend',
  prop: 'frontend',
}, {
  name: 'ZK',
  prop: 'zk'
}, {
  name: 'External Libraries',
  prop: 'existingLibSupport',
  desc: 'Does the framework allow leveraging a languages existing library ecosystem?',
  value: (val: boolean) => val ? "✅" : "❌",
}, {
  name: 'GPU',
  prop: 'gpu',
  desc: 'Does the framework support GPU acceleration?',
  value: (val?: string[]) => val ? `✅ ${val.join(', ')}` : "❌",
}, {
  name: 'SHA-256',
  prop: 'metrics.singleCPU.SHA256.run.time',
  value: (val?: Duration) => val ? `${(val.secs + val?.nanos / 1000000000).toFixed(2)}s` : null,
}]

const data = [
  {
    name: 'Polylang',
    url: 'https://polylang.xyz',
    frontend: 'Typescript-like',
    zk: 'STARK',
    existingLibSupport: false,
    gpu: ['Metal'],
    metrics: { singleCPU: midenSingleCPU.timings, multiCPU: midenMultiCPU.timings, metal: midenMetal.timings },
  },
  {
    name: 'Risc Zero',
    url: 'https://risczero.com',
    frontend: 'Rust, C, C++',
    zk: 'STARK',
    existingLibSupport: true,
    gpu: ['Metal', 'CUDA'],
    metrics: { multiCPU: riscZeroMultiCPU.timings, metal: riscZeroMetal.timings },
  },
  {
    name: 'Noir',
    url: 'https://noir-lang.org',
    frontend: 'Rust-like',
    zk: 'SNARK',
    existingLibSupport: false,
    metrics: {}
  }
]


export function ResultsTable() {
  return (
    <Box fontSize='sm' border='1px solid' borderBottomWidth={0} borderColor='whiteAlpha.300' borderRadius={5}>
      <TableContainer>
        <Table>
          <Thead>
            <Tr>
              <Th>
              </Th>
              {data.map((item) => (
                <Th key={item.name}>
                  {item.name}
                </Th>
              ))}
            </Tr>
          </Thead>
          <Tbody>
            {properties.map((prop) => {
              return (
                <Tr key={prop.name}>
                  <Td fontWeight='600'>
                    {prop.name}
                  </Td>
                  {
                    data.map((fw: any) => {
                      let value = prop.value ? prop.value(getPathValue(fw, prop.prop)) : getPathValue(fw, prop.prop);
                      return (
                        <Td key={fw.name}>
                          {value}
                        </Td>
                      )
                    })
                  }
                </Tr>
              )
            })}
          </Tbody>
        </Table>
      </TableContainer>
    </Box>
  )
}

function getPathValue(data: any, path: string) {
  let current = data;
  for (const part of path.split('.')) {
    if (!current) return undefined;
    console.log(current, part)
    current = current[part]
  }
  return current;
}