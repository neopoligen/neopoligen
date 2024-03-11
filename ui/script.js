const { emit, listen } = window.__TAURI__.event
const { invoke } = window.__TAURI__.tauri

listen('neo_message', (event) => {
  const text_output = document.getElementById("text_output")
  if (event.payload.trim() === "CMD: CLEAR") {
    text_output.innerHTML = ""
  } else {
    text_output.innerHTML = text_output.innerHTML + event.payload
  }
})


document.addEventListener('DOMContentLoaded', () => {
  const el = document.querySelector('#launchButton')
  el.addEventListener('click', () => {
  //  const { invoke } = window.__TAURI__.tauri
    invoke('open_browser', {}).then((response) => {})
  })
   update_site_list()
})

function update_site_list() {
  const list_el = document.querySelector("#siteList")
  invoke('get_site_list', {}).then((response) => {
    console.log(response)
  })
}
