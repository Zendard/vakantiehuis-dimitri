const add_booking_message = new URLSearchParams(document.location.search).get("add-booking")
const dialog = document.getElementById("message")
const dialog_text = dialog.querySelector("h1")
const bookings_array = Array.from(document.getElementById("bookings").children)

switch (add_booking_message) {
  case "success": {
    dialog_text.innerText = "Boeking toegevoegd!"
    dialog_text.style.color = "lime"
    dialog.showModal()
    break
  }

  case "wrongInput": {
    dialog_text.innerText = "Er ging iets mis, controleer of de incheckdatum voor de uitcheckdatum komt"
    dialog_text.style.color = "red"
    dialog.showModal()
    break
  }

  case "error": {
    dialog_text.innerText = "Er ging iets mis, probeer het opnieuw of contacteer Thijs"
    dialog_text.style.color = "red"
    dialog.showModal()
    break
  }

  case "alreadyBooked": {
    dialog_text.innerText = "Deze periode is al (deels) geboekt"
    dialog_text.style.color = "red"
    dialog.showModal()
    break
  }
}

bookings_array.forEach((booking) => {
  from_to = booking.innerText.split(" - ");

  booking.innerText = from_to[0].replaceAll("-", "/") + " - " + from_to[1].replaceAll("-", "/");

})

