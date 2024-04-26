////////////////////////////////////////////////////////////
// yagni-js

// TODO: Rename 'parent' to 'target'
// and internal 'target' to 'el'


function addClassTo(target, className) {
  const el = getEl(target)
  if (el) {
    el.classList.add(className)
    return el
  }
}


function addListenerTo(target, event, func) {
  const el = getEl(target)
  if (el) {
    el.addEventListener(event, func)
    return el
  }
}


function addSvgTo(parent, tag, attrs = {}) {
  const target = getEl(parent)
  if (target) {
    const el = document.createElementNS('http://www.w3.org/2000/svg', tag)
    updateSvgAttrs(el, attrs)
    target.appendChild(el)
    return el
  }
}

function addTo(parent, tag, attrs = {}) {
  const target = getEl(parent)
  if (target) {
    const el = document.createElement(tag)
    updateAttrs(el, attrs)
    target.appendChild(el)
    return el
  }
}

function addToFront(parent, tag, attrs = {}) {
  const target = getEl(parent)
  if (target) {
    const el = document.createElement(tag)
    updateAttrs(el, attrs)
    if (target.hasChildNodes()) {
      const first_child = target.firstChild
      target.insertBefore(el, first_child)
    } else {
      target.appendChild(el)
    }
    return el
  }
}

function appendInnerText(target, text) {
  const el = getEl(target)
  if (el) {
    el.innerText = el.innerText + text
  }
}

function getEl(target) {
  if (typeof target === 'string') {
    const el = document.querySelector(target)
    if (el) {
      return el
    } else {
      logError(`Could not find querySelector for: ${target}`)
      return undefined
    }
  } else if (target) {
    return target
  } else {
    logError(`Could not get element: ${target}`)
    return undefined
  }
}

function getFloat(selector) {
  const el = getEl(selector)
  if (el) {
    return parseFloat(el.value)
  } else {
    return undefined
  }
}

function getInt(selector) {
  const el = getEl(selector)
  if (el) {
    return parseInt(el.value, 10)
  } else {
    return undefined
  }
}

function getStorage(key, defaultValue = undefined) {
  return localStorage.getItem(key)
    ? JSON.parse(localStorage.getItem(key)).payload
    : defaultValue
}

function getValue(selector) {
  const el = getEl(selector)
  if (el) {
    return el.value
  } else {
    return undefined
  }
}

function logError(msg) {
  console.error(`${Date.now()} - ERROR: ${msg}`)
}

function logMsg(msg) {
  console.log(`${Date.now()} - INFO: ${msg}`)
}

function removeClassFrom(target, className) {
  const el = getEl(target)
  if (el) {
    el.classList.remove(className)
    return el
  }
}

function setStorage(key, value) {
  localStorage.setItem(key, JSON.stringify({ payload: value }))
}

function updateEl(target, attrs = {}) {
  const el = getEl(target)
  if (el) {
    updateAttrs(el,attrs)
  }
  return el
}

function updateAttrs(target, attrs) {
  const el = getEl(target)
  if (el) {
    const nonAttrs = ['classes', 'data', 'listeners']
    for (let key in attrs) {
      if (!nonAttrs.includes(key)) {
        el[key] = attrs[key]
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
    return el
  }
}

function updateSvgAttrs(target, attrs) {
  const el = getEl(target)
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
