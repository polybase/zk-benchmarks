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
  prop: 'metrics.SHA256.run.time',
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


export function Table() {
  return (
    <div className="mt-8 flow-root">
      <div className="-mx-4 -my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
        <div className="inline-block min-w-full py-2 align-middle sm:px-6 lg:px-8">
          <div className="overflow-hidden shadow ring-1 ring-black ring-opacity-5 sm:rounded-lg">
            <table className="min-w-full divide-y divide-gray-300">
              <thead className="bg-gray-50">
                <tr>
                  <th scope="col" className="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-6">

                  </th>
                  {data.map((item) => (
                    <th key={item.name} scope="col" className="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">
                      {item.name}
                    </th>
                  ))}
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-200 bg-white">
                {properties.map((prop) => {
                  return (
                    <tr key={prop.name}>
                      <td className="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-6">
                        {prop.name}
                      </td>
                      {
                        data.map((fw: any) => {
                          let value = prop.value ? prop.value(getPathValue(fw, prop.prop)) : getPathValue(fw, prop.prop);
                          console.log(value)
                          return (
                            <td key={fw.name} className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
                              {value}
                            </td>
                          )
                        })
                      }
                    </tr>
                  )
                })}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div >
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