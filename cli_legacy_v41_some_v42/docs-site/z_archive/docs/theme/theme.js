function add_selector_event_toggle_functions(selector, event_type, on_function, off_function) {
  console.log(`- Adding toggle functions to: ${selector}`)
  let elements = document.querySelectorAll(selector)
  elements.forEach((element) => {
    console.log(`- Updating element`)
    element.addEventListener(event_type, (event) => { 
        elements.forEach((update_element) => {
            update_element.classList.toggle("on")
            update_element.classList.toggle("off")
            const text_buffer = update_element.innerHTML
            update_element.innerHTML = update_element.dataset.toggle_text
            update_element.dataset.toggle_text = text_buffer
        })
      const element = event.target
      if (element.classList.contains("on")) {
        on_function.call(undefined, element)
      } else {
        off_function.call(undefined, element)
      }
    })
  })
}


function add_selector_listener_function(selector, event_type, trigger_function) {
  console.log(`- Adding '${event_type}' listener to: ${selector}`)
  let elements = document.querySelectorAll(selector)
  elements.forEach((element) => {
    element.addEventListener(event_type, trigger_function)
  })
}


function hide_original_source_content(event) {
  let display = document.getElementById(`original_content_display`)
  display.classList.add("hidden")
}


function show_original_source_content(event) {
  let display = document.getElementById(`original_content_display`)
  display.classList.remove("hidden")
}

function toggle_nav_menu_item(event) {
  console.log("hit toggle_nav_menu_item")
  let el = event.target
  let ds = el.dataset
  let li = document.getElementById(`${ds.menu}_${ds.id}`)
  if (ds.status == "closed") {
    li.classList.remove("title_folder_closed")
    li.classList.add("title_folder_opened")
    ds.status = "open"
  } else {
    li.classList.remove("title_folder_active")
    li.classList.remove("title_folder_opened")
    li.classList.add("title_folder_closed")
    ds.status = "closed"
  }
}

document.addEventListener("DOMContentLoaded", () => {
  add_selector_event_toggle_functions(
    ".original_content_toggle", 
    "click",
    show_original_source_content, 
    hide_original_source_content
  )
  add_selector_listener_function(
    ".nav_menu_button",
    "click",
    toggle_nav_menu_item
  )
})







// function add_style_to_selector(style, selector) {
//   let elements = document.querySelectorAll(selector)
//   elements.forEach((element) => {
//     element.classList.add(style)
//   })
// }

// function remove_style_from_selector(style, selector) {
//   let elements = document.querySelectorAll(selector)
//   elements.forEach((element) => {
//     element.classList.remove(style)
//   })
// }






// function toggle_style_via_selector(style, selector) {
//   let elements = document.querySelectorAll(selector)
//   elements.forEach((element) => {
//     element.classList.toggle(style)
//   })
// }

// function toggle_switch_via_selector(selector) {
//   let elements = document.querySelectorAll(selector)
//   elements.forEach((element) => {
//     element.classList.toggle("on")
//     element.classList.toggle("off")
//   })
// }

// function toggle_button(event) {
//   const element = event.target
//   element.classList.toggle("on")
//   element.classList.toggle("off")
//   const text_buffer = element.innerHTML
//   element.innerHTML = element.dataset.toggle_text
//   element.dataset.toggle_text = text_buffer
// }







//////////////////////////////////

// deprecated

// function add_nav_menu_button_listeners() {
//   const buttons = document.getElementsByClassName("nav_menu_button")
//   for (let button of buttons) {
//     button.addEventListener("click", toggle_nav_menu_item)
//   }
// }

// function add_original_content_toggle_listeners() {
//   const buttons = document.getElementsByClassName("original_content_toggle")
//   for (let button of buttons) {
//     button.addEventListener("click", toggle_original_content)
//   }
// }

// function toggle_original_content(event) {
//   let inner_el = document.getElementById(`inner_original_content_toggle`)
//   let inner_ds = inner_el.dataset
//   let outer_el = document.getElementById(`outer_original_content_toggle`)
//   let outer_ds = outer_el.dataset
//   let display = document.getElementById(`original_content_display`)
//   if (event.target.dataset.status == "hidden") {
//     display.classList.remove("hidden")
//     inner_ds.status = "visible"
//     outer_ds.status = "visible"
//     inner_el.innerHTML = "Hide original content file"
//     outer_el.innerHTML = "Hide original content file"
//   } else {
//     display.classList.add("hidden")
//     inner_ds.status = "hidden"
//     outer_ds.status = "hidden"
//     inner_el.innerHTML = "Show original content file"
//     outer_el.innerHTML = "Show original content file"
//   }
// }

