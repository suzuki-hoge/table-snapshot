import type { Meta, StoryObj } from '@storybook/react'

import { IconClose } from './IconClose'

const meta = {
  title: 'Atoms/IconClose',
  component: IconClose,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof IconClose>

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
