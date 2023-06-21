import type { Meta, StoryObj } from '@storybook/react'

import { DiffContent } from './DiffContent'

const meta = {
  title: 'Organisms/DiffContent',
  component: DiffContent,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof DiffContent>

export default meta
type Story = StoryObj<typeof meta>

export const Component: Story = {
  args: {
    diff: {
      tableName: 'users',
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
  },
}
