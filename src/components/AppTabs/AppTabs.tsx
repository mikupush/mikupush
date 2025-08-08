import { useContext } from 'react'
import { UploadsContext } from '../../context/upload'
import styles from './AppTabs.module.css'

type Tab = 'upload' | 'uploads-in-progress' | 'finished-uploads'

interface AppTabsProps {
    onTabSelected?: (tab: Tab) => void
}

export default function AppTabs({ onTabSelected }: AppTabsProps) {
	const {
		inProgressUploadsCount,
		finishedUploadsCount,
		resetInProgressUploadsCount,
		resetFinishedUploadsCount,
	} = useContext(UploadsContext)

	const handleTabSelected = (index: Tab) => {
		if (typeof onTabSelected !== 'undefined') {
			onTabSelected(index)
		}
	}

	return (
		<md-tabs>
			<Tab
				text="Upload files"
				icon="upload"
				onClick={() => handleTabSelected('upload')}
			/>
			<Tab
				text="Uploads in progresss"
				icon="schedule"
				badge={inProgressUploadsCount}
				onClick={() => {
					handleTabSelected('uploads-in-progress')
					resetInProgressUploadsCount()
				}}
			/>
			<Tab
				text="Finished uploads"
				icon="check_circle"
				badge={finishedUploadsCount}
				onClick={() => {
					handleTabSelected('finished-uploads')
					resetFinishedUploadsCount()
				}}
			/>
		</md-tabs>
	)
}

interface TabsProps {
    text: string
    icon: string
    onClick: () => void
    badge?: number
}

function Tab(props: TabsProps) {
	const { text, icon, onClick } = props
	const badge = props.badge ?? 0

	return (
		<md-primary-tab onClick={onClick}>
			{badge > 0 ? (
				<span className={styles.badge}>{badge > 99 ? '99+' : badge}</span>
			) : (
				''
			)}
			<md-icon>{icon}</md-icon>
			{text}
		</md-primary-tab>
	)
}
