import React, { type FC } from 'react'
import { type Diff } from '../../../types/Tmp'
import { DiffTable } from '../../molecules/diff-table/DiffTable'
import styles from './DiffContent.module.scss'

interface Props {
  diff: Diff
}

export const DiffContent: FC<Props> = (props) => {
  return (
    <div className={styles.component}>
      <span className={styles.label}>{props.diff.tableName}</span>
      <DiffTable {...props.diff} />
    </div>
  )
}
