import { type Color } from './Color'

export type Status = 'stay' | 'added' | 'deleted' | 'none'

export interface Project {
  id: string
  name: string // todo validation
  color: Color
  user: string
  password: string
  host: string
  port: string
  schema: string
  rdbms: Rdbms
}

export type Rdbms = 'MySQL'

export interface Snapshot {
  id: string
  title: string
  created: string
}

export type Row = Record<string, { status: Status; value: string }>

export interface Diff {
  tableName: string
  primaryValues: string[]
  primaryColName: string
  colNames: string[]
  rows1: Record<string, Row>
  rows2: Record<string, Row>
}
