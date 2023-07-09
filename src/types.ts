export interface Project {
  id: string
  name: string
  rdbms: string
  user: string
  password: string
  host: string
  port: string
  schema: string
}

export interface SnapshotSummary {
  id: string
  name: string
  createAt: string
}

type PrimaryValue = string
type ColName = string
type RowDiff = Record<
  PrimaryValue,
  Record<
    ColName,
    { status: 'stay' | 'added' | 'deleted' | 'none'; value: string }
  >
>

export interface TableDiff {
  tableName: string
  primaryValues: PrimaryValue[]
  primaryColName: ColName
  colNames: ColName[]
  rowDiffs1: RowDiff
  rowDiffs2: RowDiff
}
