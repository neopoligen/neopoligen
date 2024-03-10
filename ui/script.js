const { emit, listen } = window.__TAURI__.event

listen('neo_message', (event) => {
  const text_output = document.getElementById("text_output")
  if (event.payload.trim() === "CMD: CLEAR") {
    text_output.innerHTML = ""
  } else {
    text_output.innerHTML = text_output.innerHTML + event.payload
  }
})
