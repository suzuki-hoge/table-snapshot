import React, { type FC, Fragment } from 'react'
import styles from './DiffTable.module.scss'
import { type Status } from '../../../types/Tmp'

type Row = Record<string, { status: Status; value: string }>

interface Props {
  primaryValues: string[]
  primaryColName: string
  colNames: string[]
  rows1: Record<string, Row>
  rows2: Record<string, Row>
}

const colors = {
  stay: styles.stay,
  added: styles.added,
  deleted: styles.deleted,
  none: styles.none,
}

interface TRProps {
  primaryValue: string
  colNames: string[]
  row?: Row
  n: number
}

const TR: FC<TRProps> = (props) => {
  return (
    <tr>
      {props.n === 1 && <td rowSpan={2}>{props.primaryValue}</td>}
      {props.colNames.map((colName, i) =>
        props.row != null ? (
          colName in props.row ? (
            <td key={i} className={colors[props.row[colName].status]}>
              {props.row[colName].value}
            </td>
          ) : (
            <td key={i} className={colors.none}></td>
          )
        ) : (
          <td key={i} className={colors.none}></td>
        )
      )}
    </tr>
  )
}

export const DiffTable: FC<Props> = (props) => {
  return (
    <table>
      <thead>
        <tr>
          <th>{props.primaryColName}</th>
          {props.colNames.map((colName, i) => (
            <th key={i}>{colName}</th>
          ))}
        </tr>
      </thead>
      <tbody>
        {props.primaryValues.map((primaryValue, i) => (
          <Fragment key={i}>
            <TR
              key={`${i}-1`}
              primaryValue={primaryValue}
              colNames={props.colNames}
              row={props.rows1[primaryValue]}
              n={1}
            />
            <TR
              key={`${i}-2`}
              primaryValue={primaryValue}
              colNames={props.colNames}
              row={props.rows2[primaryValue]}
              n={2}
            />
          </Fragment>
        ))}
      </tbody>
    </table>
  )
}
