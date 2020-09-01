import { useRef, useEffect } from 'react'

export const usePrevious = <T>(value: T): void => {
  const ref = useRef()
  useEffect(() => {
    ref.current = value
  })
  return ref.current
}
