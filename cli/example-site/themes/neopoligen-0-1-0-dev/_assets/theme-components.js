customElements.define('code-block', 
  class extends HTMLElement {    

    template() {
      const template = this.ownerDocument.createElement('template') 
      template.innerHTML = `
<style>
.code {
  color: var(--code-block-alfa);
  background-color: var(--code-block-base);
}

.comment,
.meta.documentation {
  color: var(--code-block-bravo);
}
.string {
  color: var(--code-block-charlie);
}
.string.regexp {
  color: var(--code-block-charlie);
}
.constant.character.escape {
  color: var(--code-block-delta);
}
.constant.numeric {
  color: var(--code-block-echo);
}
.variable {
  color: var(--code-block-foxtrot);
}
.variable.function {
  color: var(--code-block-golf);
}
.variable.language {
  color: var(--code-block-hotel);
}
.keyword {
  color: var(--code-block-india);
}
.meta.import .keyword,
.keyword.control.import,
.keyword.control.import.from,
.keyword.other.import,
.keyword.control.at-rule.include,
.keyword.control.at-rule.import {
  color: var(--code-block-juliet);
}
.keyword.operator.comparison,
.keyword.operator.assignment,
.keyword.operator.arithmetic {
  color: var(--code-block-kilo);
}
.storage {
  color: var(--code-block-india);
}
.storage.modifier {
  color: var(--code-block-lima);
}
.keyword.control.class,
.entity.name,
.entity.name.class,
.entity.name.type.class {
  color: var(--code-block-golf);
}
.entity.other.inherited-class {
  color: var(--code-block-foxtrot);
}
.entity.other.attribute-name {
  color: var(--code-block-golf);
}
.support,
.support.type,
.support.class {
  color: var(--code-block-india);
}
.entity.name.function {
  color: var(--code-block-golf);
}
.punctuation.definition.variable {
  color: var(--code-block-india);
}
.constant,
.constant.language,
.meta.preprocessor {
  color: var(--code-block-golf);
}
.entity.name.section {
  color: var(--code-block-juliet);
}
.support.function.construct,
.keyword.other.new {
  color: var(--code-block-delta);
}
.constant.character,
.constant.other {
  color: var(--code-block-juliet);
}
.entity.name.tag {
  color: var(--code-block-foxtrot);
}
.punctuation.definition.tag.html,
.punctuation.definition.tag.begin,
.punctuation.definition.tag.end {
  color: var(--code-block-bravo);
}
.support.function {
  color: var(--code-block-india);
}
.punctuation.separator.continuation {
  color: var(--code-block-delta);
}
.storage.type {
  color: var(--code-block-foxtrot);
}
.support.type.exception {
  color: var(--code-block-juliet);
}
.keyword.other.special-method {
  color: var(--code-block-juliet);
}
.invalid {
  background-color: var(--code-block-mike);
}
.string.quoted.double,
.string.quoted.single {
  color: var(--code-block-charlie);
}
.punctuation.definition.string {
  color: var(--code-block-alfa);
}
.meta.brace.square,
.punctuation.section.brackets {
  color: var(--code-block-foxtrot);
}
.meta.brace.round,
.meta.brace.curly,
.punctuation.section,
.punctuation.section.block,
.punctuation.definition.parameters,
.punctuation.section.group {
  color: var(--code-block-kilo);
}
.support.constant.color,
.invalid.deprecated.color.w3c-non-standard-color-name.scss {
  color: var(--code-block-golf);
}
.meta.selector.css {
  color: var(--code-block-kilo);
}
.entity.name.tag.css,
.entity.name.tag.scss,
.source.less .keyword.control.html.elements,
.source.sass .keyword.control.untitled {
  color: var(--code-block-golf);
}
.entity.other.attribute-name.class {
  color: var(--code-block-golf);
}
.entity.other.attribute-name.id {
  color: var(--code-block-golf);
}
.entity.other.attribute-name.pseudo-element,
.entity.other.attribute-name.tag.pseudo-element,
.entity.other.attribute-name.pseudo-class,
.entity.other.attribute-name.tag.pseudo-class {
  color: var(--code-block-foxtrot);
}
.text.html.basic .meta.tag.other.html,
.text.html.basic .meta.tag.any.html,
.text.html.basic .meta.tag.block.any,
.text.html.basic .meta.tag.inline.any,
.text.html.basic .meta.tag.structure.any.html,
.text.html.basic .source.js.embedded.html,
.punctuation.separator.key-value.html {
  color: var(--code-block-kilo);
}
.text.html.basic .entity.other.attribute-name.html,
.meta.tag.xml .entity.other.attribute-name {
  color: var(--code-block-golf);
}
.keyword.other.special-method.ruby {
  color: var(--code-block-india);
}
.variable.other.constant.ruby {
  color: var(--code-block-golf);
}
.constant.other.symbol.ruby {
  color: var(--code-block-charlie);
}
.keyword.other.special-method.ruby {
  color: var(--code-block-juliet);
}
.meta.array .support.function.construct.php {
  color: var(--code-block-golf);
}
.entity.name.function.preprocessor.c,
.meta.preprocessor.c.include,
.meta.preprocessor.macro.c {
  color: var(--code-block-juliet);
}
.meta.preprocessor.c.include .string.quoted.other.lt-gt.include.c,
.meta.preprocessor.c.include .punctuation.definition.string.begin.c,
.meta.preprocessor.c.include .punctuation.definition.string.end.c {
  color: var(--code-block-charlie);
}
.other.package.exclude,
.other.remove {
  color: var(--code-block-delta);
}
.other.add {
  color: var(--code-block-charlie);
}
.punctuation.section.group.tex,
.punctuation.definition.arguments.begin.latex,
.punctuation.definition.arguments.end.latex,
.punctuation.definition.arguments.latex {
  color: var(--code-block-delta);
}
.meta.group.braces.tex {
  color: var(--code-block-golf);
}
.string.other.math.tex {
  color: var(--code-block-golf);
}
.variable.parameter.function.latex {
  color: var(--code-block-juliet);
}
.punctuation.definition.constant.math.tex {
  color: var(--code-block-delta);
}
.text.tex.latex .constant.other.math.tex,
.constant.other.general.math.tex,
.constant.other.general.math.tex,
.constant.character.math.tex {
  color: var(--code-block-charlie);
}
.string.other.math.tex {
  color: var(--code-block-golf);
}
.punctuation.definition.string.begin.tex,
.punctuation.definition.string.end.tex {
  color: var(--code-block-delta);
}
.keyword.control.label.latex,
.text.tex.latex .constant.other.general.math.tex {
  color: var(--code-block-charlie);
}
.variable.parameter.definition.label.latex {
  color: var(--code-block-delta);
}
.support.function.be.latex {
  color: var(--code-block-india);
}
.support.function.section.latex {
  color: var(--code-block-juliet);
}
.support.function.general.tex {
  color: var(--code-block-charlie);
}
.keyword.control.ref.latex {
  color: var(--code-block-charlie);
}
.storage.type.class.python,
.storage.type.function.python,
.storage.modifier.global.python {
  color: var(--code-block-india);
}
.support.type.exception.python {
  color: var(--code-block-golf);
}
.meta.scope.for-in-loop.shell,
.variable.other.loop.shell {
  color: var(--code-block-lima);
}
.meta.scope.case-block.shell,
.meta.scope.case-body.shell {
  color: var(--code-block-lima);
}
.punctuation.definition.logical-expression.shell {
  color: var(--code-block-delta);
}
.storage.modifier.c++ {
  color: var(--code-block-india);
}
.support.function.perl {
  color: var(--code-block-foxtrot);
}
.meta.diff,
.meta.diff.header {
  color: var(--code-block-bravo);
}
.meta.diff.range {
  color: var(--code-block-foxtrot);
}
.markup.deleted {
  color: var(--code-block-delta);
}
.markup.changed {
  color: var(--code-block-charlie);
}
.markup.inserted {
  color: var(--code-block-india);
}
.markup.heading,
.punctuation.definition.heading.markdown {
  color: var(--code-block-golf);
}
.markup.quote {
  color: var(--code-block-india);
}
.markup.italic {
  font-style: italic;
}
.markup.bold {
  font-weight: bold;
}
.markup.underline.link.markdown,
.meta.link.reference .constant.other.reference.link.markdown {
  color: var(--code-block-charlie);
}
.constant.other.reference.link.markdown {
  color: var(--code-block-echo);
}
.meta.paragraph.markdown .meta.dummy.line-break {
  background-color: var(--code-block-bravo);
}
.sublimelinter.notes {
  color: var(--code-block-bravo);
  background-color: var(--code-block-bravo);
}
.sublimelinter.outline.illegal {
  color: var(--code-block-bravo);
  background-color: var(--code-block-bravo);
}
.sublimelinter.underline.illegal {
  background-color: var(--code-block-delta);
}
.sublimelinter.outline.warning {
  color: var(--code-block-alfa);
  background-color: var(--code-block-alfa);
}
.sublimelinter.underline.warning {
  background-color: var(--code-block-golf);
}
.sublimelinter.outline.violation {
  color: var(--code-block-kilo);
  background-color: var(--code-block-kilo);
}
.sublimelinter.underline.violation {
  background-color: var(--code-block-juliet);
}
.sublimelinter.mark.warning {
  color: var(--code-block-golf);
}
.sublimelinter.mark.error {
  color: var(--code-block-delta);
}
.sublimelinter.gutter-mark {
  color: var(--code-block-kilo);
}
.brackethighlighter.all {
  color: var(--code-block-bravo);
}
.entity.name.filename.find-in-files {
  color: var(--code-block-charlie);
}
.constant.numeric.line-number.find-in-files {
  color: var(--code-block-bravo);
}
.markup.deleted.git_gutter {
  color: var(--code-block-delta);
}
.markup.inserted.git_gutter {
  color: var(--code-block-india);
}
.markup.changed.git_gutter {
  color: var(--code-block-golf);
}
.variable.other.readwrite.js,
.variable.other.object.js,
.variable.other.constant.js {
  color: var(--code-block-alfa);
}
</style>

<style>
*, 
*::before, 
*::after {
  box-sizing: border-box;
}

* {
  margin: 0;
}

.code-block-copy-button {
  position: absolute;
  right: 0;
  top: 0;
  background: none;
  color: var(--code-block-line-numbers);
  border: 1px solid var(--code-block-line-numbers);
  border-radius: 0.3rem;
}

.code-block-wrapper {
  position: relative;
  padding: 0.7rem;
  background-color: var(--code-block-base);
  border-radius: 0.3rem;
  border: 1px solid var(--code-block-border);
}

.numberedLines {
  counter-reset: lineNumber;
}

.numberedLine {
  counter-increment: lineNumber;
}

.numberedLine:before {
  display: inline-block;
  color: var(--code-block-line-numbers);
  content: counter(lineNumber);
  padding-right: 0.7rem;
  text-align: right;
  width: 2rem;
}
</style>

<h3></h3>
<div class="code-block-wrapper">
  <button class="code-block-copy-button">copy</button>
  <pre></pre>
</div>`

      return template
    }

    constructor() {
      super()
      this.attachShadow({ mode: 'open' })
    }

    addClickListener() {
      const el = this.shadowRoot.querySelector('.code-block-copy-button')
      el.addEventListener('click', (event) => this.handleClick.call(this, event))
    }

    connectedCallback() {
      this.content = this.template().content.cloneNode(true)
      this.shadowRoot.appendChild(this.content)
      this.addClickListener()
      const observer = new MutationObserver(
        (list, observer) => this.updateContent.call(this, list, observer)
      );
      observer.observe(this, {childList: true, subtree: true,})
    }

    updateContent() {
      const title = this.querySelector('h3')
      if (title) {
        this.shadowRoot.querySelector('h3').innerHTML = title.innerHTML
      } else {
        this.shadowRoot.querySelector('h3').innerHTML = 'code'
      }
      const code = this.querySelector('pre')
      if (code) {
        this.shadowRoot.querySelector('pre').innerHTML = code.innerHTML
      } 
    }

    async copyCode(button) {
      try {
        await navigator.clipboard.writeText(
          this.querySelector('pre').innerText
        )
        button.innerHTML = "Copied!"
      } catch (err) {
        button.innerHTML = "Error copying"
      }
    } 

    handleClick(event) {
      this.copyCode(event.target)
    }

  }
)



