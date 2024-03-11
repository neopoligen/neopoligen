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
    invoke('open_browser', {}).then((response) => {})
  })
   update_site_list()
})

function update_site_list() {
  const list_el = document.querySelector("#siteList")
  invoke('get_site_list', {}).then((response) => {
    let data = JSON.parse(response)
    data.sites.forEach((site) => {
      let site_button = document.createElement("button")
      site_button.innerHTML = site.key
      let site_li = document.createElement("li")
      site_li.appendChild(site_button)
      list_el.appendChild(site_li)
    })
  })
}
