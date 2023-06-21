import type { Meta, StoryObj } from '@storybook/react'

import { IconBack } from './IconBack'

const meta = {
  title: 'Atoms/IconBack',
  component: IconBack,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof IconBack>

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
