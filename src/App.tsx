import { type FC } from 'react'
import { Button } from './components/atoms/button/Button'
import './global.scss'

export const App: FC = () => {
  return (
    <>
      <Button label={'foo'} onClick={() => {}} />
      <h1>Table Snapshot</h1>
    </>
  )
}
