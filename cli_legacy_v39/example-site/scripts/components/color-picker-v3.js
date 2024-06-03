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
      this.els = {}
      this.attachShadow({ mode: 'open' })
      this.addInitialState()
      this.addStyles()
      this.addMainWrapper()
      this.buildModeButtons()
      this.buildLightDarkDefaultButtons()
      this.buildSliders()
      this.buildPrimaryButtons()
      this.buildPrimaryChips()
      this.buildSecondaryButtons()
      this.buildTertiaryButtons()
      this.update()
    }

    addInitialState() {
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
            l: 93.076,
            c: 0.03467,
            h: 92.999,
            colors: {
              alfa: {
                l: 40,
                c: 1,
                h: 120,
                collectionShift: 120,
                collectionIndex: 13,
              },
              bravo: {
                l: 40,
                c: 0,
                h: 0,
                collectionShift: 0,
                collectionIndex: 13,
              },
            },
          },
          dark: {
            display: '☾',
            l: 16.009,
            c: 0.03076,
            h: 39.927,
            colors: {
              alfa: {
                l: 40,
                c: 2,
                h: 180,
                collectionShift: 240,
                collectionIndex: 2,
              },
              bravo: {
                l: 80,
                c: 0,
                h: 300,
                collectionShift: 180,
                collectionIndex: 11,
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
              secondaryH: 240,
            },
            bravo: {
              secondaryH: 180,
            },
          },
        },
        defaultMode: 'dark',
        codeBlockColors: {
          alfa: { l: 40, c: 0.1, h: 30 },
          bravo: { l: 40, c: 0.1, h: 60 },
          charlie: { l: 40, c: 0.1, h: 90 },
          delta: { l: 40, c: 0.1, h: 120 },
          echo: { l: 40, c: 0.1, h: 150 },
          foxtrot: { l: 40, c: 0.1, h: 180 },
          golf: { l: 40, c: 0.1, h: 120 },
          hotel: { l: 40, c: 0.1, h: 180 },
          india: { l: 40, c: 0.1, h: 240 },
          juliet: { l: 40, c: 0.1, h: 300 },
          kilo: { l: 40, c: 0.1, h: 330 },
          lima: { l: 40, c: 0.1, h: 15 },
          mike: { l: 40, c: 0.1, h: 45 },
        },
      }

      // make sure active is set to light to start with
      this.state.active.colors.alfa.secondaryH =
        this.state.modes.light.colors.alfa.collectionShift
      this.state.active.colors.bravo.secondaryH =
        this.state.modes.light.colors.bravo.collectionShift
    }

    addMainWrapper() {
      this.els.mainWrapper = this.modAddTo(this.shadowRoot, 'div', {
        classes: ['main-wrapper'],
        innerHTML: `
  <div class="content-wrapper">
    <div class="interface-wrapper">
        <div class="preview-section section-wrapper"></div>
        <div class="top-section section-wrapper">
            <div class="mode-section"></div>
            <div class="slider-section"></div>
        </div>
        <div class="section-wrapper">
            <h3>Primary: <span class="dev-color-alfa">alfa</span> <span class="dev-color-bravo">bravo</span></h3>
            <div class="primary-section">
                <div class="primary-buttons"></div>
                <div class="primary-chips"></div>
            </div>
        </div>
        <div class="stripe-wrapper section-wrapper">
            <div class="stripe bg-alfa"></div>
            <div class="stripe bg-bravo"></div>
            <div class="stripe bg-charlie"></div>
            <div class="stripe bg-delta"></div>
            <div class="stripe bg-echo"></div>
            <div class="stripe bg-foxtrot"></div>
        </div>
        <div class="section-wrapper">
            <div class="secondary-section">
                <div class="secondary-wrapper charliedelta-section">
                    <h3>Secondary: 
                        <span class="dev-color-charlie">charlie</span>
                        <span class="dev-color-delta">delta</span>
                    </h3>
                    <div class="secondary-buttons charliedelta-buttons"></div>
                    <div class="charliedelta-chips"></div>
                </div>
                <div class="secondary-wrapper echofoxtrot-section">
                    <h3>Secondary: 
                        <span class="dev-color-echo">echo</span>
                        <span class="dev-color-foxtrot">foxtrot</span>
                    </h3>
                    <div class="secondary-buttons echofoxtrot-buttons"></div>
                    <div class="echofoxtrot-chips"></div>
                </div>
            </div>
        </div>

        <div class="section-wrapper">
        <h3>Instructions</h3>
        <ol class="flow">
            <li>
              Launch the Preview Window</li>
            <li>
              Use the Lightness, Chroma, and Hue sliders to pick a background color
              that the rest of the colors are based off of</li>
            <li>
              Use the six mini-palette squares in the Primary section to choose a base 
              set to work off</li>
            <li>
              Use the <em>alfa</em> and <em>bravo</em> buttons in the Primary section
              to set those colors</li>
            <li>
              Changing the <em>alfa</em> color also changes the palette set available
              for the Secondary <em>charlie</em> and <em>delta</em> colors. Those are chosen
              by first selecting one of the 3x3 color square buttons then choosing a specific 
              pair of colors below it</li>
            <li>
              Changing the <em>bravo</em> color in the Primary section changes the
              Secondary <em>echo</em> and <em>foxtrot</em> in the same way</li>
            <li>
              Switch to Dark mode next to the main Lightness, Chroma, and Hue 
              sliders and set it as well</li>
            <li>
              Select the default for ☀ or ☾ to determine if Light or Dark should
              be the first mode in the stylesheet that gets rendered if no
              other preferences are set
            </li>
            <li>
              Copy the CSS from the Stylesheet section and paste it in the files
              for your site
            </li>
        </ol> 
        <h3>The Randomizer</h3>
        <p>
          The <em>Randomize</em> button produces random values for the Lightness,
          Chroma, and Hue. It only changes the Light/Dark mode you're currently on. 
          If Light mode is active the Lightness is above 50%. In Dark mode it's
          below 50%. After you've randomized you can tweak or pick colors. 
        </p>
        <h3>Notes</h3>
        <ul class="flow">
            <li>
                I've tried a bunch of color pickers. I never found one I liked. It
                always felt like I was missing some knowledge about how to use them to
                get a good looking palette. I built this one with the goal of addressing that. It's 
                all based off math. I'll write up more details, but basically you 
                pick a base/background color with the Lightness, Chrome, and Hue sliders
                and all the other colors are generated by calculating relative positions
                in the color space. (Lightness increments at 20%, Chroma by 0.1, and Hue's
                interval is 60°)
            </li>
            <li>
                One of the biggest problems I had with other tools is they'd often
                just provide swatches. I don't have the skill to just know what that
                would look like on a page. I'd have to keep copying the color values
                into a local page to be able to see what things would look like. 
                I'm addressing that here with the Preview Window. It shows what
                the colors actually look like on a page
            </li>
            <li>
                The picker is set up to make two palettes at the same time. One
                for Light mode and one for Dark. The palettes are independent.
                The copy buttons at the end of the sliders can be used to
                pull in the corresponding value from the alternate mode
                to sync things up
            </li>
            <li>
                The stylesheets ends up around 40KB. It compresses to around
                4KB. I'm not planning to add any tree-shaking/filtering to this
                tool. That's something to handle down-stream if it
                becomes necessary
            </li>
            <li>
                The picker uses <a href="https://oklch.com/">OKLCH</a> 
                which I'm finding much nicer to work with 
                in terms of the way the math picks the colors. I'm using a
                naive approach that results in values that fall out of 
                the range of colors inside the color space. I'll see about 
                addressing that at some point. For now, I'm relying on the
                fact that the browser fallbacks are working fine
                </li>
            <li>
                The output stylesheet includes two copies of both the 
                dark and light mode color. This is done to work mode
                switchers like the  
                <a href="https://www.alanwsmith.com/en/2sgie56k/">
                web component one I'm working on</a>
            </li>
            <li>
              The stylesheet also includes sizes. That's prep work for 
              a future iteration that'll let you change fonts settings as well
            </li>
            <li>
              The Raw Data at the bottom contains the core values used in the 
              calculations. It's for debugging right now. Eventually, 
              you'll be able to save and load them
            </li>
            <li>I'm looking to add calculations for contrast ratios between
            the colors to help measure against things like the
            <a href="https://www.w3.org/WAI/WCAG21/Understanding/contrast-minimum.html">WCAG
            minimum contract guidelines</a>.
            I'm not sure how to do that with OKLCH yet. If you know, 
            <a href="https://hachyderm.io/deck/@TheIdOfAlan">hit me up on mastodon</a>
            </li>
        </ul>
        </div>

        <div class="stylesheet-wrapper section-wrapper">
            <code-block>
              <h3>Stylesheet</h3>
              <pre class="the-stylesheet scroll"></pre>
            </code-block>
        </div>
        
        <div class="debug-wrapper section-wrapper">
        <code-block>
          <h3>Raw Data</h3>
          <pre slot="code" class="raw-data"></pre>
        </code-block>
        </div>
        
    </div>
  </div>

          `,
      })
    }

    addStyles() {
      const styles = this.ownerDocument.createElement('style')
      let sheet = `
@font-face {
    font-family: 'Inter';
    src: url('/theme/fonts/Inter-VariableFont_slnt,wght.ttf') format('opentype');
}

.bg-alfa {
    background-color: var(--dev-color-alfa);
}

.bg-bravo {
    background-color: var(--dev-color-bravo);
}

.bg-charlie {
    background-color: var(--dev-color-charlie);
}

.bg-delta {
    background-color: var(--dev-color-delta);
}

.bg-echo {
    background-color: var(--dev-color-echo);
}

.bg-foxtrot {
    background-color: var(--dev-color-foxtrot);
}

button {
    background: none;
    border: none;
    color: currentColor;
    margin: 0;
    padding: 0;
    cursor: pointer;
    font-size: 0.9rem;
}

button:hover {
  text-decoration: underline;
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

.chip-button-alfa {
    text-align: left;
}

.chip-button-bravo {
    text-align: right;
}

.chip-buttons {
    display: grid;
    grid-template-columns: 1fr 1fr;
    margin-top: 0.4rem;
}

.chip-swatch {
    min-width: 0.1rem;
    min-height: 0.1rem;
}

.chip-title {
    font-weight: 700;
}

.flow > :where(:not(:first-child)) {
  margin-top: var(--flow-space, 1em);
}

h2, h3 {
    color: var(--dev-color-bw-reverse-90);
}

.ld-default-button {
  margin-inline: 0.3rem;
}

.ld-wrapper {
  margin-top: 0.8rem;
  text-align: center;
  font-size: 0.9rem;
}

.main-wrapper { 
    padding: 1rem;
    background-color: var(--dev-color-base); 
    border-radius: 0.6rem;
    color: var(--dev-color-bravo);
    max-width: 1200px;
    margin: auto;
    font-family: 'Inter';
}

.mode-button {
  margin-bottom: 0.4rem;
}

.mode-button-selected {
    text-decoration: underline;
}

.mode-section {
    display: grid;
}

.preview-section {
    display: grid;
    gap: 14px;
}

.primary-button {
    margin-bottom: 0.9rem;
    margin;
    border-radius: 0.3rem;
}

.primary-button-selected {
    margin-bottom: 0.9rem;
    border: 2px solid var(--dev-color-bw-reverse-90);
}

.primary-chip {
    max-width: 110px;
    margin: 0.6rem;
    font-size: 0.7rem;
    display: grid;
    grid-template-columns: 11px 1fr;
    gap: 6px;
    border: 1px solid rgb(0 0 0 / 0%);
}

.primary-chip-selected {
    border: 1px solid var(--dev-color-bw-reverse);
}

.primary-chip-row {
    display: flex;
    flex-wrap: wrap;
}

.primary-rect {
    cursor: pointer;
}

.primary-section {
    display: grid;
    grid-template-columns: 70px 1fr;
}

.primary-wrapper {
    display: flex;
    flex-wrap: wrap;
}

.secondaryButton {
    margin: 2px;
    border-radius: 0.4rem;
}

.secondaryButton-selected {
    border: 2px solid var(--dev-color-bw-reverse-90);
}

.secondary-buttons {
    display: flex;
    flex-wrap: flex;
    gap: 20px;
}

.secondary-rect {
    cursor: pointer;
}

.secondary-section {
    display: flex;
    flex-wrap: flex;
    gap: 20px;
}

.secondary-wrapper {
    display: grid;
    border: 1px solid var(--dev-color-bw-reverse-40);
    padding: 0.7rem;
    border-radius: 0.5rem;
}

.section-wrapper {
    border: 1px solid var(--dev-color-bw-reverse-40);
    padding: 0.9rem;
    border-radius: 0.5rem;
    margin-bottom: 2.5rem;
}

.selected {
  text-decoration: underline;
}

.sliders {
    display: grid;
    grid-template-columns: 100px 1fr 80px;
}

.stripe {
    min-width: 6px;
    min-height: 18px;
}

.stripe-wrapper {
    border: 1px solid var(--dev-color-bw-reverse-40);
    display: grid;
    grid-template-columns: 1fr 1fr 1fr 1fr 1fr 1fr;
    border-radius: 0.5rem;
    margin-right: 0.9rem;
}

.stripe-wrapper :first-child {
    border-top-left-radius: 0.4rem;
    border-bottom-left-radius: 0.4rem;
}

.stripe-wrapper :last-child {
    border-top-right-radius: 0.4rem;
    border-bottom-right-radius: 0.4rem;
}

.strong {
    font-weight: 700;
}

.tertiary-chip {
    margin-inline: 2px;
    margin-block: 3px;
}

.tertiary-chip  {
  border-radius: 0.2rem;
}

.tertiary-chip-selected {
    margin-inline: 0px;
    margin-block: 0px;
    border-inline: 2px solid var(--dev-color-bw-reverse-90);
    border-block: 3px solid var(--dev-color-bw-reverse-90);
}

.tertiary-rect {
    cursor: pointer;
}

.the-stylesheet {
  font-size: var(--size-9);
}

.top-section {
    display: grid;
    grid-template-columns: 180px 1fr;
    margin-top: 1.7rem;
    margin-bottom: 1.9rem;
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

      // chip swatches and selected
      this.lValues().forEach((l) => {
        this.cValues().forEach((c, cIndex) => {
          const key = `chip-${l}-${cIndex}`
          sheet += `.chip-swatch-${l}-${cIndex} { background-color: var(--${key}); }`
          sheet += `.primary-chip-selected-${l}-${cIndex} { border: 1px solid var(--${key}); }`
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
          const key = `tertiary-rect-${primary.key}-${coords[0]}-${coords[1]}`
          sheet += `.${key} { fill: var(--${key}); }`
        })
      })

      styles.innerHTML = sheet
      this.shadowRoot.appendChild(styles)
    }

    buildLightDarkDefaultButtons() {
      const ldWrapper = this.modAddTo(`.mode-section`, 'div', {
        classes: ['ld-wrapper'],
        innerHTML: `Default`,
      })
      this.modes().forEach((mode) => {
        this.modAddTo(ldWrapper, 'button', {
          innerHTML:
            mode === 'light'
              ? this.state.modes.light.display
              : this.state.modes.dark.display,
          data: [['mode', mode]],
          listeners: [
            [
              'click',
              (event) => {
                this.handleLightDarkDefaultClick.call(this, event)
              },
            ],
          ],
          classes: [`ld-default-button`, `ld-default-button-${mode}`],
        })
      })
    }

    buildModeButtons() {
      this.modes().forEach((mode) => {
        this.modAddTo(`.mode-section`, 'button', {
          innerHTML: mode === 'light' ? 'Light ☀' : 'Dark ☾',
          data: [['mode', mode]],
          listeners: [
            [
              'click',
              (event) => {
                this.handleModeClick.call(this, event)
              },
            ],
          ],
          // listeners: [['input', () => { console.log("HEREREREH") }]],
          classes: [`mode-button`, `mode-button-${mode}`],
        })
      })

      //   this.modes().forEach((mode) => {
      //     const label = this.modAddTo('.modes', 'label', {
      //       innerHTML: `<span>${mode}</span>`,
      //     })
      //     this.modAddTo(label, 'input', {
      //       type: 'radio',
      //       name: 'mode',
      //       checked: mode === 'light' ? true : false,
      //       value: mode,
      //       listeners: [
      //         [
      //           'input',
      //           (event) => {
      //             this.handleModeClick.call(this, event)
      //           },
      //         ],
      //       ],
      //       // listeners: [['input', () => { console.log("HEREREREH") }]],
      //       classes: [`mode-${mode}`],
      //     })
      //   })
    }

    buildPreviewButton() {
      if (this.dataset.previewHref) {
        this.modAddTo(`.preview-section`, 'button', {
          innerHTML: 'Launch Preview Window',
          listeners: [
            [
              'click',
              (event) => this.handlePreviewButtonClick.call(this, event),
            ],
          ],
        })
      } else {
        this.modAddTo(`.preview-section`, 'div', {
          innerHTML: 'Error: preview href not defined',
        })
      }

      this.modAddTo(`.preview-section`, 'button', {
        innerHTML: 'Randomize',
        listeners: [
          ['click', (event) => this.handleRandomizeClick.call(this, event)],
        ],
      })
    }

    buildPrimaryButton(parent, h) {
      const svg = this.modAddSvgTo(parent, 'svg', {
        width: 50,
        height: 50,
        classes: ['primary-button', `primary-button-${h}`],
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
      this.hValues().forEach((h) => {
        this.buildPrimaryButton(`.primary-buttons`, h)
      })
    }

    buildPrimaryChips() {
      this.lValues()
        .reverse()
        .forEach((l, lIndex) => {
          const chipLine = this.modAddTo(`.primary-chips`, 'div', {
            classes: ['primary-chip-row'],
          })
          this.cValues().forEach((c, cIndex) => {
            this.modAddTo(chipLine, 'div', {
              innerHTML: `
        <div class="chip-swatch chip-swatch-${l}-${cIndex}"></div>
        <div class="chip-details">
          <div class="chip-title chip-title-${l}-${cIndex}">#</div>
          <div class="chip-text">${this.state.sampleText}</div>
          <div class="chip-buttons chip-buttons-${l}-${cIndex}"></div>
        </div>`,
              classes: ['primary-chip', `chip-${l}-${cIndex}`],
            })
            this.primaryColors().forEach((color) => {
              this.modAddTo(`.chip-buttons-${l}-${cIndex}`, 'button', {
                classes: [
                  `chip-button`,
                  `chip-button-${color}`,
                  `chip-button-${color}-${l}-${cIndex}`,
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

    buildSecondaryButtons() {
      this.primaries().forEach((primary) => {
        const mainKey = primary.secondaries.join('')
        // const secondaryButtons = this.modAddTo(this.els.mainWrapper, 'div', {
        //   classes: ['secondary-buttons', `${mainKey}-chips`],
        // })

        const key = primary.secondaries.join('')
        this.hValues().forEach((h, hIndex) => {
          let buttonWrapper = this.modAddTo(`.${mainKey}-buttons`, 'div', {
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
                  `secondary-rect`,
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
      const sliders = this.modAddTo(`.slider-section`, 'div', {
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
        innerHTML: `copy ${this.state.modes.dark.display}`,
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
              'tertiary-chip',
              `tertiary-chip-index-${primary.key}-${collectionIndex}`,
            ],
            width: 20,
            height: 40,
          })

          collection.forEach((coords, coordsIndex) => {
            const key = primary.key
            this.modAddSvgTo(el, 'rect', {
              classes: [
                'tertiary-rect',
                `tertiary-rect-${key}-${coords[0]}-${coords[1]}`,
              ],
              x: 0,
              y: coordsIndex * 20,
              width: 20,
              height: 20,
              data: [
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

    connectedCallback() {
      this.buildPreviewButton()
      this.update()
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
      let styles = ``

      const keys = [
        'base',
        'alfa',
        'bravo',
        'charlie',
        'delta',
        'echo',
        'foxtrot',
        'black',
        'white',
        'bw-match',
        'bw-reverse',
      ]

      keys.forEach((key) => {
        styles += `.color-${key} { color: var(--color-${key}); }\n`
        for (let alpha = 10; alpha < 100; alpha = alpha + 10) {
          styles += `.color-${key}-${alpha} { color: var(--color-${key}-${alpha}); }\n`
        }
      })

      keys.forEach((key) => {
        styles += `.bg-${key} { background-color: var(--color-${key}); }\n`
        for (let alpha = 10; alpha < 100; alpha = alpha + 10) {
          styles += `.bg-${key}-${alpha} { background-color: var(--color-${key}-${alpha}); }\n`
        }
      })

      for (let key in this.state.codeBlockColors) {
        styles += `.code-block-${key} { color: var(--code-block-${key}); }\n`
      }

      styles += `.code-block-line-numbers { color: var(--code-block-line-numbers); }\n`
      styles += `.code-block-base { color: var(--code-block-base); }\n`
      styles += `.code-block-border { color: var(--code-block-border); }\n`
      return styles
    }

    genMatchStyles(mode) {
      let config = {
        light: { match: '255 255 255', reverse: '0 0 0' },
        dark: { match: '0 0 0', reverse: '255 255 255' },
      }
      let styles = ``

      styles += `--color-bw-match: rgb(${config[mode].match});\n`
      for (let alpha = 90; alpha > 0; alpha = alpha - 10) {
        styles += `--color-bw-match-${alpha}: rgb(${config[mode].match} / ${alpha}%);\n`
      }

      styles += `--color-bw-reverse: rgb(${config[mode].reverse});\n`
      for (let alpha = 90; alpha > 0; alpha = alpha - 10) {
        styles += `--color-bw-reverse-${alpha}: rgb(${config[mode].reverse} / ${alpha}%);\n`
      }

      styles += `--code-block-base: rgb(${config[mode].match} / 20%);\n`

      styles += `--code-block-border: rgb(${config[mode].reverse} / 60%);\n`

      return styles
    }

    genStyles(mode) {
      let response = ``
      const theValues = {
        base: [
          this.state.modes[mode].l,
          this.state.modes[mode].c,
          this.state.modes[mode].h,
        ],
        alfa: this.getAlfa(mode),
        bravo: this.getBravo(mode),
        charlie: this.getCharlie(mode),
        delta: this.getDelta(mode),
        echo: this.getEcho(mode),
        foxtrot: this.getFoxtrot(mode),
      }
      for (let color in theValues) {
        response += this.prop(
          `--color-${color}`,
          `oklch(${theValues[color][0].toFixed(3)}% ${theValues[
            color
          ][1].toFixed(5)} ${theValues[color][2].toFixed(3)})`
        )
        for (let alpha = 90; alpha > 0; alpha = alpha - 10) {
          response += this.prop(
            `--color-${color}-${alpha}`,
            `oklch(${theValues[color][0].toFixed(3)}% ${theValues[
              color
            ][1].toFixed(5)} ${theValues[color][2].toFixed(3)} / ${alpha}%)`
          )
        }
      }

      for (let color in this.state.codeBlockColors) {
        const theValues = this.getCodeBlockColor(
          mode,
          this.state.codeBlockColors[color].l,
          this.state.codeBlockColors[color].c,
          this.state.codeBlockColors[color].h
        )
        response += this.prop(
          `--code-block-${color}`,
          `oklch(${theValues[0].toFixed(3)}% ${theValues[1].toFixed(
            5
          )} ${theValues[2].toFixed(3)})`
        )
      }

      const forLineNumbers = this.getBravo(mode)
      response += this.prop(
        `--code-block-line-numbers`,
        `oklch(${forLineNumbers[0].toFixed(3)}% ${forLineNumbers[1].toFixed(
          5
        )} ${forLineNumbers[2].toFixed(3)} / 45%)`
      )

      return response.trim()
    }

    genStylesFull() {
      let styles = `
      :root {

    --size-base: 16px;
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

    --width-alfa: 40rem;

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

      const mode = this.state.defaultMode
      const altMode = mode === 'light' ? 'dark' : 'light'

      // styles += `:root {
      //   color-scheme: ${mode} ${altMode};
      // }
      // `

      styles += `body { 
      ${this.genMatchStyles(mode)}        
      ${this.genStyles(mode)} 
      }`
      styles += `body.${altMode} { 
              ${this.genMatchStyles(altMode)}  
              ${this.genStyles(altMode)} 
          }\n`
      styles += `@media (prefers-color-scheme: ${altMode}) {\n`
      styles += `body { 
              ${this.genMatchStyles(altMode)}  
              ${this.genStyles(altMode)} 
          }\n`
      styles += `body.${mode} { 
              ${this.genMatchStyles(mode)}  
              ${this.genStyles(mode)} 
          }\n`
      styles += `}\n`

      styles += `${this.genBaseStyles()}\n`

      return styles
    }

    getAlfa(mode) {
      let payload = []
      payload.push(
        (this.state.modes[mode].l + this.state.modes[mode].colors.alfa.l) % 100
      )
      payload.push(
        ((this.state.modes[mode].c * 10 +
          this.state.modes[mode].colors.alfa.c) %
          5) /
          10
      )
      payload.push(
        (this.state.modes[mode].h + this.state.modes[mode].colors.alfa.h) % 360
      )
      return payload
    }

    getBravo(mode) {
      let payload = []

      payload.push(
        (this.state.modes[mode].l + this.state.modes[mode].colors.bravo.l) % 100
      )
      payload.push(
        ((this.state.modes[mode].c * 10 +
          this.state.modes[mode].colors.bravo.c) %
          5) /
          10
      )
      payload.push(
        (this.state.modes[mode].h + this.state.modes[mode].colors.bravo.h) % 360
      )

      return payload
    }

    getCharlie(mode) {
      let payload = []
      payload.push(
        (this.state.modes[mode].l +
          this.state.modes[mode].colors.alfa.l +
          this.state.base.l.max +
          this.state.collections[
            this.state.modes[mode].colors.alfa.collectionIndex
          ][0][0] *
            this.state.base.l.interval) %
          100
      )
      payload.push(
        ((this.state.modes[mode].c * 10 +
          this.state.modes[mode].colors.alfa.c +
          this.state.base.c.max * 10 +
          this.state.collections[
            this.state.modes[mode].colors.alfa.collectionIndex
          ][0][1] *
            (this.state.base.c.interval * 10)) %
          5) /
          10
      )
      payload.push(
        (this.state.modes[mode].h +
          this.state.modes[mode].colors.alfa.h +
          this.state.modes[mode].colors.alfa.collectionShift) %
          360
      )
      return payload
    }

    getCodeBlockColor(mode, l, c, h) {
      const payload = []
      if (mode === 'light') {
        let newL = this.state.modes[mode].colors.bravo.l + l
        if (newL > this.state.base.l.max) {
          payload.push(this.state.modes[mode].colors.bravo.l - l)
        } else {
          payload.push(newL)
        }
        let newC = this.state.modes[mode].colors.bravo.c + c
        if (newC > 0.31) {
          payload.push(this.state.modes[mode].colors.bravo.c - c)
        } else {
          payload.push(newC)
        }
        let newH = this.state.modes[mode].colors.bravo.h + h
        if (newH > this.state.base.h.max) {
          payload.push(this.state.modes[mode].colors.bravo.h - h)
        } else {
          payload.push(newH)
        }
      } else {
        let newL = this.state.modes[mode].colors.bravo.l - l
        if (newL < 0) {
          payload.push(this.state.modes[mode].colors.bravo.l + l)
        } else {
          payload.push(newL)
        }
        let newC = this.state.modes[mode].colors.bravo.c - c
        if (newC < 0) {
          payload.push(this.state.modes[mode].colors.bravo.c + c)
        } else {
          payload.push(newC)
        }
        let newH = this.state.modes[mode].colors.bravo.h - h
        if (newH < 0) {
          payload.push(this.state.modes[mode].colors.bravo.h + h)
        } else {
          payload.push(newH)
        }
      }
      return payload
    }

    getDelta(mode) {
      let payload = []
      payload.push(
        (this.state.modes[mode].l +
          this.state.modes[mode].colors.alfa.l +
          this.state.base.l.max +
          this.state.collections[
            this.state.modes[mode].colors.alfa.collectionIndex
          ][1][0] *
            this.state.base.l.interval) %
          100
      )
      payload.push(
        ((this.state.modes[mode].c * 10 +
          this.state.modes[mode].colors.alfa.c +
          this.state.base.c.max * 10 +
          this.state.collections[
            this.state.modes[mode].colors.alfa.collectionIndex
          ][1][1] *
            (this.state.base.c.interval * 10)) %
          5) /
          10
      )
      payload.push(
        (this.state.modes[mode].h +
          this.state.modes[mode].colors.alfa.h +
          this.state.modes[mode].colors.alfa.collectionShift) %
          360
      )
      return payload
    }

    getEcho(mode) {
      let payload = []
      payload.push(
        (this.state.modes[mode].l +
          this.state.modes[mode].colors.bravo.l +
          this.state.base.l.max +
          this.state.collections[
            this.state.modes[mode].colors.bravo.collectionIndex
          ][0][0] *
            this.state.base.l.interval) %
          100
      )
      payload.push(
        ((this.state.modes[mode].c * 10 +
          this.state.modes[mode].colors.bravo.c +
          this.state.base.c.max * 10 +
          this.state.collections[
            this.state.modes[mode].colors.bravo.collectionIndex
          ][0][1] *
            (this.state.base.c.interval * 10)) %
          5) /
          10
      )
      payload.push(
        (this.state.modes[mode].h +
          this.state.modes[mode].colors.bravo.h +
          this.state.modes[mode].colors.bravo.collectionShift) %
          360
      )
      return payload
    }

    getFoxtrot(mode) {
      let payload = []
      payload.push(
        (this.state.modes[mode].l +
          this.state.modes[mode].colors.bravo.l +
          this.state.base.l.max +
          this.state.collections[
            this.state.modes[mode].colors.bravo.collectionIndex
          ][1][0] *
            this.state.base.l.interval) %
          100
      )
      payload.push(
        ((this.state.modes[mode].c * 10 +
          this.state.modes[mode].colors.bravo.c +
          this.state.base.c.max * 10 +
          this.state.collections[
            this.state.modes[mode].colors.bravo.collectionIndex
          ][1][1] *
            (this.state.base.c.interval * 10)) %
          5) /
          10
      )
      payload.push(
        (this.state.modes[mode].h +
          this.state.modes[mode].colors.bravo.h +
          this.state.modes[mode].colors.bravo.collectionShift) %
          360
      )

      return payload
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

    handleLightDarkDefaultClick(event) {
      this.state.defaultMode = event.target.dataset.mode
      // this.lch().forEach((key) => {
      // this.modSetValue(`.slider-${key}`, this.state.modes[this.mode()][key])
      // this.modUpdateHTML(
      //   `.get-from-${key}`,
      //   `copy ${this.state.modes[this.otherMode()].display}`
      // )
      // })
      this.update()
    }

    handleModeClick(event) {
      this.state.active.mode = event.target.dataset.mode
      this.lch().forEach((key) => {
        this.modSetValue(`.slider-${key}`, this.state.modes[this.mode()][key])
        this.modUpdateHTML(
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
          this.dataset.previewHref,
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

    handleRandomizeClick(event) {
      if (this.mode() === 'light') {
        this.state.modes[this.mode()].l = this.randomNumBetween(50, 99)
        this.state.modes[this.mode()].c =
          this.randomNumBetween(1000, 3200000) / 10000000
        this.state.modes[this.mode()].h = this.randomNumBetween(0, 359)
        this.state.modes[this.mode()].colors.alfa.l =
          this.randomNumBetween(0, 4) * 20
        this.state.modes[this.mode()].colors.alfa.c = this.randomNumBetween(
          0,
          4
        )
      } else {
        this.state.modes[this.mode()].l = this.randomNumBetween(0, 50)
        this.state.modes[this.mode()].c =
          this.randomNumBetween(1000, 3200000) / 10000000
        this.state.modes[this.mode()].h = this.randomNumBetween(0, 359)
        this.state.modes[this.mode()].colors.alfa.l =
          this.randomNumBetween(0, 4) * 20
        this.state.modes[this.mode()].colors.alfa.c = this.randomNumBetween(
          0,
          4
        )
      }
      this.state.modes[this.mode()].colors.alfa.h =
        this.randomNumBetween(0, 6) * 60
      this.state.modes[this.mode()].colors.alfa.collectionShift =
        this.randomNumBetween(0, 6) * 60
      this.state.modes[this.mode()].colors.alfa.collectionIndex =
        this.randomNumBetween(0, 15)
      this.state.modes[this.mode()].colors.bravo.l =
        this.randomNumBetween(0, 4) * 20
      this.state.modes[this.mode()].colors.bravo.c = this.randomNumBetween(0, 4)
      this.state.modes[this.mode()].colors.bravo.h =
        this.randomNumBetween(0, 6) * 60
      this.state.modes[this.mode()].colors.bravo.collectionShift =
        this.randomNumBetween(0, 6) * 60
      this.state.modes[this.mode()].colors.bravo.collectionIndex =
        this.randomNumBetween(0, 15)
      this.updateSliders()
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
      this.state.modes[this.mode()].colors[
        event.target.dataset.primary
      ].collectionIndex = parseInt(event.target.dataset.collectionIndex, 10)
      this.state.modes[this.mode()].colors[
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

    prop(key, value) {
      return `${key}: ${value};\n`
    }

    randomNumBetween(min, max) {
      return Math.floor(Math.random() * (max - min + 1) + min)
    }

    sendStylesheet() {
      // TODO: Add bw-match and bw-reverse

      if (this.childWindow && this.childWindow.name === this.childWindowName) {
        const payload = JSON.stringify({
          type: 'colors-and-fonts',
          styles: this.genStylesFull(),
          mode: this.mode(),
        })
        this.childWindow.postMessage(payload)
      } else {
        this.modLog('Window is not available')
      }
    }

    updateSliders() {
      this.modSetValue(`.slider-l`, this.state.modes[this.mode()].l)
      this.modSetValue(`.slider-c`, this.state.modes[this.mode()].c)
      this.modSetValue(`.slider-h`, this.state.modes[this.mode()].h)
    }

    update() {
      // set the active base
      this.devProps[`--dev-color-base`] = `oklch(${this.lValue(
        this.mode()
      )}% ${this.cValue(this.mode())} ${this.hValue(this.mode())})`

      // set the reverse style
      // only doing a few since that's all that's needed for the d
      // design
      if (this.mode() === 'light') {
        this.devProps[`--dev-color-bw-match-40`] = `rgb(255 255 255 / 40%)`
        this.devProps[`--dev-color-bw-match-70`] = `rgb(255 255 255 / 70%)`
        this.devProps[`--dev-color-bw-match-90`] = `rgb(255 255 255 / 90%)`
        this.devProps[`--dev-color-bw-reverse-40`] = `rgb(0 0 0 / 40%)`
        this.devProps[`--dev-color-bw-reverse-70`] = `rgb(0 0 0 / 70%)`
        this.devProps[`--dev-color-bw-reverse-90`] = `rgb(0 0 0 / 90%)`
      } else {
        this.devProps[`--dev-color-bw-match-40`] = `rgb(0 0 0 / 40%)`
        this.devProps[`--dev-color-bw-match-70`] = `rgb(0 0 0 / 70%)`
        this.devProps[`--dev-color-bw-match-90`] = `rgb(0 0 0 / 90%)`
        this.devProps[`--dev-color-bw-reverse-40`] = `rgb(255 255 255 / 40%)`
        this.devProps[`--dev-color-bw-reverse-70`] = `rgb(255 255 255 / 70%)`
        this.devProps[`--dev-color-bw-reverse-90`] = `rgb(255 255 255 / 90%)`
      }

      // set the dev-colors
      const alfaValues = this.getAlfa(this.mode())
      this.devProps[
        `--dev-color-alfa`
      ] = `oklch(${alfaValues[0]}% ${alfaValues[1]} ${alfaValues[2]})`

      console.log(alfaValues)

      const bravoValues = this.getBravo(this.mode())
      this.devProps[
        `--dev-color-bravo`
      ] = `oklch(${bravoValues[0]}% ${bravoValues[1]} ${bravoValues[2]})`

      const charlieValues = this.getCharlie(this.mode())
      this.devProps[
        `--dev-color-charlie`
      ] = `oklch(${charlieValues[0]}% ${charlieValues[1]} ${charlieValues[2]})`

      const deltaValues = this.getDelta(this.mode())
      this.devProps[
        `--dev-color-delta`
      ] = `oklch(${deltaValues[0]}% ${deltaValues[1]} ${deltaValues[2]})`

      const echoValues = this.getEcho(this.mode())
      this.devProps[
        `--dev-color-echo`
      ] = `oklch(${echoValues[0]}% ${echoValues[1]} ${echoValues[2]})`

      const foxtrotValues = this.getFoxtrot(this.mode())
      this.modLogObject(foxtrotValues)
      this.devProps[
        `--dev-color-foxtrot`
      ] = `oklch(${foxtrotValues[0]}% ${foxtrotValues[1]} ${foxtrotValues[2]})`

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
          this.collections().forEach((collection, collectionIndex) => {
            let h2 =
              (this.state.modes[this.mode()].colors[primary.key].h + h) % 360
            collection.forEach((coords) => {
              const key = `color-secondary-rect-coords-${primary.key}-${coords[0]}-${coords[1]}-${h}`
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
        this.collectionCoords().forEach((coords, coordsIndex) => {
          const key = `tertiary-rect-${primary.key}-${coords[0]}-${coords[1]}`
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

      this.primaries().forEach((primary, primaryIndex) => {
        this.collections().forEach((collection, collectionIndex) => {
          this.modRemoveStyleFrom(
            `.tertiary-chip-index-${primary.key}-${collectionIndex}`,
            `tertiary-chip-selected`
          )
          if (
            this.state.modes[this.mode()].colors[primary.key]
              .collectionShift ===
            this.state.active.colors[primary.key].secondaryH
          ) {
            if (
              collectionIndex ===
              this.state.modes[this.mode()].colors[primary.key].collectionIndex
            ) {
              this.modAddStyleTo(
                `.tertiary-chip-index-${primary.key}-${collectionIndex}`,
                `tertiary-chip-selected`
              )
            } else {
            }
          }
        })
      })

      // mode button highlight
      if (this.state.active.mode === 'light') {
        this.modAddStyleTo(`.mode-button-light`, `mode-button-selected`)
        this.modRemoveStyleFrom(`.mode-button-dark`, `mode-button-selected`)
      } else {
        this.modAddStyleTo(`.mode-button-dark`, `mode-button-selected`)
        this.modRemoveStyleFrom(`.mode-button-light`, `mode-button-selected`)
      }

      // update all the dev props
      for (let prop in this.devProps) {
        this.ownerDocument.documentElement.style.setProperty(
          prop,
          this.devProps[prop]
        )
      }

      // update the primary button styles
      this.hValues().forEach((h) => {
        if (h === this.state.active.h) {
          this.modAddStyleTo(`.primary-button-${h}`, `primary-button-selected`)
        } else {
          this.modRemoveStyleFrom(
            `.primary-button-${h}`,
            `primary-button-selected`
          )
        }
      })

      // update the secondary button styles
      this.primaryColors().forEach((color) => {
        this.hValues().forEach((h) => {
          let hCheck = this.state.active.colors[color].secondaryH % 360
          if (h === hCheck) {
            this.modAddStyleTo(
              `.secondaryButton-${color}-${h}`,
              `secondaryButton-selected`
            )
          } else {
            this.modRemoveStyleFrom(
              `.secondaryButton-${color}-${h}`,
              `secondaryButton-selected`
            )
          }
        })
      })

      // primary chip titles
      this.lValues().forEach((l) => {
        this.cValues().forEach((c, cIndex) => {
          this.modUpdateHTML(
            `.chip-title-${l}-${cIndex}`,
            `# ${l}-${cIndex}-${this.state.active.h}`
          )
        })
      })

      //   // primary chip selected
      //   // Off for now since the border was so strong
      //   // visually
      //   this.lValues().forEach((l) => {
      //     this.cValues().forEach((c, cIndex) => {
      //       this.primaryColors().forEach((color) => {
      //         if (
      //           this.state.modes[this.mode()].colors.alfa.l === l &&
      //           this.state.modes[this.mode()].colors.alfa.c === cIndex &&
      //           this.state.modes[this.mode()].colors.alfa.h ===
      //             this.state.active.h
      //         ) {
      //           this.modAddStyleTo(
      //             `.chip-${l}-${cIndex}`,
      //             `primary-chip-selected-${l}-${cIndex}`
      //           )
      //         } else if (
      //           this.state.modes[this.mode()].colors.bravo.l === l &&
      //           this.state.modes[this.mode()].colors.bravo.c === cIndex &&
      //           this.state.modes[this.mode()].colors.bravo.h ===
      //             this.state.active.h
      //         ) {
      //           this.modAddStyleTo(
      //             `.chip-${l}-${cIndex}`,
      //             `primary-chip-selected-${l}-${cIndex}`
      //           )
      //         } else {
      //           this.modRemoveStyleFrom(
      //             `.chip-${l}-${cIndex}`,
      //             `primary-chip-selected-${l}-${cIndex}`
      //           )
      //         }
      //       })
      //     })
      //   })

      //  alfa bravo bolds
      this.lValues().forEach((l) => {
        this.cValues().forEach((c, cIndex) => {
          this.primaryColors().forEach((color) => {
            if (
              this.state.modes[this.mode()].colors.alfa.l === l &&
              this.state.modes[this.mode()].colors.alfa.c === cIndex &&
              this.state.modes[this.mode()].colors.alfa.h ===
                this.state.active.h
            ) {
              this.modUpdateHTML(
                `.chip-button-alfa-${l}-${cIndex}`,
                `alfa &lt;`
              )
              this.modAddStyleTo(`.chip-button-alfa-${l}-${cIndex}`, `strong`)
            } else {
              this.modUpdateHTML(`.chip-button-alfa-${l}-${cIndex}`, `alfa`)
              this.modRemoveStyleFrom(
                `.chip-button-alfa-${l}-${cIndex}`,
                `strong`
              )
            }

            if (
              this.state.modes[this.mode()].colors.bravo.l === l &&
              this.state.modes[this.mode()].colors.bravo.c === cIndex &&
              this.state.modes[this.mode()].colors.bravo.h ===
                this.state.active.h
            ) {
              this.modUpdateHTML(
                `.chip-button-bravo-${l}-${cIndex}`,
                `&gt; bravo`
              )
              this.modAddStyleTo(`.chip-button-bravo-${l}-${cIndex}`, `strong`)
            } else {
              this.modUpdateHTML(`.chip-button-bravo-${l}-${cIndex}`, `bravo`)
              this.modRemoveStyleFrom(
                `.chip-button-bravo-${l}-${cIndex}`,
                `strong`
              )
            }
          })
        })
      })

      // light/dark default button highlight
      this.modes().forEach((mode) => {
        if (mode == this.state.defaultMode) {
          this.modAddStyleTo(`.ld-default-button-${mode}`, 'selected')
        } else {
          this.modRemoveStyleFrom(`.ld-default-button-${mode}`, 'selected')
        }
      })

      this.modUpdateHTML(`.raw-data`, JSON.stringify(this.state.modes, null, 2))

      this.modUpdateHTML(`.the-stylesheet`, this.genStylesFull())

      this.sendStylesheet()
    }

    /////////////////////////////////////////////////////////////////////////////
    // Module functions

    modAddStyleTo(target, c) {
      const el = this.modGetEl(target)
      if (el) {
        el.classList.add(c)
      } else {
        this.modLogError('Could not add class')
      }
    }

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

    modRemoveStyleFrom(target, c) {
      const el = this.modGetEl(target)
      if (el) {
        el.classList.remove(c)
      } else {
        this.modLogError('Could not remove class')
      }
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

    modUpdateHTML(target, value) {
      this.modUpdateAttrs(target, {
        innerHTML: value,
      })
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
