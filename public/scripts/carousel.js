const img = document.querySelectorAll("#carousel>div>img")
const main_img = document.querySelector("#carousel>img")

let id

function setFocus(index) {
  clearTimeout(id)
  document.getElementById("focus").id = ""

  img[index].id = "focus"
  main_img.attributes.src.value = img[index].attributes.src.value
  main_img.attributes.alt.value = img[index].attributes.alt.value


  index++

  if (index >= img.length) { index = 0 }

  id = setTimeout(() => setFocus(index), 6000)
}

function addImgListeners() {
  for (i = 0; i < img.length; i++) {
    const index = i
    img[i].addEventListener("click", () => setFocus(index))
  }
}

setFocus(0)
addImgListeners()
