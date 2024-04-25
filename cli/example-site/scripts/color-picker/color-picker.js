let state = {
  base: {
    l: {
      interval: 20,
      max: 100,
      step: 0.001,
    },
    c: {
      interval: 0.1,
      max: 0.5,
      step: 0.00001,
    },
    h: {
      interval: 60,
      max: 360,
      step: 0.001,
    },
  },
  collections: [
    [
      [-1, -1],
      [0, 0],
    ],
    [
      [-1, 0],
      [0, 0],
    ],
    [
      [-1, 1],
      [0, 0],
    ],
    [
      [0, -1],
      [0, 0],
    ],
    [
      [0, 1],
      [0, 0],
    ],
    [
      [1, -1],
      [0, 0],
    ],
    [
      [1, 0],
      [0, 0],
    ],
    [
      [1, 1],
      [0, 0],
    ],
    [
      [-1, -1],
      [1, -1],
    ],
    [
      [-1, 0],
      [1, 0],
    ],
    [
      [-1, 1],
      [1, 1],
    ],
    [
      [-1, -1],
      [-1, 1],
    ],
    [
      [0, -1],
      [0, 1],
    ],
    [
      [1, -1],
      [1, 1],
    ],
    [
      [-1, -1],
      [1, 1],
    ],
    [
      [1, -1],
      [-1, 1],
    ],
  ],
  colors: ['alfa', 'bravo', 'charlie', 'delta', 'echo', 'foxtrot'],
  primaries: [
    { key: 'alfa', secondaries: ['charlie', 'delta'] },
    { key: 'bravo', secondaries: ['echo', 'foxtrot'] },
  ],
  modes: {
    light: {
      l: 70,
      c: 0.1,
      h: 247,
      colors: {
        alfa: {
          l: 20,
          c: 2,
          h: 0,
          collectionShift: 0,
          collectionIndex: 7,
        },
        bravo: {
          l: 60,
          c: 1,
          h: 0,
          collectionShift: 60,
          collectionIndex: 0,
        },
      },
    },
    dark: {
      l: 30,
      c: 0.3637,
      h: 93.484,
      colors: {
        alfa: {
          l: 40,
          c: 0,
          h: 0,
          collectionShift: 0,
          collectionIndex: 0,
        },
        bravo: {
          l: 60,
          c: 0,
          h: 0,
          collectionShift: 0,
          collectionIndex: 0,
        },
      },
    },
  },
  sampleText: 'Lorem ipsum sit amet elit leo augue ex nec null tellus',
  active: {
    mode: 'light',
    h: 0,
  },
}

function addStyle(key, value) {
  appendInnerText(
    `.stylesheet`,
    ` ${key} { ${value.replaceAll('\n', ' ').replaceAll(/\s\s+/g, ' ')} }`
  )
}

