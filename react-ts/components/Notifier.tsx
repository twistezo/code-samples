const Notifier: React.FC<NotifierProps> = ({
  id,
  type,
  text,
  time,
  onRemoveNotification,
}) => {
  useEffect(() => {
    setTimeout(() => onRemoveNotification(id), time ? time : NOTIFIER_CLOSE_TIME)
  }, [id, onRemoveNotification, time])

  const getIcon = (type: NotifierType): IconProp => {
    switch (type) {
      case NotifierType.SUCCESS:
        return faCheckCircle
      case NotifierType.WARNING:
        return faInfoCircle
      case NotifierType.ERROR:
        return faExclamationCircle
    }
  }

  return (
    <div
      className={`Notifier Notifier--${type}`}
      onClick={(): void => void onRemoveNotification(id)}
    >
      <div className={`Notifier__icon Notifier__icon--${type}`}>
        <Icon icon={getIcon(type)} />
      </div>

      <div className='Notifier__text'>
        {Array.isArray(text)
          ? text.map(
              (t: string, i: number): ReactNode => (
                <div className='Notifier__text--row' key={i}>
                  {t}
                </div>
              )
            )
          : text}
      </div>
    </div>
  )
}

export default Notifier
