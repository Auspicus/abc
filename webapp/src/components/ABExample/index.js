import { useEffect, useState } from 'react'

const ABExample = () => {
  const colorFromVariant = variant => variant === 0 ? 'green' : 'blue'
  const nameFromVariant = variant => variant === 0 ? 'A' : 'B'

  const [variant, setVariant] = useState(null)

  useEffect(() => {
    (async () => {
      const res = await fetch('http://localhost:8080/experiments/experiment-1/session', { credentials: 'include' })
      const { variant: fetchedVariant } = await res.json()
      setVariant(fetchedVariant)
    })()
  }, [])

  return (
    <div style={{ color: colorFromVariant(variant) }}>
      Showing variant {nameFromVariant(variant)}
    </div>
  )
}

export default ABExample