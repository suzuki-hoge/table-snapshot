import type { Meta, StoryObj } from '@storybook/react'

import { DiffViewer } from './DiffViewer'
import { type Diff } from '../../../types/Tmp'

const meta = {
  title: 'Templates/DiffViewer',
  component: DiffViewer,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof DiffViewer>

export default meta
type Story = StoryObj<typeof meta>

const diff: Omit<Diff, 'tableName'> = {
  primaryValues: ['1', '2'],
  primaryColName: 'id',
  colNames: ['name', 'age'],
  rows1: {
    '1': {
      name: { status: 'deleted', value: '"John"' },
      age: { status: 'deleted', value: '29' },
    },
    '2': {
      name: { status: 'deleted', value: '"Alice"' },
      age: { status: 'deleted', value: '31' },
    },
  },
  rows2: {
    '1': {
      name: { status: 'added', value: '"Jane"' },
      age: { status: 'added', value: '15' },
    },
  },
}

const tableNames = [
  'actions',
  'administrators',
  'bills',
  'campaigns',
  'cards',
  'emails',
  'events',
  'friends',
  'groups',
  'items',
  'letters',
  'locks',
  'login_histories',
  'mails',
  'messages',
  'options',
  'payments',
  'plans',
  'profiles',
  'receipts',
  'roles',
  'shipments',
  'users',
]

export const Component: Story = {
  args: {
    diffs: tableNames.map((tableName) => ({ tableName, ...diff })),
    ignoreTableNames: [],
  },
}
