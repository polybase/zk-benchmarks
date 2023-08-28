import Image from 'next/image'
import logo from '@/img/logo.png'
import { Table } from '@/components/Table'

export default function Home() {
  return (
    <main className="flex flex-col items-center justify-between p-24">
      <div className='flex gap-8 flex-col'>
        <div className='flex flex-col items-center justify-between'>
          <div className='flex gap-2'>
            <Image width={50} height={50} alt='zk-bench' src={logo} />
            <h1 className="text-4xl font-bold font-display">zk-bench</h1>
          </div>
        </div>

        <div>
          <Table />
        </div>
      </div>
    </main >
  )
}
