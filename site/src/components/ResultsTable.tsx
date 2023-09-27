import { useState } from 'react'
import Image from 'next/image'
import NextLink from 'next/link'
import {
  TableContainer, Box, Table, Thead, Tbody, Th, Tr, Td, Stack, HStack, Text, Button, IconButton,
  Popover, PopoverTrigger, PopoverContent, PopoverArrow, Portal, PopoverBody, Icon, Spacer, Link,
  useColorModeValue
} from '@chakra-ui/react'
import { MdInfo } from 'react-icons/md'
import { FaExternalLinkAlt } from 'react-icons/fa'
import { frameworks } from '@/fixtures/frameworks'
import benchmarks from '@/fixtures/benchmarks.json'
import bytes from 'bytes'
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
  value?: (val: any, vars: Record<string, any>) => any;
}

const metricFormatter = (empty = '') => (val: any, vars: Record<string, any>) => {
  if (!val) return empty
  if (vars.metric == 'time') {
    return val ? `${(val.secs + val?.nanos / 1000000000).toFixed(2)}s` : null

  }
  return bytes(val)
}

const properties: ResultTableProperty[] = [{
  name: 'Frontend (Language)',
  prop: 'frontend',
  desc: 'Frontend is the technical term for a programming language that is compiled into a lower level language',
}, {
  name: 'ZK',
  prop: 'zk',
  desc: 'The type of ZK used, for a detailed comparison, see the FAQ section below.',
}, {
  name: 'Unbounded Programs',
  prop: 'unbounded',
  desc: 'Unbounded programs allow inputs of variable size and loops where the number of iterations is not known at compile time. See the FAQ below for more detail on unbounded v. bounded programs.',
},
{
  name: 'Audit',
  prop: 'audit',
  desc: 'Has the framework been audited by a reputable security firm? This is generally required before using the framework for mainnet applications.',
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
  name: 'EVM Verifier',
  desc: 'Does the framework provide a verifier that can be used on the Ethereum Virtual Machine?',
  prop: 'evmVerifier',
  annotations: {
    polylang: 'Polylang is scheduled to have an EVM verifier in Q1 2023.',
    miden: 'Miden is scheduled to have an EVM verifier in Q1 2023.',
  }
}, {
  name: 'GPU',
  prop: 'gpu',
  desc: 'Does the framework support GPU acceleration? Metal is specific to Apple devices.',
  value: (val?: string[]) => val ? `✅ ${val.join(', ')}` : "❌",
}, {
  name: 'Assert',
  desc: `A very simple assertion a != b, this can be used to test the framework's minimum proving performance.`,
  prop: 'metrics.$machine.assert.results.0.$metric',
  value: metricFormatter(),
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
  name: 'SHA-256 Hash',
  desc: `Calculating the SHA-256 hash for given input size. SHA-256 is NOT zk optimised so it's normal to see degraded performance compared to other hashes. You SHOULD use an alterantive ZK-optimised hash if your use case allows and the framework provides it.`,
  // prop: 'metrics.$machine.SHA256.results.0.time',
  // value: (val?: Duration) => val ? `${(val.secs + val?.nanos / 1000000000).toFixed(2)}s` : null,
},
{
  name: '1k bytes',
  indent: 4,
  prop: 'metrics.$machine.SHA256.results.0.$metric',
  value: metricFormatter(),
  annotations: {
    leo: 'We used SHA3, as no SHA2 was available'
  }
},
{
  name: '10k bytes',
  indent: 4,
  prop: 'metrics.$machine.SHA256.results.1.$metric',
  value: metricFormatter(),
  annotations: {
    leo: 'We used SHA3, as no SHA2 was available'
  }
}, {
  name: 'Pedersen Hash',
  desc: `A ZK optimised hash, this should perform better than SHA-256.`,
},
{
  name: '1k bytes',
  indent: 4,
  prop: 'metrics.$machine.Pedersen.results.0.$metric',
  value: metricFormatter('❌'),
  annotations: {
    risc_zero: 'Risc Zero does not support Pedersen',
    polylang: 'Miden does not support Pedersen',
    miden: 'Miden does not support Pedersen',
  }
},
{
  name: '10k bytes',
  indent: 4,
  prop: 'metrics.$machine.Pedersen.results.1.$metric',
  value: metricFormatter('❌'),
  annotations: {
    polylang: 'Miden does not support Pedersen',
    risc_zero: 'Risc Zero does not support Pedersen',
    miden: 'Miden does not support Pedersen',
  }
}, {
  name: 'RPO Hash',
  desc: `A ZK optimised hash, this should perform better than SHA-256.`,
},
{
  name: '1k bytes',
  indent: 4,
  prop: 'metrics.$machine.RPO.results.0.$metric',
  value: metricFormatter('❌'),
  annotations: {
    risc_zero: 'Risc Zero does not support RPO',
    noir: 'Noir does not support RPO, but does support Pederson which is a ZK optimised hash.',
  }
},
{
  name: '10k bytes',
  indent: 4,
  prop: 'metrics.$machine.RPO.results.1.$metric',
  value: metricFormatter('❌'),
  annotations: {
    risc_zero: 'Risc Zero does not support RPO',
    noir: 'Noir does not support RPO, but does support Pederson which is a ZK optimised hash.',
  }
}, {
  name: 'Fibonacci',
  // TODO: use markdown for this
  desc: `A fibonacci sequence is calculated for a given input size. This is a good test of the framework's ability to handle a looping data-strcuture.`,
  // prop: 'metrics.$machine.SHA256.results.0.time',
  // value: (val?: Duration) => val ? `${(val.secs + val?.nanos / 1000000000).toFixed(2)}s` : null,
},
{
  name: '1',
  indent: 4,
  prop: 'metrics.$machine.Fibonacci.results.0.$metric',
  value: metricFormatter(),
  annotations: {
    risc_zero: 'Slower due to minimum 64k cycles, regardless of program complexity',
    noir: 'We use bounded program, which is probably not a fair comparison. This will be updated to use recursive proofs.'
  }
},
{
  name: '10',
  indent: 4,
  prop: 'metrics.$machine.Fibonacci.results.1.$metric',
  value: metricFormatter(),
  annotations: {
    risc_zero: 'Slower due to minimum 64k cycles, regardless of program complexity',
    noir: 'We use bounded program, which is probably not a fair comparison. This will be updated to use recursive proofs.'
  }
},
{
  name: '100',
  indent: 4,
  prop: 'metrics.$machine.Fibonacci.results.2.$metric',
  value: metricFormatter(),
  annotations: {
    risc_zero: 'Slower due to minimum 64k cycles, regardless of program complexity',
    noir: 'We use bounded program, which is probably not a fair comparison. This will be updated to use recursive proofs.'
  }
},
{
  name: '1,000',
  indent: 4,
  prop: 'metrics.$machine.Fibonacci.results.3.$metric',
  value: metricFormatter(),
  annotations: {
    risc_zero: 'Slower due to minimum 64k cycles, regardless of program complexity',
    noir: 'We use bounded program, which is probably not a fair comparison. This will be updated to use recursive proofs.'
  }
}, {
  name: '10,000',
  indent: 4,
  prop: 'metrics.$machine.Fibonacci.results.4.$metric',
  value: metricFormatter(),
  annotations: {
    noir: 'We use bounded program, which is probably not a fair comparison. This will be updated to use recursive proofs.'
  }
},
{
  name: '100,000',
  indent: 4,
  prop: 'metrics.$machine.Fibonacci.results.5.$metric',
  value: metricFormatter('🚧'),
  annotations: {
    noir: 'We use bounded program, which is probably not a fair comparison. This will be updated to use recursive proofs.'
  }
}, {
  name: 'Merkle Tree',
}, {
  name: 'Membership Proof',
  prop: 'metrics.$machine.Merkle Membership.results.0.$metric',
  value: metricFormatter('🚧'),
  indent: 4,
}, {
  name: 'Merge',
  indent: 4,
}, {
  name: '1 + 1',
  indent: 8,
  prop: 'metrics.$machine.Merkle Tree Merge.results.0.$metric',
  value: metricFormatter('🚧'),
}, /* {
  name: '2^10 + 2^10',
  indent: 8,
  prop: 'metrics.$machine.Merkle Tree Merge.results.1.$metric',
  value: metricFormatter,
}, {
  name: '2^10 + 2^20',
  indent: 8,
  prop: 'metrics.$machine.Merkle Tree Merge.results.2.$metric',
  value: metricFormatter,
}, {
  name: '2^20 + 2^20',
  indent: 8,
  prop: 'metrics.$machine.Merkle Tree Merge.results.3.$metric',
  value: metricFormatter,
}*/]


