import { type FC, useEffect, useState } from 'react'
import './global.scss'
import { invoke } from '@tauri-apps/api/tauri'

interface ProjectJson {
  projectId: string
  rdbms: string
  user: string
  password: string
  host: string
  port: string
  schema: string
}

export const App: FC = () => {
  const [a, setA] = useState(1)
  const [projects, setProjects] = useState<ProjectJson[]>([])

  useEffect(() => {
    invoke<ProjectJson[]>('all_projects_command')
      .then((data) => {
        console.log('fetched: ', data)
        setProjects(data)
      })
      .catch(console.log)
  }, [a])

  useEffect(() => {
    invoke('select_project_command', {
      projectId: '72c7097a-c04c-422d-956f-2da1bb3f5316',
    })
      .then((_) => {
        invoke('all_snapshot_summaries_command')
          .then(console.log)
          .catch(console.log)
      })
      .catch(console.log)
  })

  const insert: () => void = () => {
    invoke<string>('create_project_id_command')
      .then((data) => {
        const projectJson = {
          project_id: data,
          rdbms: 'mysql',
          user: 'John',
          password: 'pw',
          host: 'localhost',
          port: '3306',
          schema: 'test-db',
        }
        invoke('insert_project_command', { projectJson })
          .then((_) => {
            setA(a + 1)
          })
          .catch(console.log)
      })
      .catch(console.log)
  }

  if (projects.length === 0) {
    return (
      <>
        <p>empty</p>
      </>
    )
  } else {
    return (
      <>
        {projects.map((project) => (
          <div key={project.projectId}>
            <p>{project.projectId}</p>
            <p>{project.rdbms}</p>
            <p>{project.user}</p>
            <p>{project.password}</p>
            <p>{project.host}</p>
            <p>{project.port}</p>
            <p>{project.schema}</p>
          </div>
        ))}
        <button onClick={insert}>Create</button>
      </>
    )
  }
}
