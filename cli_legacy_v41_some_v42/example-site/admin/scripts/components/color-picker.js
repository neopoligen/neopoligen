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
      l: 90.762,
      c: 0.06625,
      h: 252.9,
      colors: {
        alfa: {
          l: 40,
          c: 2,
          h: 180,
          collectionShift: 180,
          collectionIndex: 7,
        },
        bravo: {
          l: 40,
          c: 0,
          h: 300,
          collectionShift: 60,
          collectionIndex: 10,
        },
      },
    },
    dark: {
      l: 16.009,
      c: 0.06625,
      h: 252.9,
      colors: {
        alfa: {
          l: 40,
          c: 2,
          h: 180,
          collectionShift: 180,
          collectionIndex: 7,
        },
        bravo: {
          l: 80,
          c: 0,
          h: 300,
          collectionShift: 60,
          collectionIndex: 10,
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

let childWindow
const childWindowName = 'previewWindow'

let activeStyles = {}

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

  /*
  primaryColors().forEach((color) => {
    addStyle(
      `.color-${color}`,
      `color: var(--color-${color}-${state.active.mode});`
    )
  })
  */

  lValues().forEach((l, lIndex) => {
    cValues().forEach((c, cIndex) => {
      addStyle(
        `.chip-${l}-${cIndex}`,
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

function baseCSS() {
  let response = `
*, 
*::before, 
*::after {
  box-sizing: border-box;
}

* {
  margin: 0;
}

a, a:active {
  color: var(--color-charlie);
  text-decoration: none;
}

a:hover, a:focus {
  color: var(--color-bravo);
}

.blue {
  color: blue;
}

body { 
  background-color: var(--color-base);
  color: var(--color-bravo); 
  font-size: 16px;
  line-height: 1.5; 
  font-family: 'Inter';
}

.flow > :where(:not(:first-child)) {
  margin-top: var(--flow-space, 1em);
}

.footnote_heading {
  display: flex;
  flex-wrap: wrap;
}

.green {
  color: green;
}

h1, h2, h3, h4, h5, h6 {
  color: var(--color-alfa);
  line-height: 1.1;
  text-wrap: balance;
  font-weight: 900;
}

h1 { 
  font-size: var(--size-1);
  margin-top: 1em; 
}

h2 { 
  font-size: var(--size-2); 
  --flow-space: 1.3em;
}

h3 { 
  font-size: var(--size-3); 
  --flow-space: 1.0em;

}

h4 { 
  font-size: var(--size-4); 
}

h5 { 
  font-size: var(--size-5); 
}

h6 { 
  font-size: var(--size-6); 
}

img {
  max-width: 100%;
  display: block;
}

::marker {
  color: var(--color-bravo);
}

.numberedLines {
  counter-reset: lineNumber;
}

.numberedLine {
  counter-increment: lineNumber;
}

.numberedLine:before {
  display: inline-block;
  color: goldenrod;
  content: counter(lineNumber);
  padding-right: 0.7rem;
  text-align: right;
  width: 2rem;
}

pre {
  white-space: pre-wrap; 
  overflow-wrap: break-word;
}

.red {
  color: red;
}

[role="ld-mode"] {
  color: var(--color-bravo-60);
  background-color: var(--color-base);
  border: none;
  cursor: pointer;
  font: inherit;
  outline: none;
}

[role="ld-mode"]:hover {
  color: var(--color-charlie);
}

[role="ld-mode"][aria-selected="true"] {
  color: var(--color-bravo);
  border-bottom: 1px solid var(--color-bravo);
}

.two-column {
  max-width: var(--width-alfa);
  margin-top: 0;
  margin-inline: auto;
  align-items: start;
  display: flex;
  flex-wrap: wrap;
  gap: 1em;
}

.two-column > :first-child {
  flex-basis: 20ch;
  flex-grow: 1;
  outline: 1px solid blue;
}

.two-column > :last-child {
  flex-basis: 62ch;
  flex-grow: 9999;
}

.wrapper {
  width: min(100vw - 3rem, 58ch);
  margin-inline: auto;
}`.trim()

  return response
}

function baseFont() {
  return `
@font-face {
  font-family: 'Inter';
  src: url('/theme/fonts/Inter-VariableFont_slnt,wght.ttf') format('opentype');
}`.trim()
}

function baseProps() {
  let response = ``

  response += prop(`--size-1`, `2.986rem`)
  response += prop(`--size-2`, `2.488rem`)
  response += prop(`--size-3`, `2.074rem`)
  response += prop(`--size-4`, `1.728rem`)
  response += prop(`--size-5`, `1.44rem`)
  response += prop(`--size-6`, `1.2rem`)
  response += prop(`--size-7`, `1rem`)
  response += prop(`--size-8`, `0.833rem`)
  response += prop(`--size-9`, `0.694rem`)
  response += prop(`--size-10`, `0.579rem`)

  response += prop(`--color-black`, `rgb(0 0 0)`)
  response += prop(`--border-black`, `1px solid var(--color-black)`)
  response += prop(`--color-white`, `rgb(255 255 255)`)
  response += prop(`--border-white`, `1px solid var(--color-white)`)

  for (let alpha = 10; alpha <= 90; alpha = alpha + 10) {
    response += prop(`--color-black-${alpha}`, `rgb(0 0 0 / ${alpha}%)`)
    response += prop(
      `--border-black-${alpha}`,
      `1px solid var(--color-black-${alpha})`
    )
    response += prop(`--color-white-${alpha}`, `rgb(255 255 255 / ${alpha}%)`)
    response += prop(
      `--border-white-${alpha}`,
      `1px solid var(--color-white-${alpha})`
    )
  }

  return response.trim()
}

function buildChipRows() {
  for (let cIndex = 4; cIndex >= 0; cIndex--) {
    addTo('.chips', 'div', {
      classes: ['chipRow', `chipRow-${cIndex}`],
    })
  }
}

function buildChips() {
  lValues().forEach((l, lIndex) => {
    cValues().forEach((c, cIndex) => {
      addTo(`.chipRow-${cIndex}`, 'div', {
        innerHTML: `
  <div class="chip chip-${l}-${cIndex}">
  <div class="x-chipSwatch"></div>
  <div class="chipDetails">
    <div class="chipTitle">#</div>
    <div class="chipText">${state.sampleText}</div>
    <div class="chipButtons-${l}-${cIndex}"></div>
  </div>
  </div>`,
      })

      primaryColors().forEach((color) => {
        addTo(`.chipButtons-${l}-${cIndex}`, 'button', {
          classes: [`chipButton-${color}-${l}-${cIndex}`],
          innerHTML: color,
          data: [
            ['color', color],
            ['l', l],
            ['cIndex', cIndex],
          ],
          listeners: [['click', handleColorButtonClick]],
        })
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
  const label = addTo(`.sliders`, 'label', {
    for: `slider${config.key}`,
    innerHTML: `<span>${config.label}</span>`,
  })
  addTo(`.sliders`, 'input', {
    classes: ['slider', `${config.key}Slider`],
    name: `${config.key}Slider`,
    id: `slider${config.key}`,
    type: 'range',
    min: config.min,
    max: config.max,
    step: config.step,
    data: [['key', config.key]],
    listeners: [['input', handleSliderChange]],
    value: config.value,
  })
  addTo(`.sliders`, `button`, {
    classes: [`getFromButton-${config.key}`],
    innerHTML: `Get From: dark mode`,
    listeners: [
      [`click`, handleGetFromClick]
    ],
    data: [
      [`key`, config.key],
    ]
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

function colors() {
  return state.colors
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

function handleColorButtonClick(event) {
  console.log(event.target.dataset)
  state.modes[state.active.mode].colors[event.target.dataset.color].l =
    parseInt(event.target.dataset.l, 10)
  state.modes[state.active.mode].colors[event.target.dataset.color].c =
    parseInt(event.target.dataset.cIndex, 10)
  state.modes[state.active.mode].colors[event.target.dataset.color].h =
    state.active.h
  updateState()
  updateProps()
}

function handleGetFromClick(event) {
  const key = event.target.dataset.key
  const otherMode = state.active.mode === `light` ? `dark` : `light`
  setValue(`.${key}Slider`, state.modes[otherMode][key])
  updateState()
  updateChips()
  updateProps()
}

function handleModeClick(event) {
  state.active.mode = getRadioValue('mode')
  const otherMode = state.active.mode === `light` ? `dark` : `light`
  lch().forEach((key) => {
    setValue(`.${key}Slider`, state.modes[state.active.mode][key])
    setHTML(`.getFromButton-${key}`, `Get From: ${otherMode} mode`)
  })
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

const handleSliderChange = debounce((event) => {
  const key = event.target.dataset.key
  const value = parseFloat(event.target.value)
  //logMsg(value)
  updateState()
  updateProps()
}, 40)

function handleTertiaryButtonClick(event) {
  // console.log(event.target.dataset)
  state.modes[event.target.dataset.mode].colors[
    event.target.dataset.primary
  ].collectionIndex = parseInt(event.target.dataset.collectionIndex, 10)

  state.modes[event.target.dataset.mode].colors[
    event.target.dataset.primary
  ].collectionShift =
    state.active.colors[event.target.dataset.primary].secondaryH

  console.log(event.target.dataset)
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

function launchPreviewWindow() {
  logMsg('launch')
}

function lch() {
  return [`l`, `c`, `h`]
}

// Deprecated: Remove in favor of modeProps(mode)
function lightModeProps() {
  let response = ``

  // base for background
  response += prop(
    `--color-base`,
    `oklch(${state.modes.light.l}% ${state.modes.light.c} ${state.modes.light.h})`
  )
  response += prop(`--border-base`, `1px solid var(--color-base)`)

  // alfa
  response += prop(
    `--color-alfa`,
    `oklch(${(state.modes.light.l + state.modes.light.colors.alfa.l) % 100}% ${
      ((state.modes.light.c * 10 + state.modes.light.colors.alfa.c) % 5) / 10
    } ${(state.modes.light.h + state.modes.light.colors.alfa.h) % 360})`
  )
  response += prop(`--border-alpha`, `1px solid var(--color-alpha)`)

  // bravo
  response += prop(
    `--color-bravo`,
    `oklch(${(state.modes.light.l + state.modes.light.colors.bravo.l) % 100}% ${
      ((state.modes.light.c * 10 + state.modes.light.colors.bravo.c) % 5) / 10
    } ${(state.modes.light.h + state.modes.light.colors.bravo.h) % 360})`
  )
  response += prop(`--border-bravo`, `1px solid var(--color-bravo)`)

  // charlie
  response += prop(
    `--color-charlie`,
    `oklch(${
      (state.modes.light.l +
        state.modes.light.colors.alfa.l +
        state.collections[state.modes.light.colors.alfa.collectionIndex][0][0] *
          state.base.l.interval) %
      100
    }% ${
      ((state.modes.light.c * 10 +
        state.modes.light.colors.alfa.c +
        state.collections[state.modes.light.colors.alfa.collectionIndex][0][1] *
          (state.base.c.interval * 10)) %
        5) /
      10
    } ${
      (state.modes.light.h +
        state.modes.light.colors.alfa.h +
        state.modes.light.colors.alfa.collectionShift) %
      360
    })`
  )
  response += prop(`--border-charlie`, `1px solid var(--color-charlie)`)

  // delta
  response += prop(
    `--color-delta`,
    `oklch(${
      (state.modes.light.l +
        state.modes.light.colors.alfa.l +
        state.collections[state.modes.light.colors.alfa.collectionIndex][1][0] *
          state.base.l.interval) %
      100
    }% ${
      ((state.modes.light.c * 10 +
        state.modes.light.colors.alfa.c +
        state.collections[state.modes.light.colors.alfa.collectionIndex][1][1] *
          (state.base.c.interval * 10)) %
        5) /
      10
    } ${
      (state.modes.light.h +
        state.modes.light.colors.alfa.h +
        state.modes.light.colors.alfa.collectionShift) %
      360
    })`
  )
  response += prop(`--border-delta`, `1px solid var(--color-delta)`)

  // echo
  response += prop(
    `--color-echo`,
    `oklch(${
      (state.modes.light.l +
        state.modes.light.colors.bravo.l +
        state.collections[
          state.modes.light.colors.bravo.collectionIndex
        ][0][0] *
          state.base.l.interval) %
      100
    }% ${
      ((state.modes.light.c * 10 +
        state.modes.light.colors.bravo.c +
        state.collections[
          state.modes.light.colors.bravo.collectionIndex
        ][0][1] *
          (state.base.c.interval * 10)) %
        5) /
      10
    } ${
      (state.modes.light.h +
        state.modes.light.colors.bravo.h +
        state.modes.light.colors.bravo.collectionShift) %
      360
    })`
  )
  response += prop(`--border-echo`, `1px solid var(--color-echo)`)

  // foxtrot
  response += prop(
    `--color-foxtrot`,
    `oklch(${
      (state.modes.light.l +
        state.modes.light.colors.bravo.l +
        state.collections[
          state.modes.light.colors.bravo.collectionIndex
        ][1][0] *
          state.base.l.interval) %
      100
    }% ${
      ((state.modes.light.c * 10 +
        state.modes.light.colors.bravo.c +
        state.collections[
          state.modes.light.colors.bravo.collectionIndex
        ][1][1] *
          (state.base.c.interval * 10)) %
        5) /
      10
    } ${
      (state.modes.light.h +
        state.modes.light.colors.bravo.h +
        state.modes.light.colors.bravo.collectionShift) %
      360
    })`
  )
  response += prop(`--border-foxtrot`, `1px solid var(--color-foxtrot)`)

  for (let alpha = 10; alpha <= 90; alpha = alpha + 10) {
    response += prop(
      `--color-base-${alpha}`,
      `oklch(${state.modes.light.l}% ${state.modes.light.c} ${state.modes.light.h} / ${alpha}%)`
    )

    // alfa
    response += prop(
      `--color-alfa-${alpha}`,
      `oklch(${
        (state.modes.light.l + state.modes.light.colors.alfa.l) % 100
      }% ${
        ((state.modes.light.c * 10 + state.modes.light.colors.alfa.c) % 5) / 10
      } ${
        (state.modes.light.h + state.modes.light.colors.alfa.h) % 360
      } / ${alpha})`
    )
    response += prop(
      `--border-alfa-${alpha}`,
      `1px solid var(--color-alfa-${alpha})`
    )

    // bravo
    response += prop(
      `--color-bravo-${alpha}`,
      `oklch(${
        (state.modes.light.l + state.modes.light.colors.bravo.l) % 100
      }% ${
        ((state.modes.light.c * 10 + state.modes.light.colors.bravo.c) % 5) / 10
      } ${
        (state.modes.light.h + state.modes.light.colors.bravo.h) % 360
      }) / ${alpha}`
    )
    response += prop(
      `--border-bravo-${alpha}`,
      `1px solid var(--color-bravo-${alpha})`
    )

    // charlie
    response += prop(
      `--color-charlie-${alpha}`,
      `oklch(${
        (state.modes.light.l +
          state.modes.light.colors.alfa.l +
          state.collections[
            state.modes.light.colors.alfa.collectionIndex
          ][0][0] *
            state.base.l.interval) %
        100
      }% ${
        ((state.modes.light.c * 10 +
          state.modes.light.colors.alfa.c +
          state.collections[
            state.modes.light.colors.alfa.collectionIndex
          ][0][1] *
            (state.base.c.interval * 10)) %
          5) /
        10
      } ${
        (state.modes.light.h +
          state.modes.light.colors.alfa.h +
          state.modes.light.colors.alfa.collectionShift) %
        360
      } / ${alpha})`
    )
    response += prop(
      `--border-charlie-${alpha}`,
      `1px solid var(--color-charlie-${alpha})`
    )

    // delta
    response += prop(
      `--color-delta-${alpha}`,
      `oklch(${
        (state.modes.light.l +
          state.modes.light.colors.alfa.l +
          state.collections[
            state.modes.light.colors.alfa.collectionIndex
          ][1][0] *
            state.base.l.interval) %
        100
      }% ${
        ((state.modes.light.c * 10 +
          state.modes.light.colors.alfa.c +
          state.collections[
            state.modes.light.colors.alfa.collectionIndex
          ][1][1] *
            (state.base.c.interval * 10)) %
          5) /
        10
      } ${
        (state.modes.light.h +
          state.modes.light.colors.alfa.h +
          state.modes.light.colors.alfa.collectionShift) %
        360
      } / ${alpha})`
    )
    response += prop(
      `--border-delta-${alpha}`,
      `1px solid var(--color-delta-${alpha})`
    )

    // echo
    response += prop(
      `--color-echo-${alpha}`,
      `oklch(${
        (state.modes.light.l +
          state.modes.light.colors.bravo.l +
          state.collections[
            state.modes.light.colors.bravo.collectionIndex
          ][0][0] *
            state.base.l.interval) %
        100
      }% ${
        ((state.modes.light.c * 10 +
          state.modes.light.colors.bravo.c +
          state.collections[
            state.modes.light.colors.bravo.collectionIndex
          ][0][1] *
            (state.base.c.interval * 10)) %
          5) /
        10
      } ${
        (state.modes.light.h +
          state.modes.light.colors.bravo.h +
          state.modes.light.colors.bravo.collectionShift) %
        360
      } / ${alpha})`
    )
    response += prop(
      `--border-echo-${alpha}`,
      `1px solid var(--color-echo-${alpha})`
    )

    // foxtrot
    response += prop(
      `--color-foxtrot-${alpha}`,
      `oklch(${
        (state.modes.light.l +
          state.modes.light.colors.bravo.l +
          state.collections[
            state.modes.light.colors.bravo.collectionIndex
          ][1][0] *
            state.base.l.interval) %
        100
      }% ${
        ((state.modes.light.c * 10 +
          state.modes.light.colors.bravo.c +
          state.collections[
            state.modes.light.colors.bravo.collectionIndex
          ][1][1] *
            (state.base.c.interval * 10)) %
          5) /
        10
      } ${
        (state.modes.light.h +
          state.modes.light.colors.bravo.h +
          state.modes.light.colors.bravo.collectionShift) %
        360
      } / ${alpha})`
    )
    response += prop(
      `--border-foxtrot-${alpha}`,
      `1px solid var(--color-foxtrot-${alpha})`
    )

    //
  }

  return response.trim()
}

function lValues() {
  const tmp = []
  for (let l = 0; l < state.base.l.max; l += state.base.l.interval) {
    tmp.push(l)
  }
  return tmp
}

function modeProps(mode) {
  let response = ``

  // base for background
  response += prop(
    `--color-base`,
    `oklch(${state.modes[mode].l}% ${state.modes[mode].c} ${state.modes[mode].h})`
  )
  response += prop(`--border-base`, `1px solid var(--color-base)`)

  // alfa
  response += prop(
    `--color-alfa`,
    `oklch(${(state.modes[mode].l + state.modes[mode].colors.alfa.l) % 100}% ${
      ((state.modes[mode].c * 10 + state.modes[mode].colors.alfa.c) % 5) / 10
    } ${(state.modes[mode].h + state.modes[mode].colors.alfa.h) % 360})`
  )
  response += prop(`--border-alpha`, `1px solid var(--color-alpha)`)

  // bravo
  response += prop(
    `--color-bravo`,
    `oklch(${(state.modes[mode].l + state.modes[mode].colors.bravo.l) % 100}% ${
      ((state.modes[mode].c * 10 + state.modes[mode].colors.bravo.c) % 5) / 10
    } ${(state.modes[mode].h + state.modes[mode].colors.bravo.h) % 360})`
  )
  response += prop(`--border-bravo`, `1px solid var(--color-bravo)`)

  // charlie
  response += prop(
    `--color-charlie`,
    `oklch(${
      (state.modes[mode].l +
        state.modes[mode].colors.alfa.l +
        state.collections[state.modes[mode].colors.alfa.collectionIndex][0][0] *
          state.base.l.interval) %
      100
    }% ${
      ((state.modes[mode].c * 10 +
        state.modes[mode].colors.alfa.c +
        state.collections[state.modes[mode].colors.alfa.collectionIndex][0][1] *
          (state.base.c.interval * 10)) %
        5) /
      10
    } ${
      (state.modes[mode].h +
        state.modes[mode].colors.alfa.h +
        state.modes[mode].colors.alfa.collectionShift) %
      360
    })`
  )
  response += prop(`--border-charlie`, `1px solid var(--color-charlie)`)

  // delta
  response += prop(
    `--color-delta`,
    `oklch(${
      (state.modes[mode].l +
        state.modes[mode].colors.alfa.l +
        state.collections[state.modes[mode].colors.alfa.collectionIndex][1][0] *
          state.base.l.interval) %
      100
    }% ${
      ((state.modes[mode].c * 10 +
        state.modes[mode].colors.alfa.c +
        state.collections[state.modes[mode].colors.alfa.collectionIndex][1][1] *
          (state.base.c.interval * 10)) %
        5) /
      10
    } ${
      (state.modes[mode].h +
        state.modes[mode].colors.alfa.h +
        state.modes[mode].colors.alfa.collectionShift) %
      360
    })`
  )
  response += prop(`--border-delta`, `1px solid var(--color-delta)`)

  // echo
  response += prop(
    `--color-echo`,
    `oklch(${
      (state.modes[mode].l +
        state.modes[mode].colors.bravo.l +
        state.collections[
          state.modes[mode].colors.bravo.collectionIndex
        ][0][0] *
          state.base.l.interval) %
      100
    }% ${
      ((state.modes[mode].c * 10 +
        state.modes[mode].colors.bravo.c +
        state.collections[
          state.modes[mode].colors.bravo.collectionIndex
        ][0][1] *
          (state.base.c.interval * 10)) %
        5) /
      10
    } ${
      (state.modes[mode].h +
        state.modes[mode].colors.bravo.h +
        state.modes[mode].colors.bravo.collectionShift) %
      360
    })`
  )
  response += prop(`--border-echo`, `1px solid var(--color-echo)`)

  // foxtrot
  response += prop(
    `--color-foxtrot`,
    `oklch(${
      (state.modes[mode].l +
        state.modes[mode].colors.bravo.l +
        state.collections[
          state.modes[mode].colors.bravo.collectionIndex
        ][1][0] *
          state.base.l.interval) %
      100
    }% ${
      ((state.modes[mode].c * 10 +
        state.modes[mode].colors.bravo.c +
        state.collections[
          state.modes[mode].colors.bravo.collectionIndex
        ][1][1] *
          (state.base.c.interval * 10)) %
        5) /
      10
    } ${
      (state.modes[mode].h +
        state.modes[mode].colors.bravo.h +
        state.modes[mode].colors.bravo.collectionShift) %
      360
    })`
  )
  response += prop(`--border-foxtrot`, `1px solid var(--color-foxtrot)`)

  for (let alpha = 10; alpha <= 90; alpha = alpha + 10) {
    response += prop(
      `--color-base-${alpha}`,
      `oklch(${state.modes[mode].l}% ${state.modes[mode].c} ${state.modes[mode].h} / ${alpha}%)`
    )

    // alfa
    response += prop(
      `--color-alfa-${alpha}`,
      `oklch(${
        (state.modes[mode].l + state.modes[mode].colors.alfa.l) % 100
      }% ${
        ((state.modes[mode].c * 10 + state.modes[mode].colors.alfa.c) % 5) / 10
      } ${
        (state.modes[mode].h + state.modes[mode].colors.alfa.h) % 360
      } / ${alpha})`
    )
    response += prop(
      `--border-alfa-${alpha}`,
      `1px solid var(--color-alfa-${alpha})`
    )

    // bravo
    response += prop(
      `--color-bravo-${alpha}`,
      `oklch(${
        (state.modes[mode].l + state.modes[mode].colors.bravo.l) % 100
      }% ${
        ((state.modes[mode].c * 10 + state.modes[mode].colors.bravo.c) % 5) / 10
      } ${
        (state.modes[mode].h + state.modes[mode].colors.bravo.h) % 360
      }) / ${alpha}`
    )
    response += prop(
      `--border-bravo-${alpha}`,
      `1px solid var(--color-bravo-${alpha})`
    )

    // charlie
    response += prop(
      `--color-charlie-${alpha}`,
      `oklch(${
        (state.modes[mode].l +
          state.modes[mode].colors.alfa.l +
          state.collections[
            state.modes[mode].colors.alfa.collectionIndex
          ][0][0] *
            state.base.l.interval) %
        100
      }% ${
        ((state.modes[mode].c * 10 +
          state.modes[mode].colors.alfa.c +
          state.collections[
            state.modes[mode].colors.alfa.collectionIndex
          ][0][1] *
            (state.base.c.interval * 10)) %
          5) /
        10
      } ${
        (state.modes[mode].h +
          state.modes[mode].colors.alfa.h +
          state.modes[mode].colors.alfa.collectionShift) %
        360
      } / ${alpha})`
    )
    response += prop(
      `--border-charlie-${alpha}`,
      `1px solid var(--color-charlie-${alpha})`
    )

    // delta
    response += prop(
      `--color-delta-${alpha}`,
      `oklch(${
        (state.modes[mode].l +
          state.modes[mode].colors.alfa.l +
          state.collections[
            state.modes[mode].colors.alfa.collectionIndex
          ][1][0] *
            state.base.l.interval) %
        100
      }% ${
        ((state.modes[mode].c * 10 +
          state.modes[mode].colors.alfa.c +
          state.collections[
            state.modes[mode].colors.alfa.collectionIndex
          ][1][1] *
            (state.base.c.interval * 10)) %
          5) /
        10
      } ${
        (state.modes[mode].h +
          state.modes[mode].colors.alfa.h +
          state.modes[mode].colors.alfa.collectionShift) %
        360
      } / ${alpha})`
    )
    response += prop(
      `--border-delta-${alpha}`,
      `1px solid var(--color-delta-${alpha})`
    )

    // echo
    response += prop(
      `--color-echo-${alpha}`,
      `oklch(${
        (state.modes[mode].l +
          state.modes[mode].colors.bravo.l +
          state.collections[
            state.modes[mode].colors.bravo.collectionIndex
          ][0][0] *
            state.base.l.interval) %
        100
      }% ${
        ((state.modes[mode].c * 10 +
          state.modes[mode].colors.bravo.c +
          state.collections[
            state.modes[mode].colors.bravo.collectionIndex
          ][0][1] *
            (state.base.c.interval * 10)) %
          5) /
        10
      } ${
        (state.modes[mode].h +
          state.modes[mode].colors.bravo.h +
          state.modes[mode].colors.bravo.collectionShift) %
        360
      } / ${alpha})`
    )
    response += prop(
      `--border-echo-${alpha}`,
      `1px solid var(--color-echo-${alpha})`
    )

    // foxtrot
    response += prop(
      `--color-foxtrot-${alpha}`,
      `oklch(${
        (state.modes[mode].l +
          state.modes[mode].colors.bravo.l +
          state.collections[
            state.modes[mode].colors.bravo.collectionIndex
          ][1][0] *
            state.base.l.interval) %
        100
      }% ${
        ((state.modes[mode].c * 10 +
          state.modes[mode].colors.bravo.c +
          state.collections[
            state.modes[mode].colors.bravo.collectionIndex
          ][1][1] *
            (state.base.c.interval * 10)) %
          5) /
        10
      } ${
        (state.modes[mode].h +
          state.modes[mode].colors.bravo.h +
          state.modes[mode].colors.bravo.collectionShift) %
        360
      } / ${alpha})`
    )
    response += prop(
      `--border-foxtrot-${alpha}`,
      `1px solid var(--color-foxtrot-${alpha})`
    )

    //
  }

  return response.trim()
}

function modes() {
  const tmp = []
  for (let mode in state.modes) {
    tmp.push(mode)
  }
  return tmp
}

function openWindow() {
  const params = `scrollbars=no,resizable=yes,status=no,location=no,toolbar=no,menubar=no,width=800,top=20,left=20`
  if (childWindow && childWindow.name === childWindowName) {
    //sendStylesheet("Connection Already Established")
  } else {
    childWindow = window.open('/en/2fehqqas/', childWindowName, params)
    childWindow.addEventListener('load', () => {
      sendStylesheet()
    })
  }
  childWindow.focus()
}

function primaries() {
  return state.primaries
}

function primaryColors() {
  return [primaries()[0].key, primaries()[1].key]
}

function prop(key, value) {
  return `  ${key}: ${value};\n`
}

function propsCSS() {
  let response = ``
  response += style(`.color-base`, `color: var(--color-base);`)
  colors().forEach((color) => {
    response += style(`.color-${color}`, `color: var(--color-${color});`)
  })

  for (let alpha = 10; alpha <= 90; alpha = alpha + 10) {
    response += style(
      `.color-base-${alpha}`,
      `color: var(--color-base-${alpha});`
    )
    colors().forEach((color) => {
      response += style(
        `.color-${color}-${alpha}`,
        `color: var(--color-${color}-${alpha});`
      )
    })
  }

  response += style(`.bg-base`, `background-color: var(--color-base);`)
  colors().forEach((color) => {
    response += style(`.bg-${color}`, `background-color: var(--color-${color});`)
  })


  for (let alpha = 10; alpha <= 90; alpha = alpha + 10) {
    response += style(
      `.bg-base-${alpha}`,
      `background-color: var(--color-base-${alpha});`
    )
    colors().forEach((color) => {
      response += style(
        `.bg-${color}-${alpha}`,
        `background-color: var(--color-${color}-${alpha});`
      )
    })
  }


  return response
}

function style(key, value) {
  return `${key} { ${value} }\n`
}

function sendStylesheet(msg) {
  if (childWindow && childWindow.name === childWindowName) {
    childWindow.postMessage(stylePayload())
  } else {
    console.log('Window is not available')
  }
}

function stylePayload() {
  let payload = `
${baseFont()}
:root {
  ${baseProps()}
}
body {
  ${modeProps('light')}
}
body.dark {
  ${modeProps('dark')}
}
@media (prefers-color-scheme: dark) {
  body {
    ${modeProps('dark')}
  }
  body.light {
    ${modeProps('light')}
  }
}
${propsCSS()}
${baseCSS()}
  `
  return payload
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
      updateEl(`.chip-${l}-${cIndex} .chipTitle`, {
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
    let footerPayload = ''
    if (state.modes[state.active.mode].colors.alfa.h === h) {
      footerPayload += 'a '
    }
    if (state.modes[state.active.mode].colors.bravo.h === h) {
      footerPayload += 'b '
    }
    if (state.active.h === h) {
      footerPayload += '^ '
    }
    updateEl(`.primaryButtonFooter-${h}`, {
      innerHTML: footerPayload,
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
            (state.modes[state.active.mode].colors[primary.key].h + h) % 360
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
            `var(--color-${l2}-${c2}-${h2}-${state.active.mode})`
          )
        })
      })
    })
  })

  // Tertiary Chip Rectangles
  primaries().forEach((primary) => {
    collectionCoords().forEach((coords) => {
      const key = `tertiaryRect-${primary.key}-${coords[0]}-${coords[1]}`
      let h =
        (state.active.colors[primary.key].secondaryH +
          state.modes[state.active.mode].colors[primary.key].h) %
        360
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
  // lValues().forEach((l, lIndex) => {
  //   cValues().forEach((c, cIndex) => {
  //     updateProp(`--chip-${l}-${cIndex}`, `blue`)
  //   })
  // })

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

  updateEl(`.currentCSS`, {
    innerHTML: stylePayload(),
  })

  sendStylesheet()
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
  updateState()
  updateProps()
  addListenerTo('.preview-launcher', 'click', openWindow)
})
