import type { Meta, StoryObj } from '@storybook/react'

import { ColorTagCard } from './ColorTagCard'
import { colors } from '../../../types/Color'

const meta = {
  title: 'Molecules/ColorTagCard',
  component: ColorTagCard,
  tags: ['autodocs'],
  argTypes: {
    variant: { control: 'select', options: colors },
  },
} satisfies Meta<typeof ColorTagCard>

export default meta
type Story = StoryObj<typeof meta>

export const Red: Story = {
  args: {
    label: 'My Laravel Project',
    variant: 'yellow',
  },
}

export const Green: Story = {
  args: {
    label: 'Todo App',
    variant: 'green',
  },
}

export const Blue: Story = {
  args: {
    label: '副業のやつ ( RoR )',
    variant: 'red',
  },
}
