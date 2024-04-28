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

function addSvgTo(target, tag, attrs = {}) {
  const el = getEl(target)
  if (el) {
    const svg = document.createElementNS('http://www.w3.org/2000/svg', tag)
    updateSvgAttrs(svg, attrs)
    el.appendChild(svg)
    return svg
  }
}

function addTo(target, tag, attrs = {}) {
  const el = getEl(target)
  if (el) {
    const newEl = document.createElement(tag)
    updateAttrs(newEl, attrs)
    el.appendChild(newEl)
    return newEl
  }
}

function addToFront(target, tag, attrs = {}) {
  const el = getEl(target)
  if (el) {
    const newEl = document.createElement(tag)
    updateAttrs(newEl, attrs)
    if (el.hasChildNodes()) {
      const first_child = el.firstChild
      el.insertBefore(newEl, first_child)
    } else {
      el.appendChild(newEl)
    }
    return newEl
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

function getFloat(target) {
  const el = getEl(target)
  if (el) {
    return parseFloat(el.value)
  } else {
    return undefined
  }
}

function getInt(target) {
  const el = getEl(target)
  if (el) {
    return parseInt(el.value, 10)
  } else {
    return undefined
  }
}

function getRadioValue(name) {
  const el = getEl(`[name=${name}]:checked`)
  if (el) {
    return el.value
  } else {
    return undefined
  }
}

function getStorage(key, defaultValue = undefined) {
  return localStorage.getItem(key)
    ? JSON.parse(localStorage.getItem(key)).payload
    : defaultValue
}

function getValue(target) {
  const el = getEl(target)
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

function setValue(target, value) {
  const el = getEl(target)
  if (el) {
    el.value = value
  } else {
    logError(`Could not set value: ${value}`)
  }
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
