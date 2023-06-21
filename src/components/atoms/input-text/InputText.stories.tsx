import type { Meta, StoryObj } from '@storybook/react'

import { InputText } from './InputText'

const meta = {
  title: 'Atoms/InputText',
  component: InputText,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof InputText>

export default meta
type Story = StoryObj<typeof meta>

export const Small: Story = {
  args: {
    value: 'foo',
    length: 10,
  },
}

export const Medium: Story = {
  args: {
    length: 20,
  },
}

export const Large: Story = {
  args: {
    value: 'bar',
    length: 40,
  },
}
