export interface FormContextProviderProps {
  children: ReactChild
}

export interface FormContextProps {
  foosValues: FoosValues
  setFoosValues: React.Dispatch<React.SetStateAction<FoosValues>>
  foosCardStatus: CardStatus
  setFoosCardStatus: React.Dispatch<React.SetStateAction<CardStatus>>
  fetchBar: useFilteredBarReturn['fetchBar']
  bar: useFilteredBarReturn['bar']
  barLoading: useFilteredBarReturn['barLoading']
}

type FoosValuesYoop = Pick<Yoop, 'name' | 'startDate' | 'endDate' | 'status' | 'bar'>

export interface FoosValues extends FoosValuesYoop {
  bazId: Baz['id']
  bazName: Baz['name']
  barName: Bar['name']
  yoopId: Yoop['id']
}

export const foosValuesDefault: FoosValues = {
  bazId: '',
  bazName: '',
  barName: '',
  yoopId: '',
  name: '',
  startDate: '',
  endDate: '',
  status: YoopStatus.SKETCH,
  bar: []
}

export const formContextPropsDefault: FormContextProps = {
  foosValues: foosValuesDefault,
  setFoosValues: () => void {},
  foosCardStatus: CardStatus.NONE,
  setFoosCardStatus: () => void {},
  fetchBar: () => void {},
  bar: [],
  barLoading: false
}
