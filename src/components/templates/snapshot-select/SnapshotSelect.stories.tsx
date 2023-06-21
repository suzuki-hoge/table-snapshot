import type { Meta, StoryObj } from '@storybook/react'

import { SnapshotSelect } from './SnapshotSelect'

const meta = {
  title: 'Templates/SnapshotSelect',
  component: SnapshotSelect,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof SnapshotSelect>

export default meta
type Story = StoryObj<typeof meta>
export const Component: Story = {
  args: {
    snapshots: [
      {
        id: 'CC181CDB-07BC-4747-AABC-653CEF526D77',
        title: '初期状態',
        created: '2023/01/01 12:34:56',
      },
      {
        id: 'A650B18F-ADC6-462D-A619-D13F2D01CDD7',
        title: 'サインアップ ( Google アカウント連携 )',
        created: '2023/02/02 12:34:56',
      },
      {
        id: '77428FAF-A06E-4273-BF21-DE576CC35F43',
        title: '退会予約',
        created: '2023/03/03 12:34:56',
      },
      {
        id: '515D47B9-0744-4519-9C54-8E67F79687D3',
        title: '退会確定',
        created: '2023/04/04 12:34:56',
      },
    ],
  },
}
