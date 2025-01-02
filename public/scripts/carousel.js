const img = document.querySelectorAll("#carousel>div>img")
const main_img = document.querySelector("#carousel>img")
const img_list = document.querySelector("#carousel>div")

let id
const SCROLL_MARGIN = 50

function setFocus(index) {
  clearTimeout(id)
  document.getElementById("focus").id = ""

  img[index].id = "focus"
  // let scrolloffset = img[index].offsetLeft - img_list.scrollWidth / 3
  // console.log(scrolloffset)
  // img_list.scrollLeft = scrolloffset
  const childOffsetLeft2 = img[index].offsetLeft + img[index].offsetWidth;
  const containerScrollLeft2 = img_list.scrollLeft + img_list.offsetWidth;

  // is child behind (left)
  if (img_list.scrollLeft > img[index].offsetLeft) {
    img_list.scrollLeft = img[index].offsetLeft - SCROLL_MARGIN;
  }
  // is child ahead (right)
  if (containerScrollLeft2 < childOffsetLeft2) {
    img_list.scrollLeft += childOffsetLeft2 - containerScrollLeft2 + SCROLL_MARGIN;
  }

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
