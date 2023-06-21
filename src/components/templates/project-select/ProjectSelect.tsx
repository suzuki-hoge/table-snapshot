import { type FC, useState } from 'react'
import styles from './ProjectSelect.module.scss'
import { type Project } from '../../../types/Tmp'
import { ColorTagCard } from '../../molecules/color-tag-card/ColorTagCard'
import { Header } from '../../molecules/header/Header'
import { ModalWindow } from '../../molecules/ModalWindow/ModalWindow'
import { ProjectInput } from '../../organisms/project-input/ProjectInput'
import { IconPlus } from '../../atoms/icon-plus/IconPlus'
import { IconGear } from '../../atoms/icon-gear/IconGear'
import { IconSave } from '../../atoms/icon-save/IconSave'
import { IconEdit } from '../../atoms/icon-edit/IconEdit'
import { IconDelete } from '../../atoms/icon-delete/IconDelete'

interface Props {
  projects: Project[]
}

export const ProjectSelect: FC<Props> = (props) => {
  const [isSetting, setIsSetting] = useState(false)
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [project, setProject] = useState<Project | undefined>(undefined)

  return (
    <div className={styles.template}>
      <Header
        globals={<></>}
        locals={
          <>
            <IconPlus
              variant={'large'}
              onClick={() => {
                setProject(undefined)
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
        {props.projects.map((project, i) => (
          <div key={i} className={styles.item}>
            <ColorTagCard
              label={project.name}
              variant={project.color}
              onClick={() => {
                console.log(project.id)
              }}
            />
            {isSetting && (
              <div className={styles.icons}>
                <IconEdit
                  variant={'medium'}
                  onClick={() => {
                    setProject(project)
                    setIsModalOpen(true)
                  }}
                />
                <IconDelete variant={'medium'} onClick={() => {}} />
              </div>
            )}
          </div>
        ))}
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
        <ProjectInput project={project} />
      </ModalWindow>
    </div>
  )
}
