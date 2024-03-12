const { emit, listen } = window.__TAURI__.event
const { invoke } = window.__TAURI__.tauri

function add_listener_to_selector(listener, selector, func) {
    const el = document.querySelector(selector)
    el.addEventListener(listener, func)
}

function update_home_page() {

  log("Welcome to the beta version of the Neopoligen website builder")
  invoke('get_state', {}).then((state_string) => {
    const state = JSON.parse(state_string)

    add_listener_to_selector("click", "#launchBrowserButton", (event) => {
      invoke('open_browser', {}).then((response) => {})
    })


    // const browser_button_el = document.querySelector('#launchBrowserButton')
    // browser_button_el.innerHTML = `Open ${state.active_site} in browser`
    // browesr_button_el.addEventListener('click', () => {
    //   invoke('open_browser', {}).then((response) => {})
    // })

    // const finder_button_el = document.querySelector('#launchFinderButton')
    // finder_button_el.innerHTML = `Open ${state.active_site} in Finder`
    // finder_button_el.addEventListener('click', () => {
    //   invoke('open_finder', { "site": state.active_site }).then((response) => {})
    // })


    console.log(state)
    log(``)
    log(`Current site: ${state.active_site}`)
    log(``)
    log(`- Click the 'Launch Browser' button to preview your site`)
    log(``)
    log(`- Click 'Sites' to change to a different site or make a new one`)
    log(``)
    log(`- Click 'About' to change learn more about Neopoligen`)
  })

}

function log(msg) {
    text_output.innerHTML = text_output.innerHTML + msg + "\n"
}

// deprecated: TODO: remove
function get_status(data) {
  invoke('get_status', {}).then((response) => {
      data = JSON.parse(response)
    console.log(data)
  }).await
}

function connect_launch_browbutton() {
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
