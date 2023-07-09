import type { Meta, StoryObj } from '@storybook/react'

import { ProjectCreate } from './ProjectCreate'

const meta = {
  title: 'Templates/ProjectCreate',
  component: ProjectCreate,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof ProjectCreate>

export default meta
type Story = StoryObj<typeof meta>

export const Component: Story = {
  args: {
    insert: console.log,
  },
}