function addStylesheet() {
  const styleEl = addTo(document.head, 'style', { classes: ['stylesheet'] })
  addStyle(`h1, h2, h3, h4, h5, h6`, `color: var(--color-alfa);`)
  addStyle(
    `body`,
    `color: var(--color-bravo); background-color: var(--background-base);`
  )
  addStyle(`.background-base`, `var(--background-base);`)
  addStyle(`.background-white`, `background-color: white;`)
  addStyle(`.background-black`, `background-color: black;`)
  addStyle(
    `.chipRow`,
    `display: flex; flex-wrap: wrap; gap: 1.3rem; margin-bottom: 1.3rem;`
  )
  addStyle(
    `.chipSwatch`,
    `min-height: 1rem; min-width: 1rem; margin-left: 0.8rem;`
  )
  addStyle(
    `.chip`,
    `width: 10rem; display: grid; grid-template-columns: 0.4rem 1fr; font-size: var(--size-9);`
  )
  addStyle(`.chipTitle`, `font-weight: 700;`)
  addStyle(`.chipButtons`, `display: grid; grid-template-columns: 1fr 1fr;`)
  addStyle(`.charlieDeltaButton`, `padding: var(--padding-alfa);`)
  addStyle(`.echoFoxtrotButton`, `padding: var(--padding-alfa);`)
  addStyle(`.pickerWrapper`, `display: grid; grid-template-columns: 4rem 1fr;`)
  addStyle(`.primaryButton`, `margin: var(--padding-alfa);`)
  addStyle(`.secondaryButton`, `padding: var(--padding-alfa);`)
  addStyle(`.sideSwatch`, `min-width: 0.4rem;`)
  addStyle(`.sidebar`, `display: flex;`)
  addStyle(`.sliders`, `display: grid;`)
  addStyle(`.sliders label input`, `margin: var(--padding-bravo);`)

  lValues().forEach((l, lIndex) => {
    cValues().forEach((c, cIndex) => {
      hValues().forEach((h, hIndex) => {
        addStyle(
          `.color-${l}-${cString(c)}-${h}`,
          `color: var(--color-${l}-${cString(c)}-${h}-${state.active.mode});`
        )
        addStyle(
          `.charlieDeltaRect-${l}-${cString(c)}-${h}`,
          `fill: var(--color-${l}-${cString(c)}-${h}-${state.active.mode});`
        )
        addStyle(
          `.echoFoxtrotRect-${l}-${cString(c)}-${h}`,
          `fill: var(--color-${l}-${cString(c)}-${h}-${state.active.mode});`
        )
      })
    })
  })

  primaries().forEach((color) => {
    addStyle(
      `.color-${color}`,
      `color: var(--color-${color}-${state.active.mode});`
    )
  })

  lValues().forEach((l, lIndex) => {
    cValues().forEach((c, cIndex) => {
      addStyle(
        `.chip-${lIndex}-${cIndex}`,
        `color: oklch(var(--l${l}-${state.active.mode}) var(--c${cString(c)}-${
          state.active.mode
        }) var(--h${state.active.h}-${state.active.mode}));`
      )
    })
  })

  primaries().forEach((primary, primaryIndex) => {
    primary.secondaries.forEach((color) => {
      collections().forEach((collection) => {
        collection.forEach((coords) => {
          const key = `${color}Rect-${coords[0]}-${coords[1]}`
          addStyle(`.${key}`, `fill: var(--${key})`)
        })
      })
    })
  })

  primaries().forEach((primary) => {
    lValues().forEach((l, lIndex) => {
      cValues().forEach((c, cIndex) => {
        hValues().forEach((h, hIndex) => {
          const key = `${primary.secondaries.join('')}Rect-${l}-${cString(
            c
          )}-${h}`
          addStyle(`.${key}`, `fill: var(--${key})`)
        })
      })
    })
  })

  lValues().forEach((l, lIndex) => {
    cValues().forEach((c, cIndex) => {
      hValues().forEach((h, hIndex) => {
        const key = `primaryRect-${l}-${cString(c)}-${h}`
        addStyle(`.${key}`, `fill: var(--${key})`)
      })
    })
  })

  primaries().forEach((primary) => {
    collectionCoords().forEach((coords) => {
      const key = `secondaryRect-${primary.key}-${coords[0]}-${coords[1]}`
      addStyle(`.${key}`, `fill: var(--${key})`)
    })
  })
}

function buildSlider(config) {
  const label = addTo('.sliders', 'label', {
    innerHTML: `<span>${config.label}</span>`,
  })
  addTo(label, 'input', {
    classes: ['slider', `${config.key}Slider`],
    name: `${config.key}Slider`,
    type: 'range',
    min: config.min,
    max: config.max,
    step: config.step,
    data: [['key', config.key]],
    listeners: [['input', handleSliderChange]],
  })
}

function buildSliders() {
  logMsg('Building sliders')

  buildSlider({
    key: 'l',
    label: 'Lightness',
    min: 0,
    max: state.base.l.max,
    step: state.base.l.step,
  })

  buildSlider({
    key: 'c',
    label: 'Chroma',
    min: 0,
    max: state.base.c.max,
    step: state.base.c.step,
  })

  buildSlider({
    key: 'h',
    label: 'Hue',
    min: 0,
    max: state.base.h.max,
    step: state.base.h.step,
  })
}

function collectionCoords() {
    const refChecks = []
    const response = []
    state.collections.forEach((collection) => {
      collection.forEach((coords) => {
        const refCheck = `${coords[0]}-${coords[1]}`
        if (!refChecks.includes(refCheck)) {
          refChecks.push(refCheck)
          response.push([coords[0], coords[1]])
        }
      })
    })
    return response 
  }

function collections() {
    return state.collections
  }

function cString(c) {
  return Math.floor(c * 10)
}

function cValues() {
  const tmp = []
  for (let c = 0; c < state.base.c.max; c += state.base.c.interval) {
    tmp.push(c)
  }
  return tmp
}

const handleSliderChange = throttle((event) => {
  const key = event.target.dataset.key
  const value = parseFloat(event.target.value)
  logMsg(value)
  //update(`.${key}Value`, { innerHTML: value })
  //updateState()
  //updateProps()
}, 30)

function hValues() {
  const tmp = []
  for (let h = 0; h < state.base.h.max; h += state.base.h.interval) {
    tmp.push(h)
  }
  return tmp
}

function lValues() {
  const tmp = []
  for (let l = 0; l < state.base.l.max; l += state.base.l.interval) {
    tmp.push(l)
  }
  return tmp
}

function primaries() {
  return state.primaries
}

function throttle(func, timeFrame) {
  var lastTime = 0
  return function (...args) {
    var now = new Date()
    if (now - lastTime >= timeFrame) {
      func(...args)
      lastTime = now
    }
  }
}

document.addEventListener('DOMContentLoaded', () => {
  addStylesheet()

  
  //buildSliders()
})
