import { type FC } from 'react'
import styles from './IconEdit.module.scss'
import { BsPencil } from 'react-icons/bs'

interface Props {
  variant: 'small' | 'medium' | 'large'
  onClick: () => void
}

export const IconEdit: FC<Props> = (props) => {
  return <BsPencil className={styles[props.variant]} onClick={props.onClick} />
}
