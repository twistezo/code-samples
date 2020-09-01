import { useState } from 'react'
import { v4 as uuidv4 } from 'uuid'
import { NotifierType, Notification } from 'components/Notifier'
import { UseNotifierProps } from './types'

const useNotifier = (): UseNotifierProps => {
  const [notifications, setNotifications] = useState<Notification[]>([])

  const addNotification = (type: NotifierType, text: string | string[], time?: number): void =>
    void setNotifications(prevState => [...prevState, { id: uuidv4(), type, text, time }])

  const removeNotification = (id: string): void =>
    void setNotifications(prevState => [...prevState.filter((n: Notification) => n.id !== id)])

  return { notifications, addNotification, removeNotification }
}

export default useNotifier
