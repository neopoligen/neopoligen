customElements.define(
  'color-picker',
  class CodeBlock extends HTMLElement {
    constructor() {
      super()
      this.state = {}
      this.mainProps = {
        light: {},
        dark: {},
      }
      this.devProps = {}
      this.attachShadow({ mode: 'open' })
      this.setInitialState()
      this.setupActiveStyles()
      this.buildMainWrapper()
      this.buildPreviewButton()
      this.buildModeButtons()
      this.buildSliders()
      this.buildPrimaryWrapper()
      this.buildPrimaryButtons()
      this.buildPrimaryChips()
      this.buildSecondaryButtons()
      this.buildTertiaryButtons()
      this.update()
    }

    buildMainWrapper() {
      this.mainWrapper = this.modAddTo(this.shadowRoot, 'div', {
        classes: ['main-wrapper'],
      })
      this.modAddTo(this.mainWrapper, 'h2', {
        innerHTML: 'Color Picker',
      })
    }

    buildModeButtons() {
      this.modAddTo(this.mainWrapper, 'div', {
        classes: ['modes'],
      })
      this.modes().forEach((mode) => {
        const label = this.modAddTo('.modes', 'label', {
          innerHTML: `<span>${mode}</span>`,
        })
        this.modAddTo(label, 'input', {
          type: 'radio',
          name: 'mode',
          checked: mode === 'light' ? true : false,
          value: mode,
          listeners: [
            [
              'input',
              (event) => {
                this.handleModeClick.call(this, event)
              },
            ],
          ],
          // listeners: [['input', () => { console.log("HEREREREH") }]],
          classes: [`mode-${mode}`],
        })
      })
    }

    buildPreviewButton() {
      this.modAddTo(this.mainWrapper, 'button', {
        innerHTML: 'Launch Preview Window',
      })
    }

    buildPrimaryButton(parent, h) {
      const button = this.modAddTo(parent, 'div', {
        // innerHTML: h,
      })
      const svg = this.modAddSvgTo(button, 'svg', {
        width: 50,
        height: 50,
      })
      this.lValues().forEach((l, lIndex) => {
        this.cValues().forEach((c, cIndex) => {
          this.modAddSvgTo(svg, 'rect', {
            x: lIndex * 10,
            y: 40 - cIndex * 10,
            width: 10,
            height: 10,
            classes: ['primary-rect', `fill-${l}-${cIndex}-${h}`],
            data: [['h', h]],
            listeners: [
              [
                'click',
                (event) => this.handlePrimaryButtonClick.call(this, event),
              ],
            ],
          })
        })
      })
    }

    buildPrimaryButtons() {
      const primaryButtons = this.modAddTo(this.primaryWrapper, 'div', {
        classes: ['primary-buttons'],
      })
      this.hValues().forEach((h) => {
        this.buildPrimaryButton(primaryButtons, h)
      })
    }

    buildPrimaryChips() {
      const primaryChips = this.modAddTo(this.primaryWrapper, 'div', {
        classes: ['primary-chips'],
      })

      this.lValues().forEach((l, lIndex) => {
        const chipLine = this.modAddTo(primaryChips, 'div', {})
        this.cValues().forEach((c, cIndex) => {
          this.modAddTo(chipLine, 'div', {
            innerHTML: `
        <div class="chip chip-${l}-${cIndex}">
        <div class="x-chip-swatch"></div>
        <div class="chip-details">
          <div class="chip-title">#</div>
          <div class="chip-text">${this.state.sampleText}</div>
          <div class="chip-buttons-${l}-${cIndex}"></div>
        </div>
        </div>`,
          })
          this.primaryColors().forEach((color) => {
            this.modAddTo(`.chip-buttons-${l}-${cIndex}`, 'button', {
              classes: [`chipButton-${color}-${l}-${cIndex}`],
              innerHTML: color,
              data: [
                ['color', color],
                ['l', l],
                ['cIndex', cIndex],
              ],
              listeners: [
                [
                  'click',
                  (event) => this.handleColorButtonClick.call(this, event),
                ],
              ],
            })
          })
        })
      })
    }

    buildPrimaryWrapper() {
      this.primaryWrapper = this.modAddTo(this.mainWrapper, 'div', {
        classes: ['primary-wrapper'],
      })
    }

    buildSecondaryButtons() {
      this.primaries().forEach((primary) => {
        const mainKey = primary.secondaries.join('')
        const secondaryButtons = this.modAddTo(this.mainWrapper, 'div', {
          classes: ['secondary-buttons', `${mainKey}-chips`],
        })

        const key = primary.secondaries.join('')
        this.hValues().forEach((h, hIndex) => {
          let buttonWrapper = this.modAddTo(secondaryButtons, 'div', {
            innerHTML: `
              <div class="secondaryButtonHeader secondaryButtonHeader-${primary.key}-${h}"></div>
              <div class="secondaryButtonHolder secondaryButtonHolder-${primary.key}-${h}"></div>
              <div class="secondaryButtonFooter secondaryButtonFooter-${primary.key}-${h}"></div>
              `,
          })
          let btn = this.modAddSvgTo(
            `.secondaryButtonHolder-${primary.key}-${h}`,
            'svg',
            {
              classes: [
                `secondaryButton`,
                `secondaryButton-${primary.key}-${h}`,
              ],
              width: 30,
              height: 30,
            }
          )
          for (let coord1 = -1; coord1 <= 1; coord1++) {
            for (let coord2 = -1; coord2 <= 1; coord2++) {
              this.modAddSvgTo(btn, 'rect', {
                classes: [
                  `secondary-rect-coords-${primary.key}-${coord1}-${coord2}-${h}`,
                ],
                x: (coord1 + 1) * 10,
                y: (coord2 + 1) * 10,
                width: 10,
                height: 10,
                data: [
                  ['primary', primary.key],
                  ['secondaryH', hIndex * this.state.base.h.interval],
                ],
                listeners: [
                  [
                    'click',
                    (event) => {
                      this.handleSecondaryButtonClick.call(this, event)
                    },
                  ],
                ],
              })
            }
          }
        })
      })
    }

    buildSlider(config) {
      const sliders = this.modAddTo(this.mainWrapper, 'div', {
        classes: ['sliders'],
      })

      this.modAddTo(sliders, 'label', {
        for: `slider-${config.key}`,
        innerHTML: `<span>${config.label}</span>`,
      })

      this.modAddTo(sliders, 'input', {
        classes: ['slider', `slider-${config.key}`],
        name: `slider-${config.key}`,
        id: `slider-${config.key}`,
        type: 'range',
        min: config.min,
        max: config.max,
        step: config.step,
        data: [['key', config.key]],
        listeners: [
          [
            'input',
            (event) => {
              this.handleSliderChange.call(this, event)
            },
          ],
        ],
        value: config.value,
      })

      // original to review
      this.modAddTo(sliders, `button`, {
        classes: [`get-from-${config.key}`],
        innerHTML: `Copy ${this.state.modes.dark.display}`,
        listeners: [
          [
            `click`,
            (event) => {
              this.handleGetFromClick.call(this, event)
            },
          ],
        ],
        data: [[`key`, config.key]],
      })
    }

    buildSliders() {
      this.buildSlider({
        key: 'l',
        label: 'Lightness',
        min: 0,
        max: this.state.base.l.max,
        step: this.state.base.l.step,
        value: this.state.modes.light.l,
      })

      this.buildSlider({
        key: 'c',
        label: 'Chroma',
        min: 0,
        max: this.state.base.c.max,
        step: this.state.base.c.step,
        value: this.state.modes.light.c,
      })

      this.buildSlider({
        key: 'h',
        label: 'Hue',
        min: 0,
        max: this.state.base.h.max,
        step: this.state.base.h.step,
        value: this.state.modes.light.h,
      })
    }

    buildTertiaryButtons() {
      this.primaries().forEach((primary) => {
        this.collections().forEach((collection, collectionIndex) => {
          const mainKey = primary.secondaries.join('')
          const el = this.modAddSvgTo(`.${mainKey}-chips`, 'svg', {
            classes: [
              'tertiaryChip',
              `tertiaryChip-index-${primary.key}-${collectionIndex}`,
            ],
            width: 20,
            height: 40,
          })
          collection.forEach((coords, coordsIndex) => {
            const key = primary.key
            this.modAddSvgTo(el, 'rect', {
              classes: [
                'tertiaryRect',
                `tertiaryRect-${key}-${coords[0]}-${coords[1]}`,
              ],
              x: 0,
              y: coordsIndex * 20,
              width: 20,
              height: 20,
              data: [
                ['mode', this.state.active.mode],
                ['primary', primary.key],
                ['collectionIndex', collectionIndex],
              ],
              // listeners: [['click', handleTertiaryButtonClick]],
            })
          })
        })
      })
    }

    cOffset(offset, mode) {
      let response = (this.state.modes[mode].c + offset) % this.state.base.c.max
      return response
    }

    collectionCoords() {
      const refChecks = []
      const response = []
      this.state.collections.forEach((collection) => {
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

    collections() {
      return this.state.collections
    }

    colors() {
      return ['alfa', 'bravo', 'charlie', 'delta', 'echo', 'foxtrot']
    }

    cValue(mode) {
      return this.state.modes[mode].c
    }

    cValues() {
      const values = []
      for (
        let c = 0;
        c < this.state.base.c.max;
        c += this.state.base.c.interval
      ) {
        values.push(c)
      }
      return values
    }

    handleColorButtonClick(event) {
      this.state.modes[this.mode()].colors[event.target.dataset.color].l =
        parseInt(event.target.dataset.l, 10)
      this.state.modes[this.mode()].colors[event.target.dataset.color].c =
        parseInt(event.target.dataset.cIndex, 10)
      this.state.modes[this.mode()].colors[event.target.dataset.color].h =
        this.state.active.h
      this.update()

      // state.modes[state.active.mode].colors[event.target.dataset.color].c =
      //   parseInt(event.target.dataset.cIndex, 10)
      // state.modes[state.active.mode].colors[event.target.dataset.color].h =
      //   state.active.h
      // updateState()
      // updateProps()

      // console.log(event.target.dataset)
      // state.modes[state.active.mode].colors[event.target.dataset.color].l =
      //   parseInt(event.target.dataset.l, 10)
      // state.modes[state.active.mode].colors[event.target.dataset.color].c =
      //   parseInt(event.target.dataset.cIndex, 10)
      // state.modes[state.active.mode].colors[event.target.dataset.color].h =
      //   state.active.h
      // updateState()
      // updateProps()
    }

    handleGetFromClick(event) {
      const key = event.target.dataset.key
      this.state.modes[this.mode()][key] =
        this.state.modes[this.otherMode()][key]
      this.modSetValue(
        `.slider-${key}`,
        this.state.modes[this.otherMode()][key]
      )
      this.update()
    }

    handleModeClick(event) {
      this.state.active.mode = this.modGetValue('[name="mode"]:checked')
      this.lch().forEach((key) => {
        this.modSetValue(`.slider-${key}`, this.state.modes[this.mode()][key])
        this.modSetHTML(
          `.get-from-${key}`,
          `copy ${this.state.modes[this.otherMode()].display}`
        )
      })
      this.update()
    }

    handlePrimaryButtonClick(event) {
      this.state.active.h = this.modGetDataInt(event.target, 'h')
      this.update()
    }

    handleSecondaryButtonClick(event) {
      this.state.active.colors[event.target.dataset.primary].secondaryH =
        parseInt(event.target.dataset.secondaryH)
      this.update()
    }

    handleSliderChange(event) {
      if (this.timeoutId === undefined) {
        this.timeoutId = null
      }
      void function (key) {
        window.clearTimeout(this.timeoutId)
        this.timeoutId = window.setTimeout(() => {
          this.state.modes[this.mode()][key] = this.modGetFloat(
            `.slider-${key}`
          )
          this.update()
        }, 30)
      }.call(this, event.target.dataset.key)
    }

    handleTertiaryButtonClick(event) {
      this.state.modes[event.target.dataset.mode].colors[
        event.target.dataset.primary
      ].collectionIndex = parseInt(event.target.dataset.collectionIndex, 10)
      this.state.modes[event.target.dataset.mode].colors[
        event.target.dataset.primary
      ].collectionShift =
        this.state.active.colors[event.target.dataset.primary].secondaryH
      this.update()
    }

    hOffset(offset, mode) {
      let response = (this.state.modes[mode].h + offset) % this.state.base.h.max
      return response
    }

    hValue(mode) {
      return this.state.modes[mode].h
    }

    hValues() {
      const values = []
      for (
        let h = 0;
        h < this.state.base.h.max;
        h += this.state.base.h.interval
      ) {
        values.push(h)
      }
      return values
    }

    lch() {
      return [`l`, `c`, `h`]
    }

    lOffset(offset, mode) {
      let response = (this.state.modes[mode].l + offset) % this.state.base.l.max
      return response
    }

    lValue(mode) {
      return this.state.modes[mode].l
    }

    lValues() {
      const values = []
      for (
        let l = 0;
        l < this.state.base.l.max;
        l += this.state.base.l.interval
      ) {
        values.push(l)
      }
      return values
    }

    mode() {
      return this.state.active.mode
    }

    modes() {
      const tmp = []
      for (let mode in this.state.modes) {
        tmp.push(mode)
      }
      return tmp
    }

    otherMode() {
      if (this.mode() === 'light') {
        return 'dark'
      } else {
        return 'light'
      }
    }

    primaries() {
      return this.state.primaries
    }

    primaryColors() {
      return ['alfa', 'bravo']
    }

    setupActiveStyles() {
      const styles = this.ownerDocument.createElement('style')
      let sheet = `
.main-wrapper { 
    padding: 1rem;
    background-color: var(--dev-color-base); 
    border-radius: 0.6rem;
    color: var(--dev-color-bravo);
}

.primary-wrapper {
    display: flex;
    flex-wrap: wrap;
}

      `

      this.colors().forEach((color) => {
        const key = `color-${color}`
        sheet += `.${key} { color: var(--${key}); }`
      })

      // full loop
      this.lValues().forEach((l) => {
        this.cValues().forEach((c, cIndex) => {
          this.hValues().forEach((h) => {
            const key = `${l}-${cIndex}-${h}`
            sheet += `.fill-${key} { fill: var(--color-${key}); }`
          })
        })
      })

      // chip connected to current chip color
      this.lValues().forEach((l) => {
        this.cValues().forEach((c, cIndex) => {
          const key = `chip-${l}-${cIndex}`
          sheet += `.${key} { color: var(--${key}); }`
        })
      })

      // secondary button prep
      this.primaries().forEach((primary) => {
        this.hValues().forEach((h, hIndex) => {
          for (let coord1 = -1; coord1 <= 1; coord1++) {
            for (let coord2 = -1; coord2 <= 1; coord2++) {
              const key = `secondary-rect-coords-${primary.key}-${coord1}-${coord2}-${h}`
              sheet += `.${key} { fill: var(--color-${key}); }`
            }
          }
        })
      })

      // tertiary rect prep
      this.primaries().forEach((primary) => {
        this.collectionCoords().forEach((coords) => {
          const key = `tertiaryRect-${primary.key}-${coords[0]}-${coords[1]}`
          sheet += `.${key} { fill: var(--${key}); }`
        })
      })

      styles.innerHTML = sheet
      this.shadowRoot.appendChild(styles)
    }

    setInitialState() {
      this.state = {
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
            display: '☀',
            l: 70.762,
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
            display: '☾',
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
    }

    update() {
      // set the active base
      this.devProps[`--color-base-active`] = `oklch(${this.lValue(
        this.mode()
      )}% ${this.cValue(this.mode())} ${this.hValue(this.mode())})`

      // set the active explicit colors
      this.lValues().forEach((l) => {
        this.cValues().forEach((c, cIndex) => {
          this.hValues().forEach((h) => {
            const key = `${l}-${cIndex}-${h}`
            // this.modLog(this.lValue(this.mode()))
            const theL = this.lOffset(l, this.mode())
            const theC = this.cOffset(c, this.mode())
            const theH = this.state.modes[this.mode()].h + h
            this.devProps[`--color-${key}`] = `oklch(${theL}% ${theC} ${theH})`
          })
        })
      })

      // set the active chip colors for the current hue
      this.lValues().forEach((l) => {
        this.cValues().forEach((c, cIndex) => {
          const key = `${l}-${cIndex}`
          const theL = this.lOffset(l, this.mode())
          const theC = this.cOffset(c, this.mode())
          const theH = this.state.active.h + this.state.modes[this.mode()].h
          this.devProps[`--chip-${key}`] = `oklch(${theL}% ${theC} ${theH})`
        })
      })

      // secondary rects
      this.primaries().forEach((primary, primaryIndex) => {
        this.hValues().forEach((h) => {
          this.collections().forEach((collection) => {
            collection.forEach((coords) => {
              const key = `color-secondary-rect-coords-${primary.key}-${coords[0]}-${coords[1]}-${h}`
              let h2 =
                (this.state.modes[this.mode()].colors[primary.key].h + h) % 360
              let l2 =
                (this.state.modes[this.mode()].colors[primary.key].l +
                  100 +
                  20 * coords[0]) %
                100
              let c2 =
                (this.state.modes[this.mode()].colors[primary.key].c +
                  5 +
                  coords[1]) %
                5
              this.devProps[`--${key}`] = `var(--color-${l2}-${c2}-${h2})`
            })
          })
        })
      })

      // tertiaries
      this.primaries().forEach((primary) => {
        this.collectionCoords().forEach((coords) => {
          const key = `tertiaryRect-${primary.key}-${coords[0]}-${coords[1]}`
          let h =
            (this.state.active.colors[primary.key].secondaryH +
              this.state.modes[this.mode()].colors[primary.key].h) %
            360
          let l =
            (this.state.modes[this.mode()].colors[primary.key].l +
              100 +
              20 * coords[0]) %
            100
          let c =
            (this.state.modes[this.mode()].colors[primary.key].c +
              5 +
              coords[1]) %
            5
          this.devProps[`--${key}`] = `var(--color-${l}-${c}-${h})`
        })
      })

      // update all the dev props
      for (let prop in this.devProps) {
        this.ownerDocument.documentElement.style.setProperty(
          prop,
          this.devProps[prop]
        )
      }
    }

    /////////////////////////////////////////////////////////////////////////////
    // Module functions

    modAddSvgTo(target, tag, attrs = {}) {
      const el = this.modGetEl(target)
      if (el) {
        const svg = this.ownerDocument.createElementNS(
          'http://www.w3.org/2000/svg',
          tag
        )
        this.modUpdateSvgAttrs(svg, attrs)
        el.appendChild(svg)
        return svg
      }
    }

    modAddTo(target, tag, attrs = {}) {
      const el = this.modGetEl(target)
      if (el) {
        const newEl = this.ownerDocument.createElement(tag)
        this.modUpdateAttrs(newEl, attrs)
        el.appendChild(newEl)
        return newEl
      }
    }

    modGetData(target, key) {
      const el = this.modGetEl(target)
      if (el) {
        return el.dataset[key]
      } else {
        return undefined
      }
    }

    modGetDataInt(target, key) {
      const el = this.modGetEl(target)
      if (el) {
        return parseInt(el.dataset[key], 10)
      } else {
        return undefined
      }
    }

    modGetEl(target) {
      if (typeof target === 'string') {
        const el = this.shadowRoot.querySelector(target)
        if (el) {
          return el
        } else {
          this.modLogError(`Could not find querySelector for: ${target}`)
          return undefined
        }
      } else if (target) {
        return target
      } else {
        this.modLogError(`Could not get element: ${target}`)
        return undefined
      }
    }

    modGetFloat(target) {
      const el = this.modGetEl(target)
      if (el) {
        return parseFloat(el.value)
      } else {
        return undefined
      }
    }

    modGetInt(target) {
      const el = this.modGetEl(target)
      if (el) {
        return parseInt(el.value, 10)
      } else {
        return undefined
      }
    }

    modGetValue(target) {
      const el = this.modGetEl(target)
      if (el) {
        return el.value
      } else {
        return undefined
      }
    }

    modLogError(msg) {
      console.error(`${Date.now()} - ERROR: ` + msg)
    }

    modLog(msg) {
      console.log(`${Date.now()} - INFO: ` + msg)
    }

    modLogObject(msg) {
      console.log(msg)
    }

    modSetHTML(target, value) {
      this.modUpdateAttrs(target, {
        innerHTML: value,
      })
    }

    modSetValue(target, value) {
      const el = this.modGetEl(target)
      if (el) {
        el.value = value
      } else {
        this.modLogError(`Could not set value: ${value}`)
      }
    }

    modUpdateAttrs(target, attrs) {
      const el = this.modGetEl(target)
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
          el.addEventListener(
            attrs.listeners[index][0],
            attrs.listeners[index][1]
          )
        }
        return el
      }
    }

    modUpdateSvgAttrs(target, attrs) {
      const el = this.modGetEl(target)
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
          el.addEventListener(
            attrs.listeners[index][0],
            attrs.listeners[index][1]
          )
        }
        for (let index in attrs.styles) {
          el.style[attrs.styles[index][0]] = attrs.styles[index][1]
        }
        return el
      }
    }

    //
  }
)
