const prev_button = document.getElementById("prev_month")
const next_button = document.getElementById("next_month")
const month_year_display = document.getElementById("month_year")
const days_display = document.querySelectorAll("#calendar>tbody>.data>td")
const bookings = Array.from(document.querySelectorAll("#bookings>li")).map((node) => node.innerText.split("|").map((string) => Date.parse(string)))

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
date.setUTCHours(0, 0, 0, 0)


function next_month() {
  date.setMonth(date.getMonth() + 1)

  update_display()
}

function prev_month() {
  date.setMonth(date.getMonth() - 1)

  update_display()
}

function update_display() {
  month_year_display.innerText = months[date.getMonth()] + " " + date.getFullYear()

  days_display.forEach((item) => item.innerText = "")

  let index = date.getDay() - 1

  if (index < 0) { index += 7 }

  const days_in_month = new Date(date.getFullYear(), date.getMonth() + 1, 0).getDate()

  for (day = 1; day <= days_in_month; day++) {
    const ms = new Date(Date.UTC(date.getFullYear(), date.getMonth(), day)).getTime()

    days_display[index].style.backgroundColor = "var(--free)"

    bookings.forEach((booking) => {
      if (booking[0] <= ms && ms <= booking[1]) {
        days_display[index].style.backgroundColor = "var(--accent)"
      }
    })

    days_display[index].innerText = day
    index++
  }

  days_display.forEach((item) => {
    if (item.innerText == "") {
      item.style.background = "none"
    }
  })
}

next_button.addEventListener("click", next_month)
prev_button.addEventListener("click", prev_month)

update_display()
