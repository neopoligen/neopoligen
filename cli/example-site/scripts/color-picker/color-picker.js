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
          h: 120,
          collectionShift: 180,
          collectionIndex: 7,
        },
        bravo: {
          l: 0,
          c: 2,
          h: 240,
          collectionShift: 60,
          collectionIndex: 10,
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
    colors: {
      alfa: {
        secondaryH: 180,
      },
      bravo: {
        secondaryH: 60,
      },
    },
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
  addStyle(
    `.primaryButton`,
    `margin: var(--padding-alfa); border: 1px solid black;`
  )
  addStyle(
    `.secondaryButton`,
    `margin: var(--padding-alfa); border: 1px solid black;`
  )
  addStyle(`.tertiaryChip`, `border: 1px solid black; margin: 0.1rem;`)
  // addStyle(`.tertiaryRect`, `border: 1px solid blue; fill: goldenrod;`)
  addStyle(`.sideSwatch`, `min-width: 0.4rem;`)
  addStyle(`.sidebar`, `display: flex;`)
  addStyle(`.sliders`, `display: grid;`)
  addStyle(`.sliders label input`, `margin: var(--padding-bravo);`)

  addStyle(`.activeSecondary::after`, `content: 'active';`)
  addStyle(`.inactiveSecondary::before`, `content: '-';`)
  addStyle(`.inactiveSecondary::after`, `content: '-';`)
  addStyle(`.currentSecondary::before`, `content: 'current';`)

  addStyle(`.activeTertiary`, `margin-bottom: 12px;`)
  addStyle(`.inactiveTertiary`, `margin-top: 12px;`)

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
        }) var(--h-active-${state.active.mode}));`
      )
    })
  })

  primaries().forEach((primary, primaryIndex) => {
    collections().forEach((collection) => {
      collection.forEach((coords) => {
        hValues().forEach((h, hIndex) => {
          const key = `secondaryRect-coords-${primary.key}-${coords[0]}-${coords[1]}-${h}`
          // logMsg(key)
          addStyle(`.${key}`, `fill: var(--${key})`)
        })
      })
    })
  })

  // i think this is depcreated
  // primaries().forEach((primary) => {
  //   lValues().forEach((l, lIndex) => {
  //     cValues().forEach((c, cIndex) => {
  //       hValues().forEach((h, hIndex) => {
  //         const key = `secondaryRect-${primary.secondaries.join(
  //           ''
  //         )}Rect-${l}-${cString(c)}-${h}`
  //         addStyle(`.${key}`, `fill: var(--${key})`)
  //       })
  //     })
  //   })
  // })

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
      const key = `tertiaryRect-${primary.key}-${coords[0]}-${coords[1]}`
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
  <div class="chip chip-${cIndex}-${4 - lIndex}">
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

function buildModeButtons() {
  modes().forEach((mode) => {
    const label = addTo('.modes', 'label', {
      innerHTML: `<span>${mode}</span>`,
    })
    addTo(label, 'input', {
      type: 'radio',
      name: 'mode',
      checked: mode === 'light' ? true : false,
      value: mode,
      listeners: [['input', handleModeClick]],
      classes: [`mode-${mode}`],
    })
  })
}

function buildPrimaryButtons() {
  hValues().forEach((h, hIndex) => {
    const buttonWrapper = addTo('.primaryButtons', 'div', {
      innerHTML: `<div class="primaryButtonWrapper primaryButtonWrapper-${h}">
        <div class="primaryButtonHeader-${h}"></div>
        <div class="primaryButtonHolder-${h}"></div>
        <div class="primaryButtonFooter-${h}"></div>
      </div>`,
    })
    const el = addSvgTo(`.primaryButtonHolder-${h}`, 'svg', {
      classes: ['primaryButton'],
      width: 50,
      height: 50,
      data: [['h', hIndex]],
    })
    cValues().forEach((c, cIndex) => {
      lValues().forEach((l, lIndex) => {
        addSvgTo(el, 'rect', {
          x: lIndex * 10,
          y: 40 - cIndex * 10,
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
      let buttonWrapper = addTo(`.${key}Buttons`, 'div', {
        innerHTML: `
        <div class="secondaryButtonHeader secondaryButtonHeader-${primary.key}-${h}">---</div>
        <div class="secondaryButtonHolder secondaryButtonHolder-${primary.key}-${h}"></div>
        <div class="secondaryButtonFooter secondaryButtonFooter-${primary.key}-${h}">---</div>
        `,
      })
      let btn = addSvgTo(`.secondaryButtonHolder-${primary.key}-${h}`, 'svg', {
        classes: [`secondaryButton`, `secondaryButton-${primary.key}-${h}`],
        width: 30,
        height: 30,
      })
      for (let coord1 = -1; coord1 <= 1; coord1++) {
        for (let coord2 = -1; coord2 <= 1; coord2++) {
          addSvgTo(btn, 'rect', {
            classes: [
              `secondaryRect-coords-${primary.key}-${coord1}-${coord2}-${h}`,
            ],
            x: (coord1 + 1) * 10,
            y: (coord2 + 1) * 10,
            width: 10,
            height: 10,
            //styles: [['fill', 'yellow']],
            data: [
              ['primary', primary.key],
              ['secondaryH', hIndex * state.base.h.interval],
            ],
            listeners: [['click', handleSecondaryButtonClick]],
          })
        }
      }
    })
  })
}

// TODO: Rename Secondary there to Tertiary
function buildTertiaryButtons() {
  primaries().forEach((primary) => {
    collections().forEach((collection, collectionIndex) => {
      const mainKey = primary.secondaries.join('')
      const el = addSvgTo(`.${mainKey}Chips`, 'svg', {
        classes: [
          'tertiaryChip',
          `tertiaryChip-index-${primary.key}-${collectionIndex}`,
        ],
        width: 20,
        height: 40,
      })
      collection.forEach((coords, coordsIndex) => {
        const key = primary.key
        addSvgTo(el, 'rect', {
          classes: [
            'tertiaryRect',
            `tertiaryRect-${key}-${coords[0]}-${coords[1]}`,
          ],
          x: 0,
          y: coordsIndex * 20,
          width: 20,
          height: 20,
          data: [
            ['mode', state.active.mode],
            ['primary', primary.key],
            ['collectionIndex', collectionIndex],
          ],
          listeners: [['click', handleTertiaryButtonClick]],
        })
      })
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
    value: config.value,
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
    value: state.modes.light.l,
  })

  buildSlider({
    key: 'c',
    label: 'Chroma',
    min: 0,
    max: state.base.c.max,
    step: state.base.c.step,
    value: state.modes.light.c,
  })

  buildSlider({
    key: 'h',
    label: 'Hue',
    min: 0,
    max: state.base.h.max,
    step: state.base.h.step,
    value: state.modes.light.h,
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

function debounce(callback, wait) {
  let timeoutId = null
  return (...args) => {
    window.clearTimeout(timeoutId)
    timeoutId = window.setTimeout(() => {
      callback.apply(null, args)
    }, wait)
  }
}

function handleModeClick(event) {
  updateState()
  updateChips()
  updateProps()
}

function handlePrimaryButtonClick(event) {
  state.active.h = parseInt(event.target.dataset.h, 10)
  logMsg(`Switched to primary hue: ${state.active.h}`)
  updateState()
  updateChips()
  updateProps()
}

function handleSecondaryButtonClick(event) {
  console.log(event.target.dataset)
  state.active.colors[event.target.dataset.primary].secondaryH = parseInt(
    event.target.dataset.secondaryH
  )
  updateState()
  updateProps()
}

const handleSliderChange = throttle((event) => {
  const key = event.target.dataset.key
  const value = parseFloat(event.target.value)
  //logMsg(value)
  updateState()
  updateProps()
}, 50)

function handleTertiaryButtonClick(event) {
  // console.log(event.target.dataset)
  state.modes[event.target.dataset.mode].colors[
    event.target.dataset.primary
  ].collectionIndex = parseInt(event.target.dataset.collectionIndex, 10)

  state.modes[event.target.dataset.mode].colors[
    event.target.dataset.primary
  ].collectionShift =
    state.active.colors[event.target.dataset.primary].secondaryH

  // console.log(event.target.dataset)
  // logMsg(
  //   state.modes[event.target.dataset.mode].colors[event.target.dataset.primary]
  //     .collectionIndex
  // )
  updateState()
  updateProps()
}

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
  cValues().forEach((c, cIndex) => {
    lValues().forEach((l, lIndex) => {
      updateEl(`.chip-${lIndex}-${cIndex} .chipTitle`, {
        innerHTML: `#${l}-${cString(c)}-${state.active.h}`,
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
      if (h === state.active.h) {
        updateProp(`--h-active-${mode}`, `${newValue}`)
      }
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


  hValues().forEach((h) => {
    let footerPayload = ""
    if (state.modes[state.active.mode].colors.alfa.h === h) {
      footerPayload += "a "
    }
    if (state.modes[state.active.mode].colors.bravo.h === h) {
      footerPayload += "b "
    }
    if (state.active.h === h) {
      footerPayload += "^ "
    } 
    updateEl(`.primaryButtonFooter-${h}`, {
      innerHTML: footerPayload
    })
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

  // I think this can be removed in favor of addressing
  // the chips via coords
  /*
  primaries().forEach((primary) => {
    lValues().forEach((l, lIndex) => {
      cValues().forEach((c, cIndex) => {
        hValues().forEach((h, hIndex) => {
          const key = `secondaryRect-${primary.secondaries.join('')}Rect-${l}-${cString(
            c
          )}-${h}`
          //let h2 = state.modes[state.active.mode].colors[primary.key].collectionShift
          //let l2 = ((state.modes[state.active.mode].colors[primary.key].l + 100) + (20 * coords[0])) % 100
          // let c2 = ((state.modes[state.active.mode].colors[primary.key].c + 5) + coords[1]) % 5

          //let h2 = (h + state.modes[state.active.mode].colors[primary.key].collectionIndex) % 5
          updateProp(
            `--${key}`,
            `var(--color-${l}-${cString(c)}-${180}-${activeMode()})`
          )
        })
      })
    })
  })*/

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
          updateProp(`--primaryRect-${modeKey}`, `var(--color-${modeKey})`)
        })
      })
    })
  })

  // highlight the current secondary set for each primary
  primaryColors().forEach((primary) => {
    hValues().forEach((h) => {
      const target = `.secondaryButton-${primary}-${h}`
      const targetHeader = `.secondaryButtonHeader-${primary}-${h}`
      const targetFooter = `.secondaryButtonFooter-${primary}-${h}`
      if (state.active.colors[primary].secondaryH === h) {
        updateEl(targetFooter, { innerHTML: 'visible' })
      } else {
        updateEl(targetFooter, { innerHTML: '-' })
      }
      if (
        state.modes[state.active.mode].colors[primary].collectionShift === h
      ) {
        updateEl(targetHeader, { innerHTML: 'current' })
      } else {
        updateEl(targetHeader, { innerHTML: '-' })
      }
    })
  })

  primaries().forEach((primary, primaryIndex) => {
    hValues().forEach((h) => {
      // primary.secondaries.forEach((color) => {
      collections().forEach((collection) => {
        collection.forEach((coords) => {
          const key = `secondaryRect-coords-${primary.key}-${coords[0]}-${coords[1]}-${h}`
          let h2 =
            state.modes[state.active.mode].colors[primary.key].collectionShift
          let l2 =
            (state.modes[state.active.mode].colors[primary.key].l +
              100 +
              20 * coords[0]) %
            100
          let c2 =
            (state.modes[state.active.mode].colors[primary.key].c +
              5 +
              coords[1]) %
            5
          updateProp(
            `--${key}`,
            `var(--color-${l2}-${c2}-${h}-${state.active.mode})`
          )
        })
      })
    })
  })

  // Tertiary Chip Rectangles
  primaries().forEach((primary) => {
    collectionCoords().forEach((coords) => {
      const key = `tertiaryRect-${primary.key}-${coords[0]}-${coords[1]}`
      let h = state.active.colors[primary.key].secondaryH
      let l =
        (state.modes[state.active.mode].colors[primary.key].l +
          100 +
          20 * coords[0]) %
        100
      let c =
        (state.modes[state.active.mode].colors[primary.key].c + 5 + coords[1]) %
        5
      updateProp(`--${key}`, `var(--color-${l}-${c}-${h}-${state.active.mode})`)
    })
  })

  // update chip- text colors
  lValues().forEach((l, lIndex) => {
    cValues().forEach((c, cIndex) => {
      updateProp(`--chip-${lIndex}-${cIndex}`, `blue`)
    })
  })

  // highlight the current secondary set for each primary
  primaryColors().forEach((primary) => {
    collections().forEach((collection, collectionIndex) => {
      const target = `.tertiaryChip-index-${primary}-${collectionIndex}`
      if (
        state.modes[state.active.mode].colors[primary].collectionIndex ===
          collectionIndex &&
        state.active.colors[primary].secondaryH ===
          state.modes[state.active.mode].colors[primary].collectionShift
      ) {
        removeClassFrom(target, 'inactiveTertiary')
        addClassTo(target, 'activeTertiary')
      } else {
        removeClassFrom(target, 'activeTertiary')
        addClassTo(target, 'inactiveTertiary')
      }
    })
  })
}

function updateState() {
  const mode = getValue('input[name=mode]:checked')
  state.active.mode = mode
  state.modes[mode].l = getFloat('.lSlider')
  state.modes[mode].c = getFloat('.cSlider')
  state.modes[mode].h = getFloat('.hSlider')
  updateEl('.currentState', {
    innerHTML: JSON.stringify(state, null, 2),
  })
}

document.addEventListener('DOMContentLoaded', () => {
  addStylesheet()
  buildPrimaryButtons()
  buildChipRows()
  buildChips()
  buildSecondaryButtons()
  buildTertiaryButtons()
  buildSliders()
  buildModeButtons()
  updateProps()
  updateState()
})
