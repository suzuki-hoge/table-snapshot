import type { Meta, StoryObj } from '@storybook/react'

import { ModalWindow } from './ModalWindow'
import { IconSave } from '../../atoms/icon-save/IconSave'

const meta = {
  title: 'Molecules/ModalWindow',
  component: ModalWindow,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof ModalWindow>

export default meta
type Story = StoryObj<typeof meta>

export const Component: Story = {
  args: {
    isOpen: true,
    setIsOpen: (isOpen: boolean) => {},
    button: <IconSave variant={'large'} onClick={() => {}} />,
    children: (
      <>
        {[...Array(20).keys()].map((n) => (
          <p key={n}>content {n}</p>
        ))}
      </>
    ),
  },
}
