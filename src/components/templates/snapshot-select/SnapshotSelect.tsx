import { type FC, useState } from 'react'
import styles from './SnapshotSelect.module.scss'
import { TextCard } from '../../molecules/text-card/TextCard'
import { type Snapshot } from '../../../types/Tmp'
import { Header } from '../../molecules/header/Header'
import { IconBack } from '../../atoms/icon-back/IconBack'
import { ModalWindow } from '../../molecules/ModalWindow/ModalWindow'
import { SnapshotInput } from '../../organisms/snapshot-input/SnapshotInput'
import { IconPlus } from '../../atoms/icon-plus/IconPlus'
import { IconGear } from '../../atoms/icon-gear/IconGear'
import { IconEdit } from '../../atoms/icon-edit/IconEdit'
import { IconDelete } from '../../atoms/icon-delete/IconDelete'
import { IconSave } from '../../atoms/icon-save/IconSave'

interface Props {
  snapshots: Snapshot[]
}

export const SnapshotSelect: FC<Props> = (props) => {
  const [isSetting, setIsSetting] = useState(false)
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [snapshot, setSnapshot] = useState<Snapshot | undefined>(undefined)
  const [selectedId, setSelectedId] = useState<string | null>(null)

  return (
    <div className={styles.template}>
      <Header
        globals={<IconBack variant={'large'} onClick={() => {}} />}
        locals={
          <>
            <IconPlus
              variant={'large'}
              onClick={() => {
                setSnapshot(undefined)
                setIsModalOpen(true)
              }}
            />
            <IconGear
              variant={'large'}
              onClick={() => {
                setIsSetting(!isSetting)
              }}
            />
          </>
        }
      />
      <div className={styles.component}>
        <div className={styles.snapshots}>
          {props.snapshots.map((snapshot) => (
            <div key={snapshot.id} className={styles.item}>
              <TextCard
                key={snapshot.id}
                label={snapshot.title}
                text={snapshot.created}
                selected={selectedId === snapshot.id}
                onClick={() => {
                  if (selectedId === null) {
                    setSelectedId(snapshot.id)
                  } else if (selectedId === snapshot.id) {
                    setSelectedId(null)
                  } else {
                    alert(2)
                  }
                }}
              />
              {isSetting && (
                <div className={styles.icons}>
                  <IconEdit
                    variant={'medium'}
                    onClick={() => {
                      setSnapshot(snapshot)
                      setIsModalOpen(true)
                    }}
                  />
                  <IconDelete variant={'medium'} onClick={() => {}} />
                </div>
              )}
            </div>
          ))}
        </div>
      </div>
      <ModalWindow
        isOpen={isModalOpen}
        setIsOpen={setIsModalOpen}
        button={
          <IconSave
            variant={'large'}
            onClick={() => {
              console.log(42)
            }}
          />
        }
      >
        <SnapshotInput snapshot={snapshot} />
      </ModalWindow>
    </div>
  )
}
