import { type FC, type ReactNode } from 'react'
import styles from './Header.module.scss'

interface Props {
  globals: ReactNode
  locals: ReactNode
}

export const Header: FC<Props> = (props) => {
  return (
    <>
      <div className={styles.component}>
        <div className={styles.icons}>{props.globals}</div>
        <div className={styles.icons}>{props.locals}</div>
      </div>
      <hr className={styles.line} />
    </>
  )
}
