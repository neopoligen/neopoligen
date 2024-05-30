customElements.define('code-block2', 
  class extends HTMLElement {    
    constructor() {
      super()
      this.attachShadow({ mode: 'open' })
    }

    addClickListener() {
      const el = this.shadowRoot.querySelector('::part(copy-button)')
      el.addEventListener('click', this.handleClick)
    }

    connectedCallback() {
      console.log('x')
      const template = this.ownerDocument.createElement('template') 
      template.innerHTML = `
<style>
::part(wrapper) {
  position: relative;
}
::part(copy-button) {
  position: absolute;
  right: 0;
}
</style>
<div part="wrapper">
  <button part="copy-button">Copy</button>
  <pre><code>
  <slot name="the-code">xxx<slot>
  </code></pre>
</div>`
      this.shadowRoot.appendChild(template.content.cloneNode(true))
      //this.addClickListener()
    }

    async copyCode(button) {
      try {
        await navigator.clipboard.writeText(
          this.shadowRoot.querySelector('slot[name=the-code]').innerHTML
        )
        button.innerHTML = "Copied!"
      } catch (err) {
        button.innerHTML = "Error copying"
      }
    } 

    handleClick(event) {
      console.log(event)
      this.copyCode(event.target)
    }

  }
)


customElements.define('neo-admin', 
  class extends HTMLElement {    
    constructor() {
      super()
      this.attachShadow({ mode: 'open' })
    }

    connectedCallback() {
      const domain = window.location.origin
      if (domain === "http://localhost:1989") {
        const template = this.ownerDocument.createElement('template') 
        template.innerHTML = this.templateContent()
        this.shadowRoot.appendChild(template.content.cloneNode(true))

        let designModeButton = this.shadowRoot.querySelector(".design-mode-button")
        this.designModeStatus = localStorage.getItem("designMode")
        if (this.designModeStatus) {
          designModeButton.innerHTML = `Design Mode: ${this.designModeStatus}`
        } else {
          this.designModeStatus = "off"
          designModeButton.innerHTML = `Design Mode: off`
        }
        if (designModeButton) {
          designModeButton.addEventListener(
            "click", 
            (event) => {
              this.toggleDesignMode.call(this)
            }
          )
        }
      }
    }

    templateContent() {
      const content = `
<header>
  <button class="design-mode-button">Design Mode</button>
</header>`
        return content
    }

    toggleDesignMode(event) {
      console.log("asdf")
      if (this.designModeStatus === "off") {
        this.designModeStatus = "on"
      } else {
        this.designModeStatus = "off"
      }
      console.log(this.designModeStatus)
      let button = this.shadowRoot.querySelector(".design-mode-button")
      if (button) {
        button.innerHTML = `Design Mode: ${this.designModeStatus}`
      }
      this.ownerDocument.designMode = this.designModeStatus
    }
})

