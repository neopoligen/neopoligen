const schemes = ["auto", "light", "dark", "black", "white"]

class AwsCodeBlock extends HTMLElement {
    connectedCallback() {
        this.pre = this.querySelector('pre');
        if (this.pre !== null) {
            this.minHeight = 0;
            this.addButtonsDiv();
            this.makeToggleWrapButton();
            this.makeReduceButton();
            this.makeEnlargeButton();
            this.makeCopyButton();
            this.setMinHeight();
        }
    }

    addButtonsDiv() {
        this.buttonsDiv = document.createElement("div");
        this.buttonsDiv.classList.add("aws-code-block-buttons");
        this.appendChild(this.buttonsDiv);
    }

    checkForWrap() {
        return true;
        /*  
        const compareEl = document.createElement('pre');
        const styles = window.getComputedStyle(this.pre);
        const cssText = Object.values(styles).reduce(
            (css, propertyName) => 
                `${css}${propertyName}:${styles.getPropertyValue(
                    propertyName
            )};`
        );
        compareEl.style.cssText = cssText;
        compareEl.innerHTML = this.pre.innerHTML;
        compareEl.style.position = 'absolute';
        compareEl.style.top = 0;
        compareEl.style.left = 0;
        compareEl.style.whiteSpace = 'nowrap';
        compareEl.style.visibility = 'hidden';
        console.log(compareEl);
        this.appendChild(compareEl);
        const isWrapping = 
            compareEl.scrollWidth > this.pre.offsetWidth 
            ? true : false;
        // compareEl.remove();
        return isWrapping;
        */
    }


    async copyCode() {
        try {
          await navigator.clipboard.writeText(this.pre.innerText);
          this.copyButton.innerHTML = 'Copied';
        } catch (err) {
          this.copyButton.innerHTML = 'Copy Failed';
        }
        setTimeout(() => 
            {this.copyButton.innerHTML = 'Copy Code'}, 
            1200
        );
    }

    enlargeFont() {
        this.pre.style.fontSize = `${this.getFontSize() * 1.05}px`;
        this.setMinHeight();
    }

    getFontSize() {
        const styles = window.getComputedStyle(this.pre);
        const size = parseFloat(
            styles.getPropertyValue('font-size')
        );
        return size;
    }

    makeCopyButton() {
        this.copyButton = document.createElement("button");
        this.copyButton.innerHTML = "Copy Code";
        this.copyButton.addEventListener(
            "click", 
            () => { this.copyCode(); }
        );
        this.buttonsDiv.appendChild(this.copyButton);
        const copyButtonRect = this.copyButton.getBoundingClientRect();
        // TODO: Figure out why adding 20px here is necessary to
        // keep the button text from wrapping.
        this.copyButton.style.width = `${copyButtonRect.width + 20}px`;
    }

    makeEnlargeButton() {
        this.enlargeButton = document.createElement("button");
        this.enlargeButton.innerHTML = "Enlarge Font";
        this.enlargeButton.addEventListener(
            "click", 
            () => { this.enlargeFont(); }
        );
        this.buttonsDiv.appendChild(this.enlargeButton);
    }

    makeReduceButton() {
        this.reduceButton = document.createElement("button");
        this.reduceButton.innerHTML = "Reduce Font";
        this.reduceButton.addEventListener(
            "click", 
            () => { this.reduceFont(); }
        );
        this.buttonsDiv.appendChild(this.reduceButton);
    }

    makeToggleWrapButton() {
        if (this.checkForWrap()) {
            this.wrapState = "On";
            this.toggleWrapButton = document.createElement("button");
            this.toggleWrapButton.innerHTML = 'Turn Wrapping Off';
            this.toggleWrapButton.addEventListener(
                "click", 
                () => { this.toggleWrap() }
            );
            this.buttonsDiv.appendChild(this.toggleWrapButton);
            const toggleWrapButtonRect = 
                this.toggleWrapButton.getBoundingClientRect();
            // NOTE: Adding 20 pixels here. I'm not sure why
            // that's necessary, but without it the button
            // changes size when changing the text in this
            // example
            this.toggleWrapButton.style.minWidth = 
                `${toggleWrapButtonRect.width + 20}px`;
        }
    }

    reduceFont() {
        this.pre.style.fontSize = `${this.getFontSize() * 0.95}px`;
    }

