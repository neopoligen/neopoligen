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

function activeMode() {
  return state.active.mode
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

  primaryColors().forEach((color) => {
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

function buildChipRows() {
  lValues().forEach((l, lIndex) => {
    addTo('.chips', 'div', {
      classes: ['chipRow', `chipRow-${lIndex}`],
    })
  })
}

function buildChips() {
  lValues().forEach((l, lIndex) => {
    cValues().forEach((c, cIndex) => {
      addTo(`.chipRow-${lIndex}`, 'div', {
        innerHTML: `
  <div class="chip chip-${lIndex}-${cIndex}">
  <div class="chipSwatch"></div>
  <div class="chipDetails">
    <div class="chipTitle">#</div>
    <div class="chipText">${state.sampleText}</div>
    <div class="chipButtons">
      <div class="chipButtonAlfa">alfa</div>
      <div class="chipButtonBravo">bravo</div>
    </div>
  </div>
  </div>`,
      })
    })
  })
  updateChips()
}

function buildPrimaryButtons() {
  hValues().forEach((h, hIndex) => {
    const el = addSvgTo('.primaryButtons', 'svg', {
      classes: ['primaryButton'],
      width: 50,
      height: 50,
      data: [['h', hIndex]],
    })
    lValues().forEach((l, lIndex) => {
      cValues().forEach((c, cIndex) => {
        addSvgTo(el, 'rect', {
          x: lIndex * 10,
          y: cIndex * 10,
          width: 10,
          height: 10,
          classes: ['primaryRect', `primaryRect-${l}-${cString(c)}-${h}`],
          data: [['h', h]],
          listeners: [['click', handlePrimaryButtonClick]],
        })
      })
    })
  })
}


function buildSecondaryButtons() {
    primaries().forEach((primary) => {
      const key = primary.secondaries.join('')
      hValues().forEach((h, hIndex) => {
        let button = addSvgTo(`.${key}Buttons`, 'svg', {
          classes: [`secondaryButton`], 
          width: 45, 
          height: 45
        })
        /*
        for (let lIndex = 1; lIndex < 4; lIndex++) {
          for (let cIndex = 1; cIndex < 4; cIndex++) {
            addSvgToEl('rect', button, {
              classes: [`secondaryRect`, `${key}Rect-${lValues()[lIndex]}-${cString(cValues()[cIndex])}-${h}`], 
              attrs: [
                ['x', (lIndex - 1) * 15], 
                ['y', (cIndex - 1) * 15],
                ['width', 15],
                ['height', 15]
              ],
              data: [
                ['key', key],
                ['h', hIndex]
              ]
            })
          }
        }
        */
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

function handlePrimaryButtonClick(event) {
  state.active.h = parseInt(event.target.dataset.h, 10)
  logMsg(`Switched to primary hue: ${state.active.h}`)
  // TODO: updateState()
  // TODO: updateChips()
  // TODO: updateProps()
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

function modes() {
  const tmp = []
  for (let mode in state.modes) {
    tmp.push(mode)
  }
  return tmp
}

function primaries() {
  return state.primaries
}

function primaryColors() {
  return [primaries()[0].key, primaries()[1].key]
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

function updateChips() {
    lValues().forEach((l, lIndex) => {
      cValues().forEach((c, cIndex) => {
        updateEl(`.chip-${lIndex}-${cIndex} .chipTitle`, {
          innerHTML: `#${l}-${cString(c)}-${state.active.h}`
        })
      })
    })
  }

function updateProp(key, value) {
  document.documentElement.style.setProperty(
    key,
    value.replaceAll('\n', ' ').replaceAll(/\s\s+/g, ' ')
  )
}

function updateProps() {
  updateProp(`--background-base`, `var(--background-base-${state.active.mode})`)
  updateProp(`--border-alfa`, `1px solid var(--color-alfa)`)
  updateProp(`--padding-alfa`, `1.1rem`)
  updateProp(`--padding-bravo`, `0.7rem`)
  updateProp('--size-1', '2.986rem')
  updateProp('--size-2', '2.488rem')
  updateProp('--size-3', '2.074rem')
  updateProp('--size-4', '1.728rem')
  updateProp('--size-5', '1.44rem')
  updateProp('--size-6', '1.2rem')
  updateProp('--size-7', '1rem')
  updateProp('--size-8', '0.833rem')
  updateProp('--size-9', '0.694rem')
  updateProp('--size-10', '0.579rem')

  modes().forEach((mode) => {
    updateProp(`--l-base-${activeMode()}`, `${state.modes[activeMode()].l}%`)
    updateProp(`--c-base-${activeMode()}`, `${state.modes[activeMode()].c}`)
    updateProp(`--h-base-${activeMode()}`, `${state.modes[activeMode()].h}`)
  })

  modes().forEach((mode) => {
    lValues().forEach((l, lIndex) => {
      const newValue = `${
        (state.modes[state.active.mode].l + l) % state.base.l.max
      }%`
      logMsg(`--l${l}-${mode} - ${newValue}`)
      updateProp(`--l${l}-${mode}`, `${newValue}`)
    })
    cValues().forEach((c, cIndex) => {
      const newValue = (state.modes[state.active.mode].c + c) % state.base.c.max
      updateProp(`--c${cString(c)}-${mode}`, `${newValue}`)
    })
    hValues().forEach((h, hIndex) => {
      const newValue = `${
        (state.modes[state.active.mode].h + h) % state.base.h.max
      }`
      updateProp(`--h${h}-${mode}`, `${newValue}`)
    })
  })

  modes().forEach((mode) => {
    lValues().forEach((l, lIndex) => {
      cValues().forEach((c, cIndex) => {
        hValues().forEach((h, hIndex) => {
          updateProp(
            `--color-${l}-${cString(c)}-${h}-${mode}`,
            `oklch(var(--l${l}-${mode}) var(--c${cString(
              c
            )}-${mode}) var(--h${h}-${mode}))`
          )
        })
      })
    })
  })

  modes().forEach((mode) => {
    primaryColors().forEach((color) => {
      updateProp(
        `--color-${color}-${mode}`,
        `oklch(
            var(--l${state.modes[mode].colors[color].l}-${mode}) 
            var(--c${cString(state.modes[mode].colors[color].c)}-${mode}) 
            var(--h${state.modes[mode].colors[color].h}-${mode}) 
          )`
      )
    })
  })

  modes().forEach((mode) => {
    updateProp(
      `--background-base-${mode}`,
      `oklch(var(--l-base-${mode}) var(--c-base-${mode}) var(--h-base-${mode}))`
    )
  })

  primaryColors().forEach((color) => {
    updateProp(`--color-${color}`, `var(--color-${color}-${state.active.mode})`)
  })

  /*
    primaries().forEach((primary, primaryIndex) => {
      primary.secondaries.forEach((color) => {
        collections().forEach((collection) => {
          collection.forEach((coords) => {
            const key = `${color}Rect-${coords[0]}-${coords[1]}`
            updateProp(`--${key}`, `oklch(40% 0.2 240)`)
          })
        })
      })
    })
    */

  primaries().forEach((primary) => {
    lValues().forEach((l, lIndex) => {
      cValues().forEach((c, cIndex) => {
        hValues().forEach((h, hIndex) => {
          const key = `${primary.secondaries.join('')}Rect-${l}-${cString(
            c
          )}-${h}`
          updateProp(
            `--${key}`,
            `var(--color-${l}-${cString(c)}-${h}-${activeMode()})`
          )
        })
      })
    })
  })

  lValues().forEach((l) => {
    cValues().forEach((c) => {
      hValues().forEach((h) => {
        const baseKey = `${l}-${cString(c)}-${h}`
        const modeKey = `${l}-${cString(c)}-${h}-${activeMode()}`
        updateProp(`--color-${baseKey}`, `var(--color-${modeKey})`)
        updateProp(`--primaryRect-${baseKey}`, `var(--primaryRect-${modeKey})`)
      })
    })
  })

  lValues().forEach((l) => {
    cValues().forEach((c) => {
      hValues().forEach((h) => {
        modes().forEach((mode) => {
          const modeKey = `${l}-${cString(c)}-${h}-${mode}`
          logMsg(`--primaryRect-${modeKey}`, `var(--color-${modeKey})`)
          updateProp(`--primaryRect-${modeKey}`, `var(--color-${modeKey})`)
        })
      })
    })
  })

  primaries().forEach((primary) => {
    collectionCoords().forEach((coords) => {
      const key = `secondaryRect-${primary.key}-${coords[0]}-${coords[1]}`
      updateProp(`--${key}`, `green`)
    })
  })
}

document.addEventListener('DOMContentLoaded', () => {
  addStylesheet()
  updateProps()
  buildPrimaryButtons()
  buildChipRows()
  buildChips()
  buildSecondaryButtons()
  //buildSecondaryChips()
  // should be ready: buildSliders()
  //buildModeButtons()
})
