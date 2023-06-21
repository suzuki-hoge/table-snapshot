import type { Meta, StoryObj } from '@storybook/react'

import { IconEdit } from './IconEdit'

const meta = {
  title: 'Atoms/IconEdit',
  component: IconEdit,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof IconEdit>

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