const machines = [{
  name: '16x CPU',
  prop: 'ubuntu-16-shared',
}, {
  name: '64x CPU',
  prop: 'ubuntu-latest-64-cores',
}]

const metrics = [{
  name: 'Time',
  prop: 'time',
}, {
  name: 'Memory',
  prop: 'metrics.memory_usage_bytes',
}, {
  name: 'Proof Size',
  prop: 'metrics.proof_size_bytes',
}]

export function ResultsTable() {
  const colorMode = useColorModeValue('light', 'dark')
  const [machine, setMachine] = useState(machines[0].prop)
  const [metric, setMetric] = useState(metrics[0].prop)
  const vars = {
    machine,
    metric,
  }

  return (
    <Stack fontSize='sm' spacing={4}>
      <HStack>
        <HStack>
          {machines.map(({ name, prop }) => {
            const selected = machine === prop;
            return (
              <Button size='sm' variant={selected ? 'solid' : 'ghost'} key={prop} onClick={() => {
                setMachine(prop)
              }}>{name}</Button>
            )
          })}
          <Popover trigger='hover' placement='top'>
            <PopoverTrigger>
              <Button size='sm' opacity='0.4' variant='ghost'>CUDA</Button>
            </PopoverTrigger>
            <PopoverContent>
              <PopoverArrow />
              <PopoverBody><Text overflowWrap='anywhere' fontSize='sm'>Follow on Twitter <Link as={NextLink} color='blue.600' href='https://twitter.com/intent/user?screen_name=polybase_xyz'>@polybase_xyz </Link>to be notified when we add this</Text></PopoverBody>
            </PopoverContent>
          </Popover>
        </HStack>
        <Spacer />
        <HStack>
          {metrics.map(({ name, prop }) => {
            const selected = metric === prop;
            return (
              <Button size='sm' variant={selected ? 'solid' : 'ghost'} key={prop} onClick={() => {
                setMetric(prop)
              }}>{name}</Button>
            )
          })}
        </HStack>
      </HStack>
      <Box border='1px solid' borderBottomWidth={0} borderColor='bw.100' borderRadius={5}>
        <TableContainer overflowX="unset" overflowY="unset">
          <Table>
            <Thead>
              <Tr>
                <Th position='sticky' top={0} background='bws' zIndex={1000}>
                </Th>
                {frameworks.map((item) => (
                  <Th key={item.name} fontSize='sm' position='sticky' top={0} background='bws' zIndex={1000}>
                    <a href={item.url} target='_blank'>
                      <Stack spacing={2}>
                        <Box textDecorationColor='#fff'>
                          <Image
                            alt={item.name}
                            src={item.logo.src[colorMode]}
                            height={item.logo.height}
                            width={item.logo.width}
                          />
                        </Box>
                        <Box>{item.name} <Icon opacity='0.4' fontSize='xs' as={FaExternalLinkAlt} /></Box>
                      </Stack>
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
                        let value = prop.value ? prop.value(getPathValue(fw, prop.prop, vars), vars) : getPathValue(fw, prop.prop, vars);
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
    </Stack >
  )
}


function getPathValue(data: any, path?: string, vars?: Record<string, any>) {
  if (!path) return
  let current = data;
  for (let part of path.split('.')) {
    if (!current) return undefined;
    if (part.startsWith('$')) {
      part = vars?.[part.slice(1)]
      if (part.split('.').length > 1) {
        for (let sub of part.split('.')) {
          current = current[sub]
        }
        continue
      }
    }
    current = current[part]
  }
  return current;
}
