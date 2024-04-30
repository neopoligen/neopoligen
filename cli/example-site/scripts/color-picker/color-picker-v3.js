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
      this.childWindow
      this.childWindowName = 'previewWindow'
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
        listeners: [
          ['click', (event) => this.handlePreviewButtonClick.call(this, event)],
        ],
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

      this.lValues()
        .reverse()
        .forEach((l, lIndex) => {
          const chipLine = this.modAddTo(primaryChips, 'div', {
            classes: ['primary-chip-row'],
          })
          this.cValues().forEach((c, cIndex) => {
            this.modAddTo(chipLine, 'div', {
              innerHTML: `
        <div class="primary-chip chip-${l}-${cIndex}">
        <div class="x-chip-swatch"></div>
        <div class="chip-details">
          <div class="chip-title-${l}-${cIndex}">#</div>
          <div class="chip-text">${this.state.sampleText}</div>
          <div class="chip-buttons chip-buttons-${l}-${cIndex}"></div>
        </div>
        </div>`,
            })
            this.primaryColors().forEach((color) => {
              this.modAddTo(`.chip-buttons-${l}-${cIndex}`, 'button', {
                classes: [
                  `chip-button`,
                  `chip-button-${color}`,
                  `chipButton-${color}-${l}-${cIndex}`,
                ],
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
              listeners: [
                [
                  'click',
                  (event) => {
                    this.handleTertiaryButtonClick.call(this, event)
                  },
                ],
              ],
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

    genBaseStyles() {
      return ` .color-base { color: var(--color-base); }
        .color-alfa { color: var(--color-alfa); }
        .color-bravo { color: var(--color-bravo); }
        .color-charlie { color: var(--color-charlie); }
        .color-delta { color: var(--color-delta); }
        .color-echo { color: var(--color-echo); }
        .color-foxtrot { color: var(--color-foxtrot); }
        .color-base-10 { color: var(--color-base-10); }
        .color-alfa-10 { color: var(--color-alfa-10); }
        .color-bravo-10 { color: var(--color-bravo-10); }
        .color-charlie-10 { color: var(--color-charlie-10); }
        .color-delta-10 { color: var(--color-delta-10); }
        .color-echo-10 { color: var(--color-echo-10); }
        .color-foxtrot-10 { color: var(--color-foxtrot-10); }
        .color-base-20 { color: var(--color-base-20); }
        .color-alfa-20 { color: var(--color-alfa-20); }
        .color-bravo-20 { color: var(--color-bravo-20); }
        .color-charlie-20 { color: var(--color-charlie-20); }
        .color-delta-20 { color: var(--color-delta-20); }
        .color-echo-20 { color: var(--color-echo-20); }
        .color-foxtrot-20 { color: var(--color-foxtrot-20); }
        .color-base-30 { color: var(--color-base-30); }
        .color-alfa-30 { color: var(--color-alfa-30); }
        .color-bravo-30 { color: var(--color-bravo-30); }
        .color-charlie-30 { color: var(--color-charlie-30); }
        .color-delta-30 { color: var(--color-delta-30); }
        .color-echo-30 { color: var(--color-echo-30); }
        .color-foxtrot-30 { color: var(--color-foxtrot-30); }
        .color-base-40 { color: var(--color-base-40); }
        .color-alfa-40 { color: var(--color-alfa-40); }
        .color-bravo-40 { color: var(--color-bravo-40); }
        .color-charlie-40 { color: var(--color-charlie-40); }
        .color-delta-40 { color: var(--color-delta-40); }
        .color-echo-40 { color: var(--color-echo-40); }
        .color-foxtrot-40 { color: var(--color-foxtrot-40); }
        .color-base-50 { color: var(--color-base-50); }
        .color-alfa-50 { color: var(--color-alfa-50); }
        .color-bravo-50 { color: var(--color-bravo-50); }
        .color-charlie-50 { color: var(--color-charlie-50); }
        .color-delta-50 { color: var(--color-delta-50); }
        .color-echo-50 { color: var(--color-echo-50); }
        .color-foxtrot-50 { color: var(--color-foxtrot-50); }
        .color-base-60 { color: var(--color-base-60); }
        .color-alfa-60 { color: var(--color-alfa-60); }
        .color-bravo-60 { color: var(--color-bravo-60); }
        .color-charlie-60 { color: var(--color-charlie-60); }
        .color-delta-60 { color: var(--color-delta-60); }
        .color-echo-60 { color: var(--color-echo-60); }
        .color-foxtrot-60 { color: var(--color-foxtrot-60); }
        .color-base-70 { color: var(--color-base-70); }
        .color-alfa-70 { color: var(--color-alfa-70); }
        .color-bravo-70 { color: var(--color-bravo-70); }
        .color-charlie-70 { color: var(--color-charlie-70); }
        .color-delta-70 { color: var(--color-delta-70); }
        .color-echo-70 { color: var(--color-echo-70); }
        .color-foxtrot-70 { color: var(--color-foxtrot-70); }
        .color-base-80 { color: var(--color-base-80); }
        .color-alfa-80 { color: var(--color-alfa-80); }
        .color-bravo-80 { color: var(--color-bravo-80); }
        .color-charlie-80 { color: var(--color-charlie-80); }
        .color-delta-80 { color: var(--color-delta-80); }
        .color-echo-80 { color: var(--color-echo-80); }
        .color-foxtrot-80 { color: var(--color-foxtrot-80); }
        .color-base-90 { color: var(--color-base-90); }
        .color-alfa-90 { color: var(--color-alfa-90); }
        .color-bravo-90 { color: var(--color-bravo-90); }
        .color-charlie-90 { color: var(--color-charlie-90); }
        .color-delta-90 { color: var(--color-delta-90); }
        .color-echo-90 { color: var(--color-echo-90); }
        .color-foxtrot-90 { color: var(--color-foxtrot-90); }
        .bg-base { background-color: var(--color-base); }
        .bg-alfa { background-color: var(--color-alfa); }
        .bg-bravo { background-color: var(--color-bravo); }
        .bg-charlie { background-color: var(--color-charlie); }
        .bg-delta { background-color: var(--color-delta); }
        .bg-echo { background-color: var(--color-echo); }
        .bg-foxtrot { background-color: var(--color-foxtrot); }
        .bg-base-10 { background-color: var(--color-base-10); }
        .bg-alfa-10 { background-color: var(--color-alfa-10); }
        .bg-bravo-10 { background-color: var(--color-bravo-10); }
        .bg-charlie-10 { background-color: var(--color-charlie-10); }
        .bg-delta-10 { background-color: var(--color-delta-10); }
        .bg-echo-10 { background-color: var(--color-echo-10); }
        .bg-foxtrot-10 { background-color: var(--color-foxtrot-10); }
        .bg-base-20 { background-color: var(--color-base-20); }
        .bg-alfa-20 { background-color: var(--color-alfa-20); }
        .bg-bravo-20 { background-color: var(--color-bravo-20); }
        .bg-charlie-20 { background-color: var(--color-charlie-20); }
        .bg-delta-20 { background-color: var(--color-delta-20); }
        .bg-echo-20 { background-color: var(--color-echo-20); }
        .bg-foxtrot-20 { background-color: var(--color-foxtrot-20); }
        .bg-base-30 { background-color: var(--color-base-30); }
        .bg-alfa-30 { background-color: var(--color-alfa-30); }
        .bg-bravo-30 { background-color: var(--color-bravo-30); }
        .bg-charlie-30 { background-color: var(--color-charlie-30); }
        .bg-delta-30 { background-color: var(--color-delta-30); }
        .bg-echo-30 { background-color: var(--color-echo-30); }
        .bg-foxtrot-30 { background-color: var(--color-foxtrot-30); }
        .bg-base-40 { background-color: var(--color-base-40); }
        .bg-alfa-40 { background-color: var(--color-alfa-40); }
        .bg-bravo-40 { background-color: var(--color-bravo-40); }
        .bg-charlie-40 { background-color: var(--color-charlie-40); }
        .bg-delta-40 { background-color: var(--color-delta-40); }
        .bg-echo-40 { background-color: var(--color-echo-40); }
        .bg-foxtrot-40 { background-color: var(--color-foxtrot-40); }
        .bg-base-50 { background-color: var(--color-base-50); }
        .bg-alfa-50 { background-color: var(--color-alfa-50); }
        .bg-bravo-50 { background-color: var(--color-bravo-50); }
        .bg-charlie-50 { background-color: var(--color-charlie-50); }
        .bg-delta-50 { background-color: var(--color-delta-50); }
        .bg-echo-50 { background-color: var(--color-echo-50); }
        .bg-foxtrot-50 { background-color: var(--color-foxtrot-50); }
        .bg-base-60 { background-color: var(--color-base-60); }
        .bg-alfa-60 { background-color: var(--color-alfa-60); }
        .bg-bravo-60 { background-color: var(--color-bravo-60); }
        .bg-charlie-60 { background-color: var(--color-charlie-60); }
        .bg-delta-60 { background-color: var(--color-delta-60); }
        .bg-echo-60 { background-color: var(--color-echo-60); }
        .bg-foxtrot-60 { background-color: var(--color-foxtrot-60); }
        .bg-base-70 { background-color: var(--color-base-70); }
        .bg-alfa-70 { background-color: var(--color-alfa-70); }
        .bg-bravo-70 { background-color: var(--color-bravo-70); }
        .bg-charlie-70 { background-color: var(--color-charlie-70); }
        .bg-delta-70 { background-color: var(--color-delta-70); }
        .bg-echo-70 { background-color: var(--color-echo-70); }
        .bg-foxtrot-70 { background-color: var(--color-foxtrot-70); }
        .bg-base-80 { background-color: var(--color-base-80); }
        .bg-alfa-80 { background-color: var(--color-alfa-80); }
        .bg-bravo-80 { background-color: var(--color-bravo-80); }
        .bg-charlie-80 { background-color: var(--color-charlie-80); }
        .bg-delta-80 { background-color: var(--color-delta-80); }
        .bg-echo-80 { background-color: var(--color-echo-80); }
        .bg-foxtrot-80 { background-color: var(--color-foxtrot-80); }
        .bg-base-90 { background-color: var(--color-base-90); }
        .bg-alfa-90 { background-color: var(--color-alfa-90); }
        .bg-bravo-90 { background-color: var(--color-bravo-90); }
        .bg-charlie-90 { background-color: var(--color-charlie-90); }
        .bg-delta-90 { background-color: var(--color-delta-90); }
        .bg-echo-90 { background-color: var(--color-echo-90); }
        .bg-foxtrot-90 { background-color: var(--color-foxtrot-90); }`
    }



    prop(key, value) {
      return `${key}: ${value};\n`
    }

    genStyles(mode) {
      let response = ``

      // base for background
      response += this.prop(
        `--color-base`,
        `oklch(${this.state.modes[mode].l}% ${this.state.modes[mode].c} ${this.state.modes[mode].h})`
      )
    //   response += this.prop(`--border-base`, `1px solid var(--color-base)`)

      // alfa
      response += this.prop(
        `--color-alfa`,
        `oklch(${
          (this.state.modes[mode].l + this.state.modes[mode].colors.alfa.l) %
          100
        }% ${
          ((this.state.modes[mode].c * 10 +
            this.state.modes[mode].colors.alfa.c) %
            5) /
          10
        } ${
          (this.state.modes[mode].h + this.state.modes[mode].colors.alfa.h) %
          360
        })`
      )
    //   response += this.prop(`--border-alpha`, `1px solid var(--color-alpha)`)

      // bravo
      response += this.prop(
        `--color-bravo`,
        `oklch(${
          (this.state.modes[mode].l + this.state.modes[mode].colors.bravo.l) %
          100
        }% ${
          ((this.state.modes[mode].c * 10 +
            this.state.modes[mode].colors.bravo.c) %
            5) /
          10
        } ${
          (this.state.modes[mode].h + this.state.modes[mode].colors.bravo.h) %
          360
        })`
      )
    //   response += this.prop(`--border-bravo`, `1px solid var(--color-bravo)`)

      // charlie
      response += this.prop(
        `--color-charlie`,
        `oklch(${
          (this.state.modes[mode].l +
            this.state.modes[mode].colors.alfa.l +
            this.state.collections[
              this.state.modes[mode].colors.alfa.collectionIndex
            ][0][0] *
              this.state.base.l.interval) %
          100
        }% ${
          ((this.state.modes[mode].c * 10 +
            this.state.modes[mode].colors.alfa.c +
            this.state.collections[
              this.state.modes[mode].colors.alfa.collectionIndex
            ][0][1] *
              (this.state.base.c.interval * 10)) %
            5) /
          10
        } ${
          (this.state.modes[mode].h +
            this.state.modes[mode].colors.alfa.h +
            this.state.modes[mode].colors.alfa.collectionShift) %
          360
        })`
      )
    //   response += this.prop(
    //     `--border-charlie`,
    //     `1px solid var(--color-charlie)`
    //   )

      // delta
      response += this.prop(
        `--color-delta`,
        `oklch(${
          (this.state.modes[mode].l +
            this.state.modes[mode].colors.alfa.l +
            this.state.collections[
              this.state.modes[mode].colors.alfa.collectionIndex
            ][1][0] *
              this.state.base.l.interval) %
          100
        }% ${
          ((this.state.modes[mode].c * 10 +
            this.state.modes[mode].colors.alfa.c +
            this.state.collections[
              this.state.modes[mode].colors.alfa.collectionIndex
            ][1][1] *
              (this.state.base.c.interval * 10)) %
            5) /
          10
        } ${
          (this.state.modes[mode].h +
            this.state.modes[mode].colors.alfa.h +
            this.state.modes[mode].colors.alfa.collectionShift) %
          360
        })`
      )
    //   response += this.prop(`--border-delta`, `1px solid var(--color-delta)`)

      // echo

      //   let lEcho =
      //     (this.state.modes[mode].l +
      //       this.state.modes[mode].colors.bravo.l +
      //       this.state.collections[
      //         this.state.modes[mode].colors.bravo.collectionIndex
      //       ][0][0] *
      //         this.state.base.l.interval) %
      //     100
      //   this.modLog(lEcho)

      //   let cEcho =
      //     ((this.state.modes[mode].c * 10 +
      //       this.state.modes[mode].colors.bravo.c +
      //       this.state.collections[
      //         this.state.modes[mode].colors.bravo.collectionIndex
      //       ][0][1] *
      //         (this.state.base.c.interval * 10)) %
      //       5) /
      //     10
      //   this.modLog(cEcho)

      // let hEcho = (this.state.modes[mode].h +
      //     this.state.modes[mode].colors.bravo.h +
      //     this.state.modes[mode].colors.bravo.collectionShift) %
      //   360
      // this.modLog(hEcho)

      response += this.prop(
        `--color-echo`,
        `oklch(${
          (this.state.modes[mode].l +
            this.state.modes[mode].colors.bravo.l +
            this.state.collections[
              this.state.modes[mode].colors.bravo.collectionIndex
            ][0][0] *
              this.state.base.l.interval) %
          100
        }% ${
          ((this.state.modes[mode].c * 10 +
            this.state.modes[mode].colors.bravo.c +
            this.state.collections[
              this.state.modes[mode].colors.bravo.collectionIndex
            ][0][1] *
              (this.state.base.c.interval * 10)) %
            5) /
          10
        } ${
          (this.state.modes[mode].h +
            this.state.modes[mode].colors.bravo.h +
            this.state.modes[mode].colors.bravo.collectionShift) %
          360
        })`
      )
    //   response += this.prop(`--border-echo`, `1px solid var(--color-echo)`)

      // foxtrot
      response += this.prop(
        `--color-foxtrot`,
        `oklch(${
          (this.state.modes[mode].l +
            this.state.modes[mode].colors.bravo.l +
            this.state.collections[
              this.state.modes[mode].colors.bravo.collectionIndex
            ][1][0] *
              this.state.base.l.interval) %
          100
        }% ${
          ((this.state.modes[mode].c * 10 +
            this.state.modes[mode].colors.bravo.c +
            this.state.collections[
              this.state.modes[mode].colors.bravo.collectionIndex
            ][1][1] *
              (this.state.base.c.interval * 10)) %
            5) /
          10
        } ${
          (this.state.modes[mode].h +
            this.state.modes[mode].colors.bravo.h +
            this.state.modes[mode].colors.bravo.collectionShift) %
          360
        })`
      )
    //   response += this.prop(
    //     `--border-foxtrot`,
    //     `1px solid var(--color-foxtrot)`
    //   )

      for (let alpha = 10; alpha <= 90; alpha = alpha + 10) {
        response += this.prop(
          `--color-base-${alpha}`,
          `oklch(${this.state.modes[mode].l}% ${this.state.modes[mode].c} ${this.state.modes[mode].h} / ${alpha}%)`
        )

        // alfa
        response += this.prop(
          `--color-alfa-${alpha}`,
          `oklch(${
            (this.state.modes[mode].l + this.state.modes[mode].colors.alfa.l) %
            100
          }% ${
            ((this.state.modes[mode].c * 10 +
              this.state.modes[mode].colors.alfa.c) %
              5) /
            10
          } ${
            (this.state.modes[mode].h + this.state.modes[mode].colors.alfa.h) %
            360
          } / ${alpha})`
        )
        // response += this.prop(
        //   `--border-alfa-${alpha}`,
        //   `1px solid var(--color-alfa-${alpha})`
        // )

        // bravo
        response += this.prop(
          `--color-bravo-${alpha}`,
          `oklch(${
            (this.state.modes[mode].l + this.state.modes[mode].colors.bravo.l) %
            100
          }% ${
            ((this.state.modes[mode].c * 10 +
              this.state.modes[mode].colors.bravo.c) %
              5) /
            10
          } ${
            (this.state.modes[mode].h + this.state.modes[mode].colors.bravo.h) %
            360
          }) / ${alpha}`
        )
        // response += this.prop(
        //   `--border-bravo-${alpha}`,
        //   `1px solid var(--color-bravo-${alpha})`
        // )

        // charlie
        response += this.prop(
          `--color-charlie-${alpha}`,
          `oklch(${
            (this.state.modes[mode].l +
              this.state.modes[mode].colors.alfa.l +
              this.state.collections[
                this.state.modes[mode].colors.alfa.collectionIndex
              ][0][0] *
                this.state.base.l.interval) %
            100
          }% ${
            ((this.state.modes[mode].c * 10 +
              this.state.modes[mode].colors.alfa.c +
              this.state.collections[
                this.state.modes[mode].colors.alfa.collectionIndex
              ][0][1] *
                (this.state.base.c.interval * 10)) %
              5) /
            10
          } ${
            (this.state.modes[mode].h +
              this.state.modes[mode].colors.alfa.h +
              this.state.modes[mode].colors.alfa.collectionShift) %
            360
          } / ${alpha})`
        )
        // response += this.prop(
        //   `--border-charlie-${alpha}`,
        //   `1px solid var(--color-charlie-${alpha})`
        // )

        // delta
        response += this.prop(
          `--color-delta-${alpha}`,
          `oklch(${
            (this.state.modes[mode].l +
              this.state.modes[mode].colors.alfa.l +
              this.state.collections[
                this.state.modes[mode].colors.alfa.collectionIndex
              ][1][0] *
                this.state.base.l.interval) %
            100
          }% ${
            ((this.state.modes[mode].c * 10 +
              this.state.modes[mode].colors.alfa.c +
              this.state.collections[
                this.state.modes[mode].colors.alfa.collectionIndex
              ][1][1] *
                (this.state.base.c.interval * 10)) %
              5) /
            10
          } ${
            (this.state.modes[mode].h +
              this.state.modes[mode].colors.alfa.h +
              this.state.modes[mode].colors.alfa.collectionShift) %
            360
          } / ${alpha})`
        )
        // response += this.prop(
        //   `--border-delta-${alpha}`,
        //   `1px solid var(--color-delta-${alpha})`
        // )

        // echo
        response += this.prop(
          `--color-echo-${alpha}`,
          `oklch(${
            (this.state.modes[mode].l +
              this.state.modes[mode].colors.bravo.l +
              this.state.collections[
                this.state.modes[mode].colors.bravo.collectionIndex
              ][0][0] *
                this.state.base.l.interval) %
            100
          }% ${
            ((this.state.modes[mode].c * 10 +
              this.state.modes[mode].colors.bravo.c +
              this.state.collections[
                this.state.modes[mode].colors.bravo.collectionIndex
              ][0][1] *
                (this.state.base.c.interval * 10)) %
              5) /
            10
          } ${
            (this.state.modes[mode].h +
              this.state.modes[mode].colors.bravo.h +
              this.state.modes[mode].colors.bravo.collectionShift) %
            360
          } / ${alpha})`
        )
        // response += this.prop(
        //   `--border-echo-${alpha}`,
        //   `1px solid var(--color-echo-${alpha})`
        // )

        // foxtrot
        response += this.prop(
          `--color-foxtrot-${alpha}`,
          `oklch(${
            (this.state.modes[mode].l +
              this.state.modes[mode].colors.bravo.l +
              this.state.collections[
                this.state.modes[mode].colors.bravo.collectionIndex
              ][1][0] *
                this.state.base.l.interval) %
            100
          }% ${
            ((this.state.modes[mode].c * 10 +
              this.state.modes[mode].colors.bravo.c +
              this.state.collections[
                this.state.modes[mode].colors.bravo.collectionIndex
              ][1][1] *
                (this.state.base.c.interval * 10)) %
              5) /
            10
          } ${
            (this.state.modes[mode].h +
              this.state.modes[mode].colors.bravo.h +
              this.state.modes[mode].colors.bravo.collectionShift) %
            360
          } / ${alpha})`
        )
        // response += this.prop(
        //   `--border-foxtrot-${alpha}`,
        //   `1px solid var(--color-foxtrot-${alpha})`
        // )

        //
      }

      return response.trim()
    }

    handleColorButtonClick(event) {
      this.state.modes[this.mode()].colors[event.target.dataset.color].l =
        parseInt(event.target.dataset.l, 10)
      this.state.modes[this.mode()].colors[event.target.dataset.color].c =
        parseInt(event.target.dataset.cIndex, 10)
      this.state.modes[this.mode()].colors[event.target.dataset.color].h =
        this.state.active.h
      this.update()

      // this.state.modes[this.state.active.mode].colors[event.target.dataset.color].c =
      //   parseInt(event.target.dataset.cIndex, 10)
      // this.state.modes[this.state.active.mode].colors[event.target.dataset.color].h =
      //   this.state.active.h
      // updateState()
      // updateProps()

      // console.log(event.target.dataset)
      // this.state.modes[this.state.active.mode].colors[event.target.dataset.color].l =
      //   parseInt(event.target.dataset.l, 10)
      // this.state.modes[this.state.active.mode].colors[event.target.dataset.color].c =
      //   parseInt(event.target.dataset.cIndex, 10)
      // this.state.modes[this.state.active.mode].colors[event.target.dataset.color].h =
      //   this.state.active.h
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

    handlePreviewButtonClick(event) {
      const params = `scrollbars=no,resizable=yes,status=no,location=no,toolbar=no,menubar=no,width=800,top=20,left=20`
      if (this.childWindow && this.childWindow.name === this.childWindowName) {
        //sendStylesheet("Connection Already Established")
      } else {
        this.childWindow = window.open(
          '/en/2fehqqas/',
          this.childWindowName,
          params
        )
        this.childWindow.addEventListener('load', () => {
          this.sendStylesheet()
        })
      }
      this.childWindow.focus()
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

    sendStylesheet() {
      let styles = `
:root {
    --size-1: 2.986rem;
    --size-2: 2.488rem;
    --size-3: 2.074rem;
    --size-4: 1.728rem;
    --size-5: 1.44rem;
    --size-6: 1.2rem;
    --size-7: 1rem;
    --size-8: 0.833rem;
    --size-9: 0.694rem;
    --size-10: 0.579rem;
    --color-black: rgb(0 0 0);
    --border-black: 1px solid var(--color-black);
    --color-white: rgb(255 255 255);
    --border-white: 1px solid var(--color-white);
    --color-black-10: rgb(0 0 0 / 10%);
    --border-black-10: 1px solid var(--color-black-10);
    --color-white-10: rgb(255 255 255 / 10%);
    --border-white-10: 1px solid var(--color-white-10);
    --color-black-20: rgb(0 0 0 / 20%);
    --border-black-20: 1px solid var(--color-black-20);
    --color-white-20: rgb(255 255 255 / 20%);
    --border-white-20: 1px solid var(--color-white-20);
    --color-black-30: rgb(0 0 0 / 30%);
    --border-black-30: 1px solid var(--color-black-30);
    --color-white-30: rgb(255 255 255 / 30%);
    --border-white-30: 1px solid var(--color-white-30);
    --color-black-40: rgb(0 0 0 / 40%);
    --border-black-40: 1px solid var(--color-black-40);
    --color-white-40: rgb(255 255 255 / 40%);
    --border-white-40: 1px solid var(--color-white-40);
    --color-black-50: rgb(0 0 0 / 50%);
    --border-black-50: 1px solid var(--color-black-50);
    --color-white-50: rgb(255 255 255 / 50%);
    --border-white-50: 1px solid var(--color-white-50);
    --color-black-60: rgb(0 0 0 / 60%);
    --border-black-60: 1px solid var(--color-black-60);
    --color-white-60: rgb(255 255 255 / 60%);
    --border-white-60: 1px solid var(--color-white-60);
    --color-black-70: rgb(0 0 0 / 70%);
    --border-black-70: 1px solid var(--color-black-70);
    --color-white-70: rgb(255 255 255 / 70%);
    --border-white-70: 1px solid var(--color-white-70);
    --color-black-80: rgb(0 0 0 / 80%);
    --border-black-80: 1px solid var(--color-black-80);
    --color-white-80: rgb(255 255 255 / 80%);
    --border-white-80: 1px solid var(--color-white-80);
    --color-black-90: rgb(0 0 0 / 90%);
    --border-black-90: 1px solid var(--color-black-90);
    --color-white-90: rgb(255 255 255 / 90%);
    --border-white-90: 1px solid var(--color-white-90);
}

`

      styles += `body { ${this.genStyles('light')} }\n`

      styles += `body.dark { ${this.genStyles('dark')} }\n`

      styles += `@media (prefers-color-scheme: dark) {\n`

      styles += `body { ${this.genStyles('dark')} }\n`

      styles += `body.light { ${this.genStyles('light')} }\n`

      styles += `}\n`

      styles += `${this.genBaseStyles()}\n`

      if (this.childWindow && this.childWindow.name === this.childWindowName) {
        const payload = JSON.stringify({
          type: 'colors-and-fonts',
          styles: styles,
          mode: this.mode(),
        })
        this.childWindow.postMessage(payload)
      } else {
        this.modLog('Window is not available')
      }
    }

    setupActiveStyles() {
      const styles = this.ownerDocument.createElement('style')
      let sheet = `
.main-wrapper { 
    padding: 1rem;
    background-color: var(--dev-color-base); 
    border-radius: 0.6rem;
    color: var(--dev-color-bravo);
    max-width: 1100px;
    margin: auto;
}

.primary-wrapper {
    display: flex;
    flex-wrap: wrap;
}

h2 {
    color: var(--dev-color-alfa);
}

.primary-chip-row {
    display: flex;
    flex-wrap: wrap;
}

.primary-chip {
    max-width: 160px;
    margin: 0.5rem;
    font-size: 0.7rem;
}

.chip-button {
    border: none;
    background: none;
    color: currentColor;
    outline: none;
    margin: 0;
    padding: 0;
    font-size: 0.7rem;
    cursor: pointer;
}


      `

      this.colors().forEach((color) => {
        const key = `dev-color-${color}`
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
      this.devProps[`--dev-color-base`] = `oklch(${this.lValue(
        this.mode()
      )}% ${this.cValue(this.mode())} ${this.hValue(this.mode())})`

      // alfa
      this.devProps[`--dev-color-alfa`] = `oklch(${
        (this.state.modes[this.mode()].l +
          this.state.modes[this.mode()].colors.alfa.l) %
        100
      }% ${
        ((this.state.modes[this.mode()].c * 10 +
          this.state.modes[this.mode()].colors.alfa.c) %
          5) /
        10
      } ${
        (this.state.modes[this.mode()].h +
          this.state.modes[this.mode()].colors.alfa.h) %
        360
      })`

      // bravo
      this.devProps[`--dev-color-bravo`] = `oklch(${
        (this.state.modes[this.mode()].l +
          this.state.modes[this.mode()].colors.bravo.l) %
        100
      }% ${
        ((this.state.modes[this.mode()].c * 10 +
          this.state.modes[this.mode()].colors.bravo.c) %
          5) /
        10
      } ${
        (this.state.modes[this.mode()].h +
          this.state.modes[this.mode()].colors.bravo.h) %
        360
      })`

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

      this.sendStylesheet()
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
