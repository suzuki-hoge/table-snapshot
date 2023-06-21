import type { Meta, StoryObj } from '@storybook/react'

import { TextCard } from './TextCard'

const meta = {
  title: 'Molecules/TextCard',
  component: TextCard,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof TextCard>

export default meta
type Story = StoryObj<typeof meta>

export const First: Story = {
  args: {
    label: '初期状態',
    text: '2023/01/01 12:34:56',
    disabled: false,
  },
}

export const Second: Story = {
  args: {
    label: 'サインアップ ( Google アカウント連携 )',
    text: '2023/01/01 12:34:56',
    disabled: true,
  },
}

export const Third: Story = {
  args: {
    label: '退会予約',
    text: '2023/01/01 12:34:56',
    disabled: false,
  },
}

export const Fourth: Story = {
  args: {
    label: '退会確定',
    text: '2023/01/01 12:34:56',
    disabled: false,
  },
}
