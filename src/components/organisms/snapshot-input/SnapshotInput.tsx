import { type FC, useState } from 'react'
import styles from './SnapshotInput.module.scss'
import { type Snapshot } from '../../../types/Tmp'
import { InputText } from '../../atoms/input-text/InputText'

interface Props {
  snapshot?: Snapshot
}

export const SnapshotInput: FC<Props> = (props) => {
  const [title, setTitle] = useState(props.snapshot?.title ?? '')

  return (
    <div className={styles.component}>
      <div className={styles.item}>
        <span>Name</span>
        <InputText
          value={title}
          length={20}
          onInput={(e) => {
            setTitle(e.target.value)
          }}
        />
      </div>
    </div>
  )
}