customElements.define(
  'settings-gear',
  class SettingsGear extends HTMLElement {
    constructor() {
      super()
      this.attachShadow({ mode: 'open' })
      this.addStyles()
      this.addGear()
      this.addWrapper()
      this.addColorModes()
      this.addCloseButton()
      this.setInitialMode()
      this.addListeners()
    }

    addCloseButton() {
      const closeButton = this.ownerDocument.createElement('button')
      closeButton.innerHTML = 'Close'
      closeButton.addEventListener('click', () => {
        this.gear.dataset.open = 'false'
        this.wrapper.hidden = true
      })
      this.wrapper.appendChild(closeButton)
    }

    addColorModes() {
      const colorModes = this.ownerDocument.createElement('div')
      colorModes.classList.add('color-modes')
      const buttons = [
        { mode: 'light', display: 'Light ☀' },
        { mode: 'dark', display: 'Dark ☾' },
        { mode: 'auto', display: 's' },
      ]
      buttons.forEach((button) => {
        const btn = this.ownerDocument.createElement('button')
        btn.dataset.mode = button.mode
        btn.setAttribute('role', 'ld-mode')
        btn.addEventListener('click', (event) => {
          this.handleClick.call(this, event)
        })
        if (button.mode !== 'auto') {
          btn.innerHTML = button.display
        }
        colorModes.appendChild(btn)
      })
      this.wrapper.appendChild(colorModes)
    }

    addGear() {
      this.gear = this.ownerDocument.createElementNS(
        'http://www.w3.org/2000/svg',
        `svg`
      )
      this.gear.innerHTML = `<path class="settings-gear" fill-rule="evenodd" clip-rule="evenodd" d="M9.99996 2C9.44768 2 8.99996 2.44772 8.99996 3V4.58178C8.30471 4.86318 7.65844 5.23923 7.07704 5.69365L5.70573 4.90193C5.47605 4.76932 5.20309 4.73338 4.94691 4.80202C4.69073 4.87067 4.47232 5.03827 4.33971 5.26795L2.33971 8.73205C2.06357 9.21034 2.22744 9.82193 2.70573 10.0981L4.07654 10.8895C4.02603 11.2528 3.99997 11.6236 3.99997 12C3.99997 12.3764 4.02603 12.7471 4.07654 13.1105L2.70574 13.9019C2.47605 14.0345 2.30846 14.2529 2.23981 14.5091C2.17117 14.7653 2.2071 15.0382 2.33971 15.2679L4.33971 18.732C4.47232 18.9617 4.69074 19.1293 4.94692 19.198C5.2031 19.2666 5.47605 19.2307 5.70574 19.0981L7.07706 18.3063C7.65846 18.7607 8.30472 19.1368 8.99996 19.4182V21C8.99996 21.5523 9.44768 22 9.99996 22H14C14.5522 22 15 21.5523 15 21V19.4182C15.6952 19.1368 16.3415 18.7607 16.9229 18.3063L18.2942 19.0981C18.5239 19.2307 18.7968 19.2666 19.053 19.198C19.3092 19.1293 19.5276 18.9617 19.6602 18.7321L21.6602 15.268C21.7928 15.0383 21.8288 14.7653 21.7601 14.5091C21.6915 14.253 21.5239 14.0345 21.2942 13.9019L19.9234 13.1105C19.9739 12.7472 20 12.3764 20 12C20 11.6236 19.9739 11.2528 19.9234 10.8895L21.2942 10.0981C21.7725 9.82191 21.9364 9.21032 21.6602 8.73203L19.6602 5.26793C19.5276 5.03824 19.3092 4.87065 19.053 4.802C18.7968 4.73336 18.5239 4.76929 18.2942 4.9019L16.9229 5.69364C16.3415 5.23922 15.6952 4.86318 15 4.58178V3C15 2.44772 14.5522 2 14 2H9.99996ZM11 5.28986V4H13V5.28986C13 5.73228 13.2907 6.12211 13.7147 6.24831C14.6258 6.51947 15.4475 7.00198 16.1223 7.64029C16.4436 7.94424 16.9264 8.00099 17.3095 7.77984L18.4282 7.13395L19.4282 8.866L18.3109 9.51107C17.9281 9.73205 17.7358 10.1781 17.8379 10.6081C17.9437 11.0538 18 11.5197 18 12C18 12.4803 17.9437 12.9462 17.8379 13.3919C17.7358 13.8219 17.9281 14.2679 18.3109 14.4889L19.4282 15.134L18.4282 16.866L17.3094 16.2201C16.9264 15.999 16.4436 16.0557 16.1222 16.3597C15.4475 16.998 14.6258 17.4805 13.7147 17.7516C13.2907 17.8778 13 18.2677 13 18.7101V20H11V18.7101C11 18.2677 10.7092 17.8778 10.2852 17.7516C9.37409 17.4805 8.55246 16.998 7.87767 16.3597C7.55635 16.0557 7.07352 15.999 6.69048 16.2201L5.57176 16.866L4.57176 15.134L5.68905 14.4889C6.0718 14.2679 6.26409 13.8219 6.16201 13.3919C6.05621 12.9462 5.99997 12.4803 5.99997 12C5.99997 11.5197 6.0562 11.0538 6.16201 10.6081C6.26409 10.1781 6.07179 9.73207 5.68905 9.51109L4.57176 8.86603L5.57176 7.13398L6.69046 7.77986C7.07351 8.00101 7.55633 7.94425 7.87766 7.6403C8.55245 7.00199 9.37409 6.51948 10.2852 6.24831C10.7092 6.12211 11 5.73228 11 5.28986ZM9.99998 12C9.99998 10.8954 10.8954 10 12 10C13.1046 10 14 10.8954 14 12C14 13.1046 13.1046 14 12 14C10.8954 14 9.99998 13.1046 9.99998 12ZM12 8C9.79084 8 7.99998 9.79086 7.99998 12C7.99998 14.2091 9.79084 16 12 16C14.2091 16 16 14.2091 16 12C16 9.79086 14.2091 8 12 8Z" fill="#000000"/>`
      this.gear.setAttribute('width', '30px')
      this.gear.setAttribute('height', '30px')
      this.gear.dataset.open = 'false'
      this.gear.addEventListener('click', (event) => {
        this.toggleGear.call(this, event)
      })
      this.shadowRoot.appendChild(this.gear)
    }

    addListeners() {
      window
        .matchMedia('(prefers-color-scheme: dark)')
        .addEventListener('change', () => {
          this.updateAutoDisplay.call(this)
        })
    }

    addStyles() {
      const styles = this.ownerDocument.createElement('style')
      styles.innerHTML = `
.color-modes {
    max-width: 300px;
    
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

.settings-gear {
    fill: var(--color-charlie, currentColor);
}
`
      this.shadowRoot.appendChild(styles)
    }

    addWrapper() {
      this.wrapper = this.ownerDocument.createElement('div')
      this.wrapper.hidden = true
      this.wrapper.setAttribute('role', 'settings')
      this.shadowRoot.appendChild(this.wrapper)
    }

    handleClick(event) {
      this.setMode(event.target.dataset.mode)
    }

    setInitialMode() {
      this.updateAutoDisplay.call(this)
      const mode = localStorage.getItem('colorMode')
      if (mode) {
        this.setMode(mode)
      } else {
        this.setMode('auto')
      }
    }

    setMode(mode) {
      localStorage.setItem('colorMode', mode)
      if (mode === `auto`) {
        this.ownerDocument.body.classList.remove('light')
        this.ownerDocument.body.classList.remove('dark')
      } else {
        const removeMode = mode === 'light' ? 'dark' : 'light'
        this.ownerDocument.body.classList.add(mode)
        this.ownerDocument.body.classList.remove(removeMode)
      }
      const buttons = this.shadowRoot.querySelectorAll(`[role="ld-mode"]`)
      buttons.forEach((button) => {
        if (button.dataset.mode === mode) {
          button.setAttribute('aria-selected', true)
        } else {
          button.setAttribute('aria-selected', false)
        }
      })
    }

    toggleGear(event) {
      const gear = event.target
      if (gear.dataset.open === 'true') {
        gear.dataset.open = 'false'
        this.wrapper.hidden = true
      } else {
        gear.dataset.open = 'true'
        this.wrapper.hidden = false
      }
    }

    updateAutoDisplay() {
      const els = this.shadowRoot.querySelectorAll(
        '[role="ld-mode"][data-mode="auto"]'
      )
      els.forEach((el) => {
        if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
          el.innerHTML = 'Auto (☾)'
        } else {
          el.innerHTML = 'Auto (☀)'
        }
      })
    }
  }
)

