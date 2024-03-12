const { emit, listen } = window.__TAURI__.event
const { invoke } = window.__TAURI__.tauri

function add_listener(listener, selector, func) {
    const el = document.querySelector(selector)
    el.addEventListener(listener, func)
}

function set_html(selector, html) {
    const el = document.querySelector(selector)
    el.innerHTML = html
}

function update_home_page() {

  invoke('get_state', {}).then((state_string) => {
    const state = JSON.parse(state_string)

  log("Welcome to the beta version of the Neopoligen")
  log(`Verison: ${state.app_version}`)
  log("")

  log("Neopoligen is a website builder. It uses files you write on")
  log("your computer to build a website you can see by clicking the")
  log("'Open in browser' button above. The site is only accessible")
  log("on your computer to start with. You can change and edit it")
  log("as much as you want. Once you've got it the way you want it")
  log("you can send it to a free service to make it available for")
  log("the world to see. Click 'Getting Started' for more details.")

  log("")
  log("Next:")
  log("")
  log("- Use the buttons above to edit and preview your site")

    add_listener("click", "#browser_button", (event) => {
      invoke('open_browser', {}).then((raw) => {})
    })

    add_listener("click", "#finder_button", (event) => {
      invoke('open_finder', { site: state.active_site}).then((raw) => {})
    })

    add_listener("click", "#vscode_button", (event) => {
      set_html("#vscode_msg", "Launching");
      invoke('edit_in_vscode', { site: state.active_site}).then((raw) => {
        const resp= JSON.parse(raw)
        console.log(resp)
        if (resp.status.type === "ok") {
          set_html("#vscode_msg", "");
        } else {
          set_html("#vscode_li", `Error: Could not launch Visual Studio Code<br/>This usually means it's not installed.<br/>
You can get it from here: <a id="vs_code_link">Visual Studio Code</a>
<br/>
(You'll need to restart Neopoligen once you've installed it)
`)
        }
      })
    })

    set_html("#current_site", `Current Site: ${state.active_site}`)

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
    log(`- Click 'Sites' to change to a different site or make a new one`)
    log(``)
    log(`- Click 'About' to change learn more about Neopoligen`)
    log(``)
    log(`- Debugging status messages will display here as you make changes to your site`)
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
