import type { Meta, StoryObj } from '@storybook/react'

import { SnapshotInput } from './SnapshotInput'

const meta = {
  title: 'Organisms/SnapshotInput',
  component: SnapshotInput,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof SnapshotInput>

export default meta
type Story = StoryObj<typeof meta>

export const Component: Story = {
  args: {},
}
