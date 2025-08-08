import appIcon from '../../assets/app-icon.png'
import styles from './AppTitle.module.css'

export default function AppTitle() {
	return (
		<div className={styles.appTitle}>
			<img src={appIcon} />
			<h1>Miku Push!</h1>
		</div>
	)
}