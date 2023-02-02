async function init() {
  let rustApp = null

  try {
    rustApp = await import('../pkg')
  } catch (error) {
    console.error(error)
    return error
  }

  console.log(rustApp.grayscale)

  const input = document.getElementById('upload')
  const fileReader = new FileReader()

  fileReader.onloadend = () => {
    let base64 = fileReader.result.replace(
      /^data:image\/(png|jpeg|jpg);base64./, ''
    )
    const imgUrl = rustApp.grayscale(base64)
    document.getElementById('new-img').setAttribute('src', imgUrl)
  }

  input.addEventListener('change', () => {
    fileReader.readAsDataURL(input.files[0])
  })
}

init()
