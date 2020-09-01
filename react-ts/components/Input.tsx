const Input: React.FC<InputProps> = ({
  id,
  className,
  title,
  help,
  placeholder,
  label,
  name,
  type = InputType.TEXT,
  value,
  onChange,
  onKeyDown,
  errors,
  maskType,
  required,
  disabled
}) => {
  const inputRef: React.RefObject<HTMLInputElement> = useRef(null)
  const [state, setState] = useState<InputState>(inputStateDefault)
  const [hasErrors, setHasErrors] = useState<boolean>(false)

  useEffect(() => {
    if (maskType !== undefined) {
      Inputmask(InputMaskUtils.options(maskType)).mask(inputRef.current as HTMLElement)
    }
  }, [maskType])

  useEffect(() => {
    setHasErrors((errors && errors.length > 0) || false)
  }, [errors])

  const handleKeyDown = (event: React.KeyboardEvent<HTMLDivElement>): void => {
    if (onKeyDown && event.key === onKeyDown.key) {
      event.preventDefault()
      event.stopPropagation()
      onKeyDown.callback()
    }
  }

  const handleChange = (e: Event): void => {
    const { value, id } = e.currentTarget
    onChange(value, id)
  }

  const handleFocus = (): void => void setState({ ...state, focused: true, pureFocused: true })

  const handleBlur = (): void => void setState({ ...state, focused: false, pureFocused: false })

  // workaround for handle browser autofill event (https://github.com/facebook/react/issues/1159)
  const handleAutoFill = (e: Event): void => {
    e.persist()

    setState(prevState => ({
      ...prevState,

      focused: (e as any).animationName === 'onAutoFillStart'
    }))
  }

  const wrapperStyle: string =
    `Input` +
    TsxUtils.extraStyle(label, 'Input--extra-height') +
    TsxUtils.extraStyle(className, className)

  const labelStyle: string =
    `Input__label` +
    TsxUtils.extraStyle(label && (state.focused || value), 'Input__label--nested') +
    TsxUtils.extraStyle(state.focused && state.pureFocused, 'Input__label--nested-focused') +
    TsxUtils.extraStyle(hasErrors, 'Input__label--error') +
    TsxUtils.extraStyle(disabled, 'Input__label--disabled')

  const inputStyle: string =
    `Input__field` +
    TsxUtils.extraStyle(hasErrors, 'Input__field--error') +
    TsxUtils.extraStyle(disabled, 'Input__field--disabled')

  return (
    <div className='Input--container'>
      {title && <InputTitle title={title} help={help} />}

      <div className={wrapperStyle}>
        <label htmlFor={id} className={labelStyle}>
          {label}
        </label>

        <input
          id={id}
          className={inputStyle}
          placeholder={label ? undefined : placeholder}
          name={name}
          type={type}
          value={value}
          onChange={handleChange}
          onKeyDown={handleKeyDown}
          onBlur={handleBlur}
          onFocus={handleFocus}
          onAnimationStart={handleAutoFill}
          required={required}
          disabled={disabled}
          ref={inputRef}
        />

        {hasErrors && <InputError errors={[...(errors ?? [])]} />}
      </div>
    </div>
  )
}

export default Input
