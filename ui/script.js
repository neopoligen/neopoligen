const { emit, listen } = window.__TAURI__.event
const { invoke } = window.__TAURI__.tauri


function init_page() {
  add_link_listeners()
  add_button_listeners()
}


function add_listener(listener, selector, func) {
    const el = document.querySelector(selector)
    el.addEventListener(listener, func)
}

function add_link_listeners() {
  const els = document.querySelectorAll(".link") 
  els.forEach((el) => {
    el.addEventListener("click", open_link)
  })
}

function add_button_listeners() {
  const els = document.querySelectorAll("button") 
  els.forEach((el) => {
    console.log(el)
    el.addEventListener("click", handle_button_click)
  })
}

function delete_neopoligen_config() {
  console.log("delete_neopoligen_config")
  invoke('delete_neopoligen_config', {}).then((raw) => {})
}

function handle_button_click(event) {
  if (event.target.dataset.command) {
    window[event.target.dataset.command](event)
  }
}

function edit_in_vscode(event) {
  invoke('edit_in_vscode', {}).then((raw) => {
    const json = JSON.parse(raw)
    console.log(json)
    console.log(event.target.parentElement)
    if (json.status.type !== "ok") {
      set_html(
        "#edit_in_vscode_button_wrapper", 
        `Could not open Visual Studio Code. This usually means it's not
installed properly. Try installing it again then restart Neopoligen to
see if that fixes the issue`)
    }
  })
}

function open_browser(event) {
  invoke('open_browser', {}).then((raw) => {})
}

function open_finder(event) {
  invoke('open_finder', {}).then((raw) => {})
}

function open_link(event) {
  invoke('open_link', { url: event.target.dataset.href }).then((raw) => {})
}


function open_neo_folder() {
  invoke('open_neo_folder', {}).then((raw) => {})
}

function set_html(selector, html) {
    const el = document.querySelector(selector)
    el.innerHTML = html
}

function update_debug_page() {
  add_link_listeners()
}


function update_home_page() {

  invoke('get_state', {}).then((state_string) => {
    const state = JSON.parse(state_string)
    console.log(state)

  log("Welcome to the beta version of the Neopoligen")
  log(`Verison: ${state.app_version}`)
  log("")
  log("- Check out the 'Getting Started' link above if you're new to the app")
  log("")
  log("- Use the 'Preview' and 'Edit' buttons above to work with your site")

    // add_listener("click", "#vscode_button", (event) => {
    //   set_html("#vscode_msg", "Launching");
    //   invoke('edit_in_vscode', { site: state.active_site}).then((raw) => {
    //     const resp= JSON.parse(raw)
    //     console.log(resp)
    //     if (resp.status.type === "ok") {
    //       set_html("#vscode_msg", "");
    //     } else {
    //       set_html("#vscode_li", `Error: Could not launch Visual Studio Code<br/>This usually means it's not installed.<br/>
    //         You can get it from here: <a id="vscode_link">Visual Studio Code</a><br/>
    //         (You'll need to restart Neopoligen once you've installed it)`)
    //       add_listener("click", "#vscode_link", () => {
    //         invoke('open_link', { url: 'https://code.visualstudio.com/'}).then((raw) => {})
    //       })
    //     }
    //   })
    // })

    // set_html("#current_site", `Current Site: ${state.active_site}`)

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
    log(`- Use the 'Sites' link to change to a different site or make a new one`)
    log(``)
    log(`- Click 'About' to change learn more about Neopoligen`)
    log(``)
    log(`- Debugging status messages will display here as you make changes to your site`)
    log(``)
    log(`- Note that Neopoligen doesn't have automatic updates yet. `)

  })

}

function log(msg) {
  const output_el = document.querySelector("#text_output")
  if (output_el) {
    text_output.innerHTML = output_el.innerHTML + msg + "\n"
  } else {
    console.log(msg)
  }
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
  const output_el = document.querySelector("#text_output")
  if (output_el) {
    if (event.payload.trim() === "CMD: CLEAR") {
      output_el.innerHTML = ""
    } else {
      output_el.innerHTML = output_el.innerHTML + event.payload + "\n"
    }
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
