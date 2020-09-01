const Preview: React.FC<PreviewProps> = ({ className, src, thumbnailSrc, isVideo, alt }) => {
  const [loading, setLoading] = useState<boolean>(true)

  const onImageLoad = (): void => {
    setLoading(false)
  }

  const videoPreview = (): ReactNode => (
    <video src={src} poster={thumbnailSrc ?? src} onLoadedData={onImageLoad} loop muted autoPlay />
  )

  const imgPreview = (): ReactNode => (
    <img src={thumbnailSrc ?? src} alt={alt} onLoad={onImageLoad} />
  )

  return (
    <div className={className}>
      <div className='Preview__thumbnail'>
        {loading && (
          <div className='Preview__thumbnail--empty'>
            <FillingSpinner type={FillingSpinnerType.STANDARD} />
          </div>
        )}

        <a
          className={
            'Preview__thumbnail' + TsxUtils.extraStyle(loading, 'Preview__thumbnail--hidden')
          }
          rel='noopener noreferrer'
          href={src}
          target='_blank'
        >
          {isVideo ? videoPreview() : imgPreview()}
        </a>
      </div>
    </div>
  )
}

export default Preview
