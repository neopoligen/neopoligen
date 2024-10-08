function addCodeCopyButtons() {
  // console.log('Adding code copy buttons')
  const codeExamples = document.querySelectorAll('.code-button-wrapper')

  codeExamples.forEach((example, index) => {
    const dataId = `code-block-${index}`
    example.dataset.codeblock = dataId
    const copyButton = document.createElement('button')
    copyButton.innerHTML = 'Copy'
    copyButton.classList.add('code-button')
    copyButton.dataset.codeblockbutton = dataId
    copyButton.addEventListener('click', async (event) => {
      const el = event.target
      const blockId = el.dataset.codeblockbutton
      // console.log(`Copy code block: ${blockNum}`)
      const codePreEl = document.querySelector(
        `[data-codeblock="${blockId}"] pre`
      )
      try {
        await navigator.clipboard.writeText(codePreEl.innerText)
        el.innerHTML = 'Copied'
      } catch (err) {
        el.innerHTML = 'Error copying'
      }
      setTimeout(
        (theButton) => {
          theButton.innerHTML = 'Copy'
        },
        2000,
        el
      )
    })
    example.appendChild(copyButton)
  })

}

document.addEventListener('DOMContentLoaded', () => {
  addCodeCopyButtons()
})
