const { emit, listen } = window.__TAURI__.event
const { invoke } = window.__TAURI__.tauri

function log(msg) {
    text_output.innerHTML = text_output.innerHTML + msg + "\n"
}

async function get_status() {
  invoke('get_status', {}).then((response) => JSON.parse(response))
}

function connect_launch_button() {
  const el = document.querySelector('#launchButton')
  el.addEventListener('click', () => {
    invoke('open_browser', {}).then((response) => {})
  })
}

listen('neo_message', (event) => {
  const text_output = document.getElementById("text_output")
  if (event.payload.trim() === "CMD: CLEAR") {
    text_output.innerHTML = ""
  } else {
    text_output.innerHTML = text_output.innerHTML + event.payload + "\n"
  }
})



function update_site_list() {
  const list_el = document.querySelector("#siteList")
  invoke('get_site_list', {}).then((response) => {
    let data = JSON.parse(response)
    data.sites.forEach((site) => {
      let site_button = document.createElement("button")
      site_button.innerHTML = site.key
      site_button.dataset.site_key = site.key
      site_button.addEventListener("click", set_active_site)
      let site_li = document.createElement("li")
      site_li.appendChild(site_button)
      list_el.appendChild(site_li)
    })
  })
}

function set_active_site(event) {
  invoke('set_active_site', {site_key: event.target.dataset.site_key} ).then((response) => {
    console.log("set_actitve_site")
  })
}
