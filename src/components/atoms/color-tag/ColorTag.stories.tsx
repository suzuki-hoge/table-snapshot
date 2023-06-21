import type { Meta, StoryObj } from '@storybook/react'

import { ColorTag } from './ColorTag'

const meta = {
  title: 'Atoms/ColorTag',
  component: ColorTag,
  tags: ['autodocs'],
  argTypes: {
    variant: { control: 'select', options: ['red', 'green'] },
  },
} satisfies Meta<typeof ColorTag>

export default meta
type Story = StoryObj<typeof meta>

export const Red: Story = {
  args: {
    variant: 'red',
  },
}

export const Yellow: Story = {
  args: {
    variant: 'yellow',
  },
}

export const Green: Story = {
  args: {
    variant: 'green',
  },
}

export const Blue: Story = {
  args: {
    variant: 'blue',
  },
}

export const Purple: Story = {
  args: {
    variant: 'purple',
  },
}
