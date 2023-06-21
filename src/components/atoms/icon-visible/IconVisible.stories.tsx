import type { Meta, StoryObj } from '@storybook/react'

import { IconVisible } from './IconVisible'

const meta = {
  title: 'Atoms/IconVisible',
  component: IconVisible,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof IconVisible>

export default meta
type Story = StoryObj<typeof meta>

export const Small: Story = {
  args: {
    variant: 'small',
    visible: true,
  },
}

export const Medium: Story = {
  args: {
    variant: 'medium',
    visible: true,
  },
}

export const Large: Story = {
  args: {
    variant: 'large',
    visible: true,
  },
}
