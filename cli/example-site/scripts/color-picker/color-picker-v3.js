customElements.define(
  'color-picker',
  class CodeBlock extends HTMLElement {
    constructor() {
      super()
      this.attachShadow({ mode: 'open' })
      this.loadInitialState()
      this.buildWrapper()
      this.buildModeButtons()
      this.buildSliders()
    }

    addTo(target, tag, attrs = {}) {
      const el = this.getEl(target)
      if (el) {
        const newEl = document.createElement(tag)
        updateAttrs(newEl, attrs)
        el.appendChild(newEl)
        return newEl
      }
    }

    buildModeButtons() {
      addTo(this.shadowRoot, 'div', {
        classes: ['modes'],
      })
      this.modes().forEach((mode) => {
        const label = this.addTo('.modes', 'label', {
          innerHTML: `<span>${mode}</span>`,
        })
        this.addTo(label, 'input', {
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

    buildSlider(config) {
      const sliders = this.addTo(this.shadowRoot, 'div', {
        classes: ['sliders'],
      })

      this.addTo(sliders, 'label', {
        for: `slider-${config.key}`,
        innerHTML: `<span>${config.label}</span>`,
      })

      this.addTo(sliders, 'input', {
        classes: ['slider', `slider-${config.key}`],
        name: `slider-${config.key}`,
        id: `slider-${config.key}`,
        type: 'range',
        min: config.min,
        max: config.max,
        step: config.step,
        data: [
            ['key', config.key]
        ],
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
      // addTo(`.sliders`, `button`, {
      //   classes: [`getFromButton-${config.key}`],
      //   innerHTML: `Get From: dark mode`,
      //   listeners: [
      //     [`click`, handleGetFromClick]
      //   ],
      //   data: [
      //     [`key`, config.key],
      //   ]
      // })
    }

    buildSliders() {
      this.buildSlider({
        key: 'l',
        label: 'Lightness',
        min: 0,
        max: state.base.l.max,
        step: state.base.l.step,
        value: state.modes.light.l,
      })

      this.buildSlider({
        key: 'c',
        label: 'Chroma',
        min: 0,
        max: state.base.c.max,
        step: state.base.c.step,
        value: state.modes.light.c,
      })

      this.buildSlider({
        key: 'h',
        label: 'Hue',
        min: 0,
        max: state.base.h.max,
        step: state.base.h.step,
        value: state.modes.light.h,
      })
    }

    buildWrapper() {
      addTo(this.shadowRoot, 'button', {
        innerHTML: 'Launch Preview Window',
      })
    }

    getEl(target) {
      if (typeof target === 'string') {
        const el = this.shadowRoot.querySelector(target)
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

    getValue(target) {
      const el = this.getEl(target)
      if (el) {
        return el.value
      } else {
        return undefined
      }
    }

    handleModeClick(event) {
      this.state.active.mode = this.getValue('[name="mode"]:checked')
      this.lch().forEach((key) => {
        this.setValue(`.slider-${key}`, this.state.modes[this.mode()][key])
        // this.setHTML(`.getFromButton-${key}`, `Get From: ${otherMode} mode`)
      })
      this.update()
    }

    handleSliderChange(event) {
      if (this.timeoutId === undefined) {
        this.timeoutId = null
      }
      void function () {
        window.clearTimeout(this.timeoutId)
        this.timeoutId = window.setTimeout(() => {
          this.update()
        }, 30)
      }.call(this)
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
    }

    logError(msg) {
      console.error(`${Date.now()} - ERROR: ` + msg)
    }

    logMsg(msg) {
      console.log(`${Date.now()} - INFO: ` + msg)
    }

    modGetFloat(target) {
      const el = this.getEl(target)
      if (el) {
        return parseFloat(el.value)
      } else {
        return undefined
      }
    }

    modLogObject(msg) {
      console.log(msg)
    }

    mode() {
      return this.state.active.mode
    }

    modes() {
      const tmp = []
      for (let mode in state.modes) {
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

    setValue(target, value) {
      const el = this.getEl(target)
      if (el) {
        el.value = value
      } else {
        this.logError(`Could not set value: ${value}`)
      }
    }

    update() {
      this.lch().forEach((key) => {
        this.state.modes[this.mode()][key] = this.modGetFloat(`.slider-${key}`)
        this.logMsg(this.state.modes[this.mode()][key])
      })
      this.logMsg('Doing update')
    }

    updateAttrs(target, attrs) {
      const el = getEl(target)
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
  }
)
