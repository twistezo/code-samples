const NotifierQueue: React.FC = () => {
  const { notifications, removeNotification } = useContext(AppContext)

  return (
    <div className='NotifierQueue'>
      {notifications.map((n: Notification) => (
        <Notifier
          key={n.id}
          id={n.id}
          type={n.type}
          text={n.text}
          time={n.time}
          onRemoveNotification={removeNotification}
        />
      ))}
    </div>
  )
}

export default NotifierQueue
