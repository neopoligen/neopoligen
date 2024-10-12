const schemes = ["auto", "light", "dark", "black", "white"]

function addCopyButtons() {
  const highlightWrappers = document.querySelectorAll('.highlight-wrapper')
  highlightWrappers.forEach((wrapper, indx) => {
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
  })
}

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
  //duplicateDarkStyles() - currently out since you need to duplicate more than :root
  addCopyButtons()
  makeContentVisible()
})
