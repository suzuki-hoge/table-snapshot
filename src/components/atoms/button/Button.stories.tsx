import type { Meta, StoryObj } from '@storybook/react'

import { Button } from './Button'

const meta = {
  title: 'Atoms/Button',
  component: Button,
  tags: ['autodocs'],
  argTypes: {
    variant: { control: 'select', options: ['primary', 'secondary'] },
  },
} satisfies Meta<typeof Button>

export default meta
type Story = StoryObj<typeof meta>

export const Primary: Story = {
  args: {
    variant: 'primary',
    label: 'Select',
  },
}

export const Secondary: Story = {
  args: {
    variant: 'secondary',
    label: 'Cancel',
  },
}

export const Warn: Story = {
  args: {
    variant: 'warn',
    label: 'Delete',
  },
}
