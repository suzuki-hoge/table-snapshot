import type { Meta, StoryObj } from '@storybook/react'

import { IconSave } from './IconSave'

const meta = {
  title: 'Atoms/IconSave',
  component: IconSave,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof IconSave>

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
