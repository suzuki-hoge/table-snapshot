import { type FC } from 'react'
import styles from './ColorTag.module.scss'
import { type Color } from '../../../types/Color'

interface Props {
  variant: Color
}

const colors = {
  red: styles.red,
  yellow: styles.yellow,
  green: styles.green,
  blue: styles.blue,
  purple: styles.purple,
}

export const ColorTag: FC<Props> = (props) => {
  return <span className={colors[props.variant]}></span>
}
