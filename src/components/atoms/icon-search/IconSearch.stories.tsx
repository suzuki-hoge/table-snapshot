import type { Meta, StoryObj } from '@storybook/react'

import { IconSearch } from './IconSearch'

const meta = {
  title: 'Atoms/IconSearch',
  component: IconSearch,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof IconSearch>

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
