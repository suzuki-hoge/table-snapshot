import type { Meta, StoryObj } from '@storybook/react'

import { DiffTable } from './DiffTable'

const meta = {
  title: 'Molecules/DiffTable',
  component: DiffTable,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof DiffTable>

export default meta
type Story = StoryObj<typeof meta>

export const RowDeleted: Story = {
  args: {
    primaryValues: ['1'],
    primaryColName: 'id',
    colNames: ['name', 'age'],
    rows1: {
      '1': {
        name: { status: 'deleted', value: '"John"' },
        age: { status: 'deleted', value: '29' },
      },
    },
    rows2: {},
  },
}

export const RowAdded: Story = {
  args: {
    primaryValues: ['1'],
    primaryColName: 'id',
    colNames: ['name', 'age'],
    rows1: {},
    rows2: {
      '1': {
        name: { status: 'added', value: '"John"' },
        age: { status: 'added', value: '29' },
      },
    },
  },
}

export const RowModified: Story = {
  args: {
    primaryValues: ['1'],
    primaryColName: 'id',
    colNames: ['name', 'age'],
    rows1: {
      '1': {
        name: { status: 'deleted', value: '"John"' },
        age: { status: 'deleted', value: '29' },
      },
    },
    rows2: {
      '1': {
        name: { status: 'added', value: '"Jane"' },
        age: { status: 'added', value: '15' },
      },
    },
  },
}

export const RowsDeleted: Story = {
  args: {
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
    rows2: {},
  },
}

export const RowsAdded: Story = {
  args: {
    primaryValues: ['1', '2'],
    primaryColName: 'id',
    colNames: ['name', 'age'],
    rows1: {},
    rows2: {
      '1': {
        name: { status: 'added', value: '"John"' },
        age: { status: 'added', value: '29' },
      },
      '2': {
        name: { status: 'added', value: '"Alice"' },
        age: { status: 'added', value: '31' },
      },
    },
  },
}

export const RowsModified: Story = {
  args: {
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
      '2': {
        name: { status: 'added', value: '"Bob"' },
        age: { status: 'added', value: '42' },
      },
    },
  },
}

export const RowsModifiedAndDeleted: Story = {
  args: {
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
  },
}

export const RowsModifiedAndAdded: Story = {
  args: {
    primaryValues: ['1', '2'],
    primaryColName: 'id',
    colNames: ['name', 'age'],
    rows1: {
      '1': {
        name: { status: 'deleted', value: '"John"' },
        age: { status: 'deleted', value: '29' },
      },
    },
    rows2: {
      '1': {
        name: { status: 'added', value: '"Jane"' },
        age: { status: 'added', value: '15' },
      },
      '2': {
        name: { status: 'added', value: '"Alice"' },
        age: { status: 'added', value: '31' },
      },
    },
  },
}

export const ColModified: Story = {
  args: {
    primaryValues: ['1'],
    primaryColName: 'id',
    colNames: ['name', 'age'],
    rows1: {
      '1': {
        name: { status: 'stay', value: '"John"' },
        age: { status: 'deleted', value: '29' },
      },
    },
    rows2: {
      '1': {
        name: { status: 'stay', value: '"John"' },
        age: { status: 'added', value: '15' },
      },
    },
  },
}

export const ColRemoved: Story = {
  args: {
    primaryValues: ['1'],
    primaryColName: 'id',
    colNames: ['name', 'age'],
    rows1: {
      '1': {
        name: { status: 'stay', value: '"John"' },
        age: { status: 'deleted', value: '29' },
      },
    },
    rows2: {
      '1': {
        name: { status: 'stay', value: '"John"' },
      },
    },
  },
}

export const ColCreated: Story = {
  args: {
    primaryValues: ['1'],
    primaryColName: 'id',
    colNames: ['name', 'age'],
    rows1: {
      '1': {
        name: { status: 'stay', value: '"John"' },
      },
    },
    rows2: {
      '1': {
        name: { status: 'stay', value: '"John"' },
        age: { status: 'added', value: '29' },
      },
    },
  },
}

export const ColMismatched: Story = {
  args: {
    primaryValues: ['1'],
    primaryColName: 'id',
    colNames: ['name', 'age', 'rate'],
    rows1: {
      '1': {
        name: { status: 'stay', value: '"John"' },
        age: { status: 'deleted', value: '29' },
      },
    },
    rows2: {
      '1': {
        name: { status: 'stay', value: '"John"' },
        rate: { status: 'added', value: '1' },
      },
    },
  },
}
