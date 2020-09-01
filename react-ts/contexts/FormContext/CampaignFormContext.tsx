import React, { createContext, useState } from 'react'
import {
  FormContextProps,
  formContextPropsDefault,
  FormContextProviderProps,
  FoosValues,
  foosValuesDefault
} from './types'
import { CardStatus } from 'components/Card'
import useFilteredBar from 'hooks/useFilteredBar'

export const FormContext = createContext<FormContextProps>(formContextPropsDefault)

const FormContextProvider: React.FC<FormContextProviderProps> = ({ children }) => {
  const [foosValues, setFoosValues] = useState<FoosValues>(foosValuesDefault)
  const [foosCardStatus, setFoosCardStatus] = useState<CardStatus>(CardStatus.NONE)
  const { fetchBar, bar, barLoading } = useFilteredBar()

  return (
    <FormContext.Provider
      value={{
        foosValues,
        setFoosValues,
        foosCardStatus,
        setFoosCardStatus,
        fetchBar,
        bar,
        barLoading
      }}
    >
      {children}
    </FormContext.Provider>
  )
}

export default FormContextProvider
