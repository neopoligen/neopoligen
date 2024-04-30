////////////////////////////////////////////////////////////
// yagni-js

// TODO: Rename 'parent' to 'target'
// and internal 'target' to 'el'


function pageAddClassTo(target, className) {
  const el = pageGetEl(target)
  if (el) {
    el.classList.add(className)
    return el
  }
}

function pageAddListenerTo(target, event, func) {
  const el = pageGetEl(target)
  if (el) {
    el.addEventListener(event, func)
    return el
  }
}

function pageAddListenersTo(selector, event, func) {
  const els = document.querySelectorAll(selector)
  els.forEach((el) => {
    el.addEventListener(event, func)
  })
}

function pageAddSvgTo(target, tag, attrs = {}) {
  const el = pageGetEl(target)
  if (el) {
    const svg = document.createElementNS('http://www.w3.org/2000/svg', tag)
    pageUpdateSvgAttrs(svg, attrs)
    el.appendChild(svg)
    return svg
  }
}

function pageAddTo(target, tag, attrs = {}) {
  const el = pageGetEl(target)
  if (el) {
    const newEl = document.createElement(tag)
    pageUpdateAttrs(newEl, attrs)
    el.appendChild(newEl)
    return newEl
  }
}

function pageAddToFront(target, tag, attrs = {}) {
  const el = pageGetEl(target)
  if (el) {
    const newEl = document.createElement(tag)
    pageUpdateAttrs(newEl, attrs)
    if (el.hasChildNodes()) {
      const first_child = el.firstChild
      el.insertBefore(newEl, first_child)
    } else {
      el.appendChild(newEl)
    }
    return newEl
  }
}

function pageAppendText(target, text) {
  const el = pageGetEl(target)
  if (el) {
    el.innerText = el.innerText + text
  }
}

function pageGetData(target, key) {
  // TODO: get a data attribute value from an element
  // with JSON stringify
}

function pageGetEl(target) {
  if (typeof target === 'string') {
    const el = document.querySelector(target)
    if (el) {
      return el
    } else {
      pageLogError(`Could not find querySelector for: ${target}`)
      return undefined
    }
  } else if (target) {
    return target
  } else {
    pageLogError(`Could not get element: ${target}`)
    return undefined
  }
}

function pageGetEls(selector) {
  return document.querySelectorAll(selector)
}

function pageGetFloat(target) {
  const el = pageGetEl(target)
  if (el) {
    return parseFloat(el.value)
  } else {
    return undefined
  }
}

function pageGetInt(target) {
  const el = pageGetEl(target)
  if (el) {
    return parseInt(el.value, 10)
  } else {
    return undefined
  }
}

function pageGetStorage(key, defaultValue = undefined) {
  return localStorage.getItem(key)
    ? JSON.parse(localStorage.getItem(key)).payload
    : defaultValue
}

function pageGetValue(target) {
  const el = pageGetEl(target)
  if (el) {
    return el.value
  } else {
    return undefined
  }
}

function pageLogError(msg) {
  console.error(`${Date.now()} - ERROR: ${msg}`)
}

function pageLog(msg) {
  console.log(`${Date.now()} - INFO: ${msg}`)
}

function pageLogObject(msg) {
  console.log(msg)
}

function pageRemoveClassFrom(target, className) {
  const el = pageGetEl(target)
  if (el) {
    el.classList.remove(className)
    return el
  }
}

function pageSetData(target, key, value) {
  // TODO: set data attributes for an element
  // with JSON stringify
}

function pageSetHTML(target, value) {
  pageUpdateAttrs(target, {
    innerHTML: value
  })
}

function pageSetStorage(key, value) {
  localStorage.setItem(key, JSON.stringify({ payload: value }))
}

function pageSetValue(target, value) {
  const el = pageGetEl(target)
  if (el) {
    el.value = value
  } else {
    pageLogError(`Could not set value: ${value}`)
  }
}

function pageUpdateEl(target, attrs = {}) {
  const el = pageGetEl(target)
  if (el) {
    pageUpdateAttrs(el,attrs)
  }
  return el
}

function pageUpdateEls(selector, attrs = {}) {
  const els = pageGetEls(selector)
  els.forEach((el) => {
    pageUpdateAttrs(el, attrs)
  })
}

function pageUpdateAttrs(target, attrs) {
  const el = pageGetEl(target)
  if (el) {
    const nonAttrs = ['aria', 'classes', 'data', 'listeners']
    for (let key in attrs) {
      if (!nonAttrs.includes(key)) {
        el[key] = attrs[key]
      }
    }
    for (let index in attrs.aria) {
      el.setAttribute(`aria-${attrs.aria[index][0]}`, attrs.aria[index][1])
    }
    for (let index in attrs.classes) {
      el.classList.add(attrs.classes[index])
    }
    for (let index in attrs.data) {
      el.dataset[attrs.data[index][0]] = attrs.data[index][1]
    }
    for (let index in attrs.listeners) {
      el.addEventListener(attrs.listeners[index][0], attrs.listeners[index][1])
    }
    return el
  }
}

function pageUpdateSvgAttrs(target, attrs) {
  const el = pageGetEl(target)
  if (el) {
    const nonAttrs = ['classes', 'data', 'listeners', 'styles']
    for (let key in attrs) {
      if (!nonAttrs.includes(key)) {
        el.setAttribute(key, attrs[key])
      }
    }
    for (let index in attrs.classes) {
      el.classList.add(attrs.classes[index])
    }
    for (let index in attrs.data) {
      el.dataset[attrs.data[index][0]] = attrs.data[index][1]
    }
    for (let index in attrs.listeners) {
      el.addEventListener(attrs.listeners[index][0], attrs.listeners[index][1])
    }
    for (let index in attrs.styles) {
      el.style[attrs.styles[index][0]] = attrs.styles[index][1]
    }
    return el
  }
}
