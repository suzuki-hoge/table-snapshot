import { type FC } from 'react'
import { type Color } from '../../../types/Color'
import { ColorTag } from '../../atoms/color-tag/ColorTag'
import styles from './ColorTagCard.module.scss'

interface Props {
  label: string
  variant: Color
  onClick: () => void
}

export const ColorTagCard: FC<Props> = (props) => {
  return (
    <div className={styles.component} onClick={props.onClick}>
      <ColorTag variant={props.variant} />
      <span>{props.label}</span>
    </div>
  )
}
