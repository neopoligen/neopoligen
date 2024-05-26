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

