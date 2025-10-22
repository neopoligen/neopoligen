const state = {};

export default class {
  bittyInit() {
    // TODO: Add the css class switch here.
  }

  async copyText(event, _element) {
    if (!state.textCopy) {
      state.textCopy = {};
    }
    const ds = event.target.dataset;
    const uuid = ds.uuid;
    console.log(`Copy from: ${uuid}`);
    state.textCopy[uuid] = {};
    const elToCopy = document.querySelector(ds.target);
    if (!state.textCopy[uuid].originalText) {
      state.textCopy[uuid].originalText = event.target.innerHTML;
    }
    try {
      if (elToCopy.value) {
        state.textCopy[uuid].value = elToCopy.value;
      } else {
        state.textCopy[uuid].value = elToCopy.innerText;
      }
      await navigator.clipboard.writeText(state.textCopy[uuid].value);
      event.target.innerHTML = "Copied";
    } catch (err) {
      event.target.innerHTML = "Could Not Copy";
    }
    await new Promise((resolve) => setTimeout(resolve, 2000));
    event.target.innerHTML = state.textCopy[uuid].originalText;
    delete state.textCopy[uuid];
  }
}
