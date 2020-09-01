const Button: React.FC<ButtonProps> = ({
  children,
  className,
  disabled = false,
  icon,
  onClick,
  size = ButtonSize.STANDARD,
  theme = ButtonTheme.BLUE,
  toUrl,
  toUrlExternal
}) => {
  const history = useHistory()

  return (
    <button
      className={
        'Button' +
        ` Button--${theme}` +
        ` Button--${size}` +
        TsxUtils.extraStyle(disabled, 'Button--disabled') +
        TsxUtils.extraStyle(className, className)
      }
      onClick={(event): void => {
        event?.preventDefault()

        onClick && onClick(event)

        if (toUrl) void history.push(toUrl)
        if (toUrlExternal) void window.open(toUrlExternal, '_blank')?.focus
      }}
      disabled={disabled}
    >
      {icon && (
        <Icon
          icon={icon}
          className={
            TsxUtils.extraStyle(children, 'Button__icon--margin') +
            TsxUtils.extraStyle(icon, 'Button__icon')
          }
        />
      )}
      {children}
    </button>
  )
}

export default Button