    setMinHeight() {
        // TODO: Figure out why this is adding an extra lines
        // worth of space to the bottom of the pre element
        // in this example.
        const preRect = this.pre.getBoundingClientRect();
        if (this.minHeight < preRect.height) {
            this.minHeight = preRect.height;
            this.pre.style.minHeight = `${preRect.height}px`;
        }
    }

    toggleWrap() {
        this.pre.classList.toggle("no-wrapping");
        this.toggleWrapButton.innerHTML = 
            `Turn Wrapping ${this.wrapState}`;
        this.wrapState = this.wrapState === "On" ? "Off" : "On";
        this.setMinHeight();
    }

}

customElements.define("aws-code-block", AwsCodeBlock);


/*

function addCopyButtons() {
  const highlightWrappers = document.querySelectorAll('.highlight-wrapper')
  highlightWrappers.forEach((wrapper, indx) => {
    if (wrapper.classList.contains('no-buttons') === false) {
      const dataId = `highlight-block-${indx}`
      wrapper.dataset.highlightblock = dataId
      const copyButton = document.createElement('button')
      copyButton.innerHTML = 'Copy This Content'
      copyButton.classList.add('highlight-copy-button')
      copyButton.dataset.highlighttarget = dataId
      copyButton.addEventListener('click', async (event) => {
        const el = event.target
        const blockId = el.dataset.highlighttarget
        const preEl = document.querySelector(
          `[data-highlightblock="${blockId}"] pre`
        )
        try {
          await navigator.clipboard.writeText(preEl.innerText)
          el.innerHTML = 'Copied'
        } catch (err) {
          el.innerHTML = 'Error copying'
        }
        setTimeout(
          (theButton) => {theButton.innerHTML = 'Copy This Content'}, 2000, el
        )
      })
      wrapper.appendChild(copyButton)
    }
  })
}


function addWrapButtons() {
  const highlightWrappers = document.querySelectorAll('.highlight-wrapper')
  highlightWrappers.forEach((wrapper, indx) => {
    if (wrapper.classList.contains('no-buttons') === false) {
      wrapper.dataset.wrapstate = "on" 
      const dataId = `wrap-block-${indx}`
      wrapper.dataset.wrapblock = dataId
      const wrapButton = document.createElement('button')
      wrapButton.innerHTML = 'Toggle Wrapping'
      wrapButton.classList.add('highlight-copy-button')
      wrapButton.dataset.wraptarget = dataId
      wrapButton.addEventListener('click', async (event) => {
        const el = event.target
        const targetId = el.dataset.wraptarget
        const theWrapper = document.querySelector(
          `[data-wrapblock="${targetId}"]`
        )
        const thePre = theWrapper.querySelector(
          `pre`
        )
        if (theWrapper.dataset.wrapstate === "on") {
          theWrapper.dataset.wrapstate = "off"
          thePre.style.whiteSpace = "pre"
          thePre.style.overflowWrap ="normal"
          thePre.style.overflowX ="auto"
          thePre.style.overscrollBehaviorX = "none"
        } else {
          theWrapper.dataset.wrapstate = "on"
          thePre.style.whiteSpace = "pre-wrap"
          thePre.style.overflowWrap ="break-word"
          thePre.style.overflowX ="visible"
          thePre.style.overscrollBehaviorX = "auto"
        }
      })
      wrapper.appendChild(wrapButton)
    }
  })
}

*/

function addSchemeSwitchers() {
  const switchers = document.querySelectorAll(".color-scheme-switcher")
  switchers.forEach((switcher, num) => {
    const fieldSet = document.createElement("fieldset")
    fieldSet.classList.add("color-scheme-list")
    const legend = document.createElement("legend")
    legend.innerHTML = "Color Scheme"
    fieldSet.appendChild(legend)
    schemes.forEach((scheme) => {
      if (scheme === "auto" && !hasSystem()) {
        // skip system if there isn't data for it
      } else {
        const schemeLabel = document.createElement("label")
        schemeLabel.htmlFor = `scheme-switcher-${scheme}-${num}`
        schemeLabel.innerHTML = `${scheme} `
        const schemeButton = document.createElement("input")
        schemeButton.type = "radio"
        schemeButton.name = `scheme-switcher-${num}`
        schemeButton.id = `scheme-switcher-${scheme}-${num}`
        schemeButton.value = scheme
        schemeButton.dataset.num = num
        if (currentSchemer() === scheme) {
          schemeButton.checked = true 
        }
        schemeButton.addEventListener("input", switchSchemer)
        schemeLabel.appendChild(schemeButton)
        fieldSet.appendChild(schemeLabel)
      }
    })
    switcher.appendChild(fieldSet)
  })
}