customElements.define(
  'tab-group',
  class TabGroup extends HTMLElement {
    get tabs() {
      return [...this.querySelectorAll('[role=tab]')]
    }

    get panels() {
      return [...this.querySelectorAll('[role=tabpanel]')]
    }

    get selected() {
      return this.querySelector('[role=tab][aria-selected=true]')
    }

    set selected(element) {
      this.selected?.setAttribute('aria-selected', 'false')
      element?.setAttribute('aria-selected', 'true')
      element?.focus()
      this.updateSelection()
    }

    connectedCallback() {
      this.generateIds()
      this.updateSelection()
      this.setupEvents()
    }

    generateIds() {
      const prefix = Math.floor(Date.now()).toString(36)
      this.tabs.forEach((tab, index) => {
        const panel = this.panels[index]
        tab.id ||= `${prefix}-tab-${index}`
        panel.id ||= `${prefix}-panel-${index}`
        tab.setAttribute('aria-controls', panel.id)
        panel.setAttribute('aria-labelledby', tab.id)
      })
    }

    updateSelection() {
      this.tabs.forEach((tab, index) => {
        const panel = this.panels[index]
        const isSelected = tab.getAttribute('aria-selected') === 'true'
        tab.setAttribute('aria-selected', isSelected ? 'true' : 'false')
        tab.setAttribute('tabindex', isSelected ? '0' : '-1')
        panel.setAttribute('tabindex', isSelected ? '0' : '-1')
        panel.hidden = !isSelected
      })
    }

    setupEvents() {
      this.tabs.forEach((tab) => {
        tab.addEventListener('click', () => (this.selected = tab))
        tab.addEventListener('keydown', (e) => {
          if (e.key === 'ArrowLeft') {
            this.selected = tab.previousElementSibling ?? this.tabs.at(-1)
          } else if (e.key === 'ArrowRight') {
            this.selected = tab.nextElementSibling ?? this.tabs.at(0)
          }
        })
      })
    }
  }
)
