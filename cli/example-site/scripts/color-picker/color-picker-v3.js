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
      this.loadInitialState()
      this.setupActiveStyles()
      this.buildWrapper()
      this.buildPreviewButton()
      this.buildModeButtons()
      this.buildSliders()
      this.buildPrimaryButtons()
      this.update()
    }

    buildModeButtons() {
      this.modAddTo(this.wrapper, 'div', {
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
      this.modAddTo(this.wrapper, 'button', {
        innerHTML: 'Launch Preview Window',
      })
    }

    buildPrimaryButton(parent, h) {
      const button = this.modAddTo(parent, 'div', {
        innerHTML: h,
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
            classes: ['primary-rect', `primary-rect-${l}-${cIndex}-${h}`],
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
      const primaryButtons = this.modAddTo(this.wrapper, 'div', {
        classes: ['primary-buttons'],
      })
      this.hValues().forEach((h) => {
        this.buildPrimaryButton(primaryButtons, h)
      })
    }

    buildSlider(config) {
      const sliders = this.modAddTo(this.wrapper, 'div', {
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

    buildWrapper() {
      this.wrapper = this.modAddTo(this.shadowRoot, 'div', {
        classes: ['picker-wrapper'],
      })
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
      this.state.active.h = this.modGetInt(event.target)
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

    loadInitialState() {
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

    lOffset(offset, mode) {
      let response = (this.state.modes[mode].l + offset) % this.state.base.l.max
    //   this.modLog(response)
      return `${response}%`
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

    setupActiveStyles() {
      const styles = this.ownerDocument.createElement('style')
      let sheet = ``
      this.lValues().forEach((l) => {
        this.cValues().forEach((c, cIndex) => {
          this.hValues().forEach((h) => {
            const key = `${l}-${cIndex}-${h}`
            sheet += `.primary-rect-${key} { fill: var(--color-${key}); }`
          })
        })
      })
      styles.innerHTML = sheet
      this.shadowRoot.appendChild(styles)
    }

    update() {
      this.lValues().forEach((l) => {
        this.cValues().forEach((c, cIndex) => {
          this.hValues().forEach((h) => {
            const key = `${l}-${cIndex}-${h}`
            // this.modLog(this.mode())
            // this.modLog(this.state.modes[this.mode()].l)
            // this.modLog(this.lOffset(this.state.modes[this.mode()].l, l))
            // this.modLog(this.lValue(this.mode()))

            const theL = this.lOffset(l, this.mode())
            // const theC = this.cOffset(c, this.mode())
           // const theC = this.cOffset(this.state.modes[this.mode()].c, c)
            this.devProps[`--color-${key}`] = `oklch(${theL} 0.2 200)`
          })
        })
      })
      for (let prop in this.devProps) {
        document.documentElement.style.setProperty(prop, this.devProps[prop])
      }
    }

    /////////////////////////////////////////////////////////////////////////////
    // Module functions

    modAddSvgTo(target, tag, attrs = {}) {
      const el = getEl(target)
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
