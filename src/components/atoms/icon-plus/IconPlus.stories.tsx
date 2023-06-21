import type { Meta, StoryObj } from '@storybook/react'

import { IconPlus } from './IconPlus'

const meta = {
  title: 'Atoms/IconPlus',
  component: IconPlus,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof IconPlus>

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
