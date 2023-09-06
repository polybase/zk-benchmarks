import { useState } from 'react'
import { TableContainer, Box, Table, Thead, Tbody, Th, Tr, Td, Stack, HStack, Text, Button } from '@chakra-ui/react'
import { frameworks } from '@/fixtures/frameworks'
import benchmarks from '@/fixtures/benchmarks.json'
import { formatDate, timeSinceLastUpdate } from '@/util/date'

interface Duration {
  secs: number;
  nanos: number;
}

interface ResultTableProperty {
  name: string;
  desc?: string;
  prop?: string;
  indent?: number;
  value?: (val: any) => any;
}

const properties: ResultTableProperty[] = [{
  name: 'Frontend',
  prop: 'frontend',
}, {
  name: 'ZK',
  prop: 'zk'
}, {
  name: 'External Libraries',
  desc: 'Does the framework allow leveraging a languages existing library ecosystem? For example, in Rust this would be crates.io.',
  prop: 'existingLibSupport',
  value: (val: boolean) => val ? "✅" : "❌",
}, {
  name: 'GPU',
  prop: 'gpu',
  desc: 'Does the framework support GPU acceleration? Metal is a specific to Apple devices.',
  value: (val?: string[]) => val ? `✅ ${val.join(', ')}` : "❌",
}, {
  name: 'Assert',
  desc: `A very simple assertion a != b, this can be used to test the frameworks minimum proving performance.`,
  prop: 'metrics.$machine.assert.results.0.time',
  value: (val?: Duration) => val ? `${(val.secs + val?.nanos / 1000000000).toFixed(2)}s` : null,
}, {
  name: 'SHA-256',
  desc: `Calculating the SHA-256 hash for given input size. SHA-256 is NOT zk optimised so it's normal to see degraded performance compared to other hashes.`,
  // prop: 'metrics.$machine.SHA256.results.0.time',
  // value: (val?: Duration) => val ? `${(val.secs + val?.nanos / 1000000000).toFixed(2)}s` : null,
},
{
  name: '1 byte',
  indent: 4,
  prop: 'metrics.$machine.SHA256.results.0.time',
  value: (val?: Duration) => val ? `${(val.secs + val?.nanos / 1000000000).toFixed(2)}s` : null,
},
{
  name: '10 bytes',
  indent: 4,
  prop: 'metrics.$machine.SHA256.results.1.time',
  value: (val?: Duration) => val ? `${(val.secs + val?.nanos / 1000000000).toFixed(2)}s` : null,
},
{
  name: '100 bytes',
  indent: 4,
  prop: 'metrics.$machine.SHA256.results.2.time',
  value: (val?: Duration) => val ? `${(val.secs + val?.nanos / 1000000000).toFixed(2)}s` : null,
},
{
  name: '1000 bytes',
  indent: 4,
  prop: 'metrics.$machine.SHA256.results.3.time',
  value: (val?: Duration) => val ? `${(val.secs + val?.nanos / 1000000000).toFixed(2)}s` : null,
}, {
  name: 'Fibonacci',
  // TODO: use markdown for this
  desc: `A fibonacci sequence is calculated for a given input size. This is a good test of the frameworks ability to handle recursion.`,
  // prop: 'metrics.$machine.SHA256.results.0.time',
  // value: (val?: Duration) => val ? `${(val.secs + val?.nanos / 1000000000).toFixed(2)}s` : null,
},
{
  name: '1',
  indent: 4,
  prop: 'metrics.$machine.Fibonacci.results.0.time',
  value: (val?: Duration) => val ? `${(val.secs + val?.nanos / 1000000000).toFixed(2)}s` : null,
},
{
  name: '10',
  indent: 4,
  prop: 'metrics.$machine.Fibonacci.results.1.time',
  value: (val?: Duration) => val ? `${(val.secs + val?.nanos / 1000000000).toFixed(2)}s` : null,
},
{
  name: '100',
  indent: 4,
  prop: 'metrics.$machine.Fibonacci.results.2.time',
  value: (val?: Duration) => val ? `${(val.secs + val?.nanos / 1000000000).toFixed(2)}s` : null,
},
{
  name: '1,000',
  indent: 4,
  prop: 'metrics.$machine.Fibonacci.results.3.time',
  value: (val?: Duration) => val ? `${(val.secs + val?.nanos / 1000000000).toFixed(2)}s` : null,
}, {
  name: '10,000',
  indent: 4,
  prop: 'metrics.$machine.Fibonacci.results.4.time',
  value: (val?: Duration) => val ? `${(val.secs + val?.nanos / 1000000000).toFixed(2)}s` : null,
},
{
  name: '100,000',
  indent: 4,
  prop: 'metrics.$machine.Fibonacci.results.5.time',
  value: (val?: Duration) => val ? `${(val.secs + val?.nanos / 1000000000).toFixed(2)}s` : null,
}]


const machines = [{
  name: '16x CPU',
  prop: 'ubuntu-16-shared',
}, {
  name: '64x CPU',
  prop: 'ubuntu-latest-64-cores',
}]

export function ResultsTable() {
  const [machine, setMachine] = useState(machines[0].prop)
  const vars = {
    machine,
  }

  return (
    <Stack fontSize='sm' spacing={4}>
      <HStack>
        {machines.map(({ name, prop }) => {
          const selected = machine === prop;
          return (
            <Button size='sm' variant={selected ? 'solid' : 'ghost'} key={prop} onClick={() => {
              setMachine(prop)
            }}>{name}</Button>
          )
        })}
      </HStack>
      <Box border='1px solid' borderBottomWidth={0} borderColor='whiteAlpha.300' borderRadius={5}>
        <TableContainer>
          <Table>
            <Thead>
              <Tr>
                <Th>
                </Th>
                {frameworks.map((item) => (
                  <Th key={item.name}>
                    <a href={item.url}>
                      {item.name}
                    </a>
                  </Th>
                ))}
              </Tr>
            </Thead>
            <Tbody>
              {properties.map((prop) => {
                return (
                  <Tr key={prop.name}>
                    <Td fontWeight='600'>
                      <Box pl={prop.indent ?? 0}>
                        {prop.name}
                      </Box>
                    </Td>
                    {
                      frameworks.map((fw: any) => {
                        let value = prop.value ? prop.value(getPathValue(fw, prop.prop, vars)) : getPathValue(fw, prop.prop, vars);
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
      <Box px={2}>
        <HStack spacing={1} fontStyle='italic'>
          <Text fontWeight={600}>
            Last Updated:
          </Text>
          <Box>
            {timeSinceLastUpdate(benchmarks.meta.lastUpdated)} (<time>{formatDate(benchmarks.meta.lastUpdated)}</time>)
          </Box>
        </HStack>
      </Box>
    </Stack>
  )
}


function getPathValue(data: any, path?: string, vars?: Record<string, any>) {
  if (!path) return
  let current = data;
  for (let part of path.split('.')) {
    if (!current) return undefined;
    if (part.startsWith('$')) {
      part = vars?.[part.slice(1)]
    }
    current = current[part]
  }
  return current;
}
