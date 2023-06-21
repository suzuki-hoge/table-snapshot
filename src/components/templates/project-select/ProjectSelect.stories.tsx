import type { Meta, StoryObj } from '@storybook/react'

import { ProjectSelect } from './ProjectSelect'

const meta = {
  title: 'Templates/ProjectSelect',
  component: ProjectSelect,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof ProjectSelect>

export default meta
type Story = StoryObj<typeof meta>
export const Component: Story = {
  args: {
    projects: [
      {
        id: '92B07638-8EBA-471D-BDC1-71685B21EFE4',
        name: 'My Laravel Project',
        color: 'yellow',
        user: 'admin',
        password: 'admin-pw',
        host: 'localhost',
        port: '3306',
        schema: 'my-laravel-project',
        rdbms: 'MySQL',
      },
      {
        id: '5594251B-5F1B-4706-9521-324BDF343B33',
        name: 'Todo App',
        color: 'green',
        user: 'admin',
        password: 'admin-pw',
        host: 'localhost',
        port: '3306',
        schema: 'todo',
        rdbms: 'MySQL',
      },
      {
        id: 'EBFE34CE-AB67-4B01-AC6A-F0487F3115B8',
        name: '副業のやつ ( RoR )',
        color: 'red',
        user: 'admin',
        password: 'admin-pw',
        host: 'localhost',
        port: '3306',
        schema: 'data',
        rdbms: 'MySQL',
      },
    ],
  },
}
