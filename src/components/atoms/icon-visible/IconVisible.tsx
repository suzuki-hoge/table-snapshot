import { type FC, useState } from 'react'
import styles from './IconVisible.module.scss'
import { BsEye, BsEyeSlash } from 'react-icons/bs'

interface Props {
  variant: 'small' | 'medium' | 'large'
  visible: boolean
  onClick: () => void
}

export const IconVisible: FC<Props> = (props) => {
  const [visible, setVisible] = useState(props.visible)

  return (
    <>
      {visible ? (
        <BsEye
          className={styles[props.variant]}
          onClick={() => {
            setVisible(!visible)
            props.onClick()
          }}
        />
      ) : (
        <BsEyeSlash
          className={styles[props.variant]}
          onClick={() => {
            setVisible(!visible)
            props.onClick()
          }}
        />
      )}
    </>
  )
}
