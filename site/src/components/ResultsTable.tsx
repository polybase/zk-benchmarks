import { useState } from 'react'
import {
  TableContainer, Box, Table, Thead, Tbody, Th, Tr, Td, Stack, HStack, Text, Button, IconButton,
  Popover, PopoverTrigger, PopoverContent, PopoverArrow, PopoverCloseButton, Portal, PopoverBody
} from '@chakra-ui/react'
import { MdInfo } from 'react-icons/md'
import { frameworks } from '@/fixtures/frameworks'
import benchmarks from '@/fixtures/benchmarks.json'
import { timeSinceLastUpdate } from '@/util/date'

interface Duration {
  secs: number;
  nanos: number;
}

interface ResultTableProperty {
  name: string;
  desc?: string | JSX.Element;
  prop?: string;
  indent?: number;
  annotations?: Record<string, string | JSX.Element>
  value?: (val: any) => any;
}

const properties: ResultTableProperty[] = [{
  name: 'Frontend',
  prop: 'frontend',
}, {
  name: 'ZK',
  prop: 'zk',
  desc: 'The type of ZK used, for a detailed comparison, see the FAQ section below.',
}, {
  name: 'Unbounded Programs',
  prop: 'unbounded',
  desc: 'Unbounded programs allow inputs of variable size and loops where the number of iterations is not known at compile time. See the FAQ below for more detail on unbounded v. bounded programs.',
}, {
  name: 'External Libraries',
  desc: 'Does the framework allow leveraging a languages existing library ecosystem? For example, in Rust this would be crates.io.',
  prop: 'existingLibSupport',
  // value: (val: boolean) => val ? "✅" : "❌",
  annotations: {
    miden: 'Miden does support loading library modules, but these must also be written in Miden. There is no existing library ecosystem for Miden.',
    risc_zero: 'Risc Zero supports most crates in the Rust ecosystem. This is one of the powerful features of Risc Zero.',
    noir: 'Noir does support loading library modules, but these must also be written in Noir. There is no existing library ecosystem for Noir.',
  }
}, {
  name: 'GPU',
  prop: 'gpu',
  desc: 'Does the framework support GPU acceleration? Metal is specific to Apple devices.',
  value: (val?: string[]) => val ? `✅ ${val.join(', ')}` : "❌",
}, {
  name: 'Assert',
  desc: `A very simple assertion a != b, this can be used to test the framework's minimum proving performance.`,
  prop: 'metrics.$machine.assert.results.0.time',
  value: (val?: Duration) => val ? `${(val.secs + val?.nanos / 1000000000).toFixed(2)}s` : null,
  annotations: {
    risc_zero: 'Risc Zero is significantly slower for this test, as the minimum number of cycles for all Risc Zero programs is 64k. Therefore this very small program still requires a large number of cycles.',
  }
}, {
  name: 'Optimised Hashes',
  prop: 'optimisedHash',
  desc: `Hashes that have been optimised by the framework and therefore should perform faster. SHA-256 and Blake are not optimised for ZK in general, but may still be optimised by a framework.`,
  annotations: {
    risc_zero: 'SHA-256 is the most optimised hash for Risc Zero, but SHA-256 is in general not ZK optimised.'
  }
}, {
  name: 'SHA-256',
  desc: `Calculating the SHA-256 hash for given input size. SHA-256 is NOT zk optimised so it's normal to see degraded performance compared to other hashes.`,
  // prop: 'metrics.$machine.SHA256.results.0.time',
  // value: (val?: Duration) => val ? `${(val.secs + val?.nanos / 1000000000).toFixed(2)}s` : null,
},
{
  name: '1k bytes',
  indent: 4,
  prop: 'metrics.$machine.SHA256.results.0.time',
  value: (val?: Duration) => val ? `${(val.secs + val?.nanos / 1000000000).toFixed(2)}s` : null,
},
{
  name: '10k bytes',
  indent: 4,
  prop: 'metrics.$machine.SHA256.results.1.time',
  value: (val?: Duration) => val ? `${(val.secs + val?.nanos / 1000000000).toFixed(2)}s` : null,
}, {
  name: 'Fibonacci',
  // TODO: use markdown for this
  desc: `A fibonacci sequence is calculated for a given input size. This is a good test of the framework's ability to handle recursion.`,
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
      <Box border='1px solid' borderBottomWidth={0} borderColor='bw.100' borderRadius={5}>
        <TableContainer>
          <Table>
            <Thead>
              <Tr>
                <Th>
                </Th>
                {frameworks.map((item) => (
                  <Th key={item.name} fontSize='sm'>
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
                      <HStack pl={prop.indent ?? 0} spacing={1}>
                        <Box>
                          {prop.name}
                        </Box>

                        {prop.desc && (
                          <Box>
                            <Popover>
                              <PopoverTrigger>
                                <IconButton opacity={0.3} variant='ghost' aria-label='info' height='18px' size='sm' icon={<MdInfo />} />
                              </PopoverTrigger>
                              <Portal>
                                <PopoverContent>
                                  <PopoverArrow />
                                  <PopoverBody><Text overflowWrap='anywhere' fontSize='sm'>{prop.desc}</Text></PopoverBody>
                                </PopoverContent>
                              </Portal>
                            </Popover>
                          </Box>
                        )}
                      </HStack>
                    </Td>
                    {
                      frameworks.map((fw: any) => {
                        let value = prop.value ? prop.value(getPathValue(fw, prop.prop, vars)) : getPathValue(fw, prop.prop, vars);
                        const annotation = prop.annotations?.[fw.id]
                        return (
                          <Td key={fw.name}>
                            <HStack spacing={1}>
                              <Box>
                                {value}
                              </Box>
                              {annotation && (
                                <Box>
                                  <Popover>
                                    <PopoverTrigger>
                                      <IconButton opacity={0.3} variant='ghost' height='18px' aria-label='info' size='sm' icon={<MdInfo />} />
                                    </PopoverTrigger>
                                    <Portal>
                                      <PopoverContent>
                                        <PopoverArrow />
                                        <PopoverBody><Text overflowWrap='anywhere' fontSize='sm'>{annotation}</Text></PopoverBody>
                                      </PopoverContent>
                                    </Portal>
                                  </Popover>
                                </Box>
                              )}
                            </HStack>
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
            {timeSinceLastUpdate(benchmarks.meta.lastUpdated)} (<time>{benchmarks.meta.lastUpdated}</time>)
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
