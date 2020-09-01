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
