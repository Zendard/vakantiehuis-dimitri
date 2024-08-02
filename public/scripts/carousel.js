const img = document.querySelectorAll("#carousel>img")

const IMG_SPACING = 3
const IMG_WIDTH = 10.25

function placeImgs() {
  let index = 0

  const starting_position = (-(IMG_WIDTH / 2) - ((IMG_SPACING + IMG_WIDTH) * ((img.length - 1) / 2)))

  img.forEach((img) => {

    const style_string = `${starting_position + (IMG_WIDTH + (IMG_SPACING)) * index}rem`

    console.log(style_string)

    img.style.left = style_string

    console.log(img)

    index++
  })
}

let id

function setFocus(index) {
  clearTimeout(id)
  document.getElementById("focus").id = ""

  img[index].id = "focus"

  if (index >= 4) { index = 0 }

  id = setTimeout(() => setFocus(index + 1), 6000)
}

function addImgListeners() {
  for (i = 0; i < img.length; i++) {
    const index = i
    img[i].addEventListener("click", () => setFocus(index))
  }
}

placeImgs()
setFocus(0)
addImgListeners()
