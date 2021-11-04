import { useEffect, useState } from 'react'
import Grid from './components/Grid'
import ABExample from './components/ABExample'
import './App.css'

function App() {
  const [data, setData] = useState([])

  useEffect(() => {
    (async () => {
      const res = await fetch('https://abc.desarol.com/sessions')
      const body = await res.json()
      setData(body.data)
    })()
  }, [])

  return (
    <div className="App">
      <header className="App-header">
        <div style={{ margin: '4rem auto', maxWidth: '960px', }}>
          <ABExample />
          <div style={{ marginBottom: '1rem' }}></div>
          <Grid
            columns={['Session', 'Experiment', 'Variant']}
            rows={data?.map(d => [d.id, d.experiment_id, d.variant])}
          />
        </div>
      </header>
    </div>
  )
}

export default App
