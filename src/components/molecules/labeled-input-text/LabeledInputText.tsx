import { type FC } from 'react'
import styles from './LabeledInputText.module.scss'
import { InputText } from '../../atoms/input-text/InputText'

interface Props {
  label: string
  length: number
}

export const LabeledInputText: FC<Props> = (props) => {
  return (
    <div className={styles.component}>
      <span>{props.label}</span>
      <InputText length={props.length} />
    </div>
  )
}
