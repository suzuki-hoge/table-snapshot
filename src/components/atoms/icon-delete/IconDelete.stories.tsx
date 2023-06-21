import type { Meta, StoryObj } from '@storybook/react'

import { IconDelete } from './IconDelete'

const meta = {
  title: 'Atoms/IconDelete',
  component: IconDelete,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof IconDelete>

export default meta
type Story = StoryObj<typeof meta>

export const Small: Story = {
  args: {
    variant: 'small',
  },
}

export const Medium: Story = {
  args: {
    variant: 'medium',
  },
}

export const Large: Story = {
  args: {
    variant: 'large',
  },
}
