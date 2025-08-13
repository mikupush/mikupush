import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App'
import '@material/web/all'
import { styles as typescaleStyles } from '@material/web/typography/md-typescale-styles.js'
import '@/i18n'

document.adoptedStyleSheets.push(typescaleStyles.styleSheet!)

createRoot(document.getElementById('root')!).render(
	<StrictMode>
		<App />
	</StrictMode>,
)
