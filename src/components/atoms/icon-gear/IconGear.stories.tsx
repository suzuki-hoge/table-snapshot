import type { Meta, StoryObj } from '@storybook/react'

import { IconGear } from './IconGear'

const meta = {
  title: 'Atoms/IconGear',
  component: IconGear,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof IconGear>

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
