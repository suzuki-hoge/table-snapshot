import type { Meta, StoryObj } from '@storybook/react'

import { ProjectUpdate } from './ProjectUpdate'

const meta = {
  title: 'Templates/ProjectUpdate',
  component: ProjectUpdate,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof ProjectUpdate>

export default meta
type Story = StoryObj<typeof meta>

export const Component: Story = {
  args: {
    project: {
      id: '92B07638-8EBA-471D-BDC1-71685B21EFE4',
      name: 'My Laravel Project',
      rdbms: 'MySQL',
      user: 'admin',
      password: 'admin-pw',
      host: 'localhost',
      port: '3306',
      schema: 'my-laravel-project',
    },
    update: console.log,
  },
}
