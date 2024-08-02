const prev_button = document.getElementById("prev_month")
const next_button = document.getElementById("next_month")
const month_year_display = document.getElementById("month_year")
const days_display = document.querySelectorAll("#calendar>tbody>.data>td")

const months = [
  "Januari",
  "Februari",
  "Maart",
  "April",
  "Mei",
  "Juni",
  "Juli",
  "Augustus",
  "September",
  "Oktober",
  "November",
  "December"
]

let date = new Date()
date.setDate(1)
date.setHours(0, 0, 0, 0)

function next_month() {
  date.setMonth(date.getMonth() + 1)
  console.log(date)

  update_display()
}

function prev_month() {
  date.setMonth(date.getMonth() - 1)
  console.log(date)

  update_display()
}

function update_display() {
  month_year_display.innerText = months[date.getMonth()] + " " + date.getFullYear()

  days_display.forEach((item) => item.innerText = "")

  let index = date.getDay() - 1

  if (index < 0) { index += 7 }

  const days_in_month = new Date(date.getFullYear(), date.getMonth() + 1, 0).getDate()

  for (day = 1; day <= days_in_month; day++) {
    days_display[index].innerText = day
    index++
  }

  days_display.forEach((item) => {
    if (item.innerText == "") {
      item.style.background = "none"
    } else {
      item.style.backgroundColor = "var(--free)"
    }
  })
}

next_button.addEventListener("click", next_month)
prev_button.addEventListener("click", prev_month)

update_display()