function duplicateDarkStyles() {
  for (let sheetNum = 0; sheetNum < document.styleSheets.length; sheetNum++) {
    const sheet = document.styleSheets[sheetNum]
    for (let ruleNum = 0; ruleNum < sheet.cssRules.length; ruleNum++) {
      const rule = sheet.cssRules[ruleNum]
      if (rule.conditionText === "(prefers-color-scheme: dark)") {
        for (let subNum = 0; subNum < rule.cssRules.length; subNum++) {
          const subRule = rule.cssRules[subNum]
          if (subRule.selectorText === ":root") {
            const ruleString = subRule
            const parsedString = ruleString.cssText.replace(subRule.selectorText, "")
            sheet.insertRule(`[data-scheme="dark"] ${parsedString}`, sheet.cssRules.length)
          }
        }
      }
    }
  }
}

function makeContentVisible() {
  const showSheet = document.createElement("style")
  showSheet.innerHTML = `html { visibility: visible };`
  document.body.appendChild(showSheet)
}

function switchSchemer(event) {
  const newSchemer = event.target.value
  localStorage.setItem("schemer", newSchemer)
  const switcherNum = parseInt(event.target.dataset.num, 10)
  const switchers = document.querySelectorAll(".scheme-switcher")
  switchers.forEach((switcher, num) => {
    schemes.forEach((scheme) => {
      if (switcherNum !== num) {
        const el = document.querySelector(`#scheme-switcher-${scheme}-${num}`)
        if (el) {
          if (newSchemer === scheme) {
            el.checked = true
          } else {
            el.checked = false
          }
        }
      }
    })
  })
  updateScheme()
}

function updateScheme() {
  if (currentSchemer() === "auto") {
    document.body.dataset.scheme = "auto"
  } else {
    document.body.dataset.scheme = currentScheme()
  }
}

/*
function addColorModeSwitcher() {
  const wrapper = document.querySelector("#color-mode-wrapper")
  const darkLabel = document.createElement("label")
  darkLabel.for = "color-mode-dark"
  darkLabel.innerHTML = "Dark "
  const darkButton = document.createElement("input")
  darkButton.type = "radio"
  darkButton.name = "color-mode"
  darkButton.id = "color-mode-dark"
  darkButton.value = "dark"
  wrapper.appendChild(darkLabel)
  wrapper.appendChild(darkButton)
  const lightLabel = document.createElement("label")
  lightLabel.for = "color-mode-light"
  lightLabel.innerHTML = "Light "
  const lightButton = document.createElement("input")
  lightButton.type = "radio"
  lightButton.name = "color-mode"
  lightButton.id = "color-mode-light"
  lightButton.value = "light"
  wrapper.appendChild(lightLabel)
  wrapper.appendChild(lightButton)
  const currentColorMode = localStorage.getItem("colorMode")
  if (currentColorMode === "light") {
    lightButton.checked = true 
  } else {
    darkButton.checked = true
  }
  darkButton.addEventListener("change", switchColorMode)
  lightButton.addEventListener("change", switchColorMode)
}
*/

/*
function makeHtmlVisible() {
  const newSheet = document.createElement("link");
  newSheet.rel  = "stylesheet";
  newSheet.href = "/theme/styles/make-html-visible.css"
  document.querySelector("body").appendChild(newSheet)
}
*/

/*
function switchColorMode(event) {
  const value = event.target.value
  console.log(`Switch is now ${value}`)
  localStorage.setItem("colorMode", value)
  var link = document.createElement( "link" )
  link.href = `/theme/styles/variables-${value}.css`
  link.rel = "stylesheet"
  document.querySelector("body").appendChild(link)
}
*/

document.addEventListener('DOMContentLoaded', () => {
  addSchemeSwitchers()
  updateScheme()
  // duplicateDarkStyles() - currently out since you need to duplicate more than :root
  // addWrapButtons()
  // addCopyButtons()
  makeContentVisible()
})
