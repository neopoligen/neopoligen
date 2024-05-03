function add_listener_function_to_selector(listener, func, selector) {
  let elements = document.querySelectorAll(selector)
  elements.forEach((element) => {
    element.addEventListener(listener, func)
  })
}

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

function add_style_to_selector(style, selector) {
  let elements = document.querySelectorAll(selector)
  elements.forEach((element) => {
    element.classList.add(style)
  })
}

function remove_style_from_selector(style, selector) {
  let elements = document.querySelectorAll(selector)
  elements.forEach((element) => {
    element.classList.remove(style)
  })
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

function toggle_original_content(event) {
  let inner_el = document.getElementById(`inner_original_content_toggle`)
  let inner_ds = inner_el.dataset
  let outer_el = document.getElementById(`outer_original_content_toggle`)
  let outer_ds = outer_el.dataset
  let display = document.getElementById(`original_content_display`)
  if (event.target.dataset.status == "hidden") {
    display.classList.remove("hidden")
    inner_ds.status = "visible"
    outer_ds.status = "visible"
    inner_el.innerHTML = "Hide original content file"
    outer_el.innerHTML = "Hide original content file"
  } else {
    display.classList.add("hidden")
    inner_ds.status = "hidden"
    outer_ds.status = "hidden"
    inner_el.innerHTML = "Show original content file"
    outer_el.innerHTML = "Show original content file"
  }
}

function toggle_style_via_selector(style, selector) {
  let elements = document.querySelectorAll(selector)
  elements.forEach((element) => {
    element.classList.toggle(style)
  })
}

function toggle_switch_via_selector(selector) {
  let elements = document.querySelectorAll(selector)
  elements.forEach((element) => {
    element.classList.toggle("on")
    element.classList.toggle("off")
  })
}

function toggle_button(event) {
  const element = event.target
  element.classList.toggle("on")
  element.classList.toggle("off")
  const text_buffer = element.innerHTML
  element.innerHTML = element.dataset.toggle_text
  element.dataset.toggle_text = text_buffer
}


function add_toggle_function_to_button_selector(on_function, off_function, selector) {
  let elements = document.querySelectorAll(selector)
  elements.forEach((element) => {
    element.addEventListener("click", (event) => { 
      const element = event.target
      element.classList.toggle("on")
      element.classList.toggle("off")
      const text_buffer = element.innerHTML
      element.innerHTML = element.dataset.toggle_text
      element.dataset.toggle_text = text_buffer
      if (element.classList.contains("on")) {
        on_function.call(undefined, selector, element)
      } else {
        off_function.call(undefined, selector, element)
      }
    })
  })
}



function test_function_on(selector, element) {
  console.log(selector)
  console.log(element)
}

function test_function_off(selector, element) {
  console.log(selector)
  console.log(element)
}

document.addEventListener("DOMContentLoaded", () => {
  console.log("Content loaded")
  // add_listener_function_to_selector(
  //   "click", toggle_nav_menu_item, ".nav_menu_button"
  // )
  // add_listener_function_to_selector(
  //   "click", toggle_button, { 
  //     selector: ".original_content_toggle" 
  //   }
  // )

  add_toggle_function_to_button_selector(
    test_function_on, 
    test_function_off, 
    ".original_content_toggle"
  )

})

