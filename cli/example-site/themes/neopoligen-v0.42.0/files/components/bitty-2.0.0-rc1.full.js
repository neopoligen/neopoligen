const version = [2, 0, 0, "rc1"];
const tagName = `bitty-${version[0]}-${version[1]}`;
const blockStylesheet = new CSSStyleSheet();
blockStylesheet.replaceSync(
  `${tagName} { display: block; }`,
);
document.adoptedStyleSheets.push(blockStylesheet);

function getUUID() {
  return self.crypto.randomUUID();
}

class BittyJs extends HTMLElement {
  constructor() {
    super();
    this.config = {
      listeners: ["click", "input"],
    };
    this.metadata = {
      copyright: "Copyright 2025 - Alan W. Smith",
      license:
        "License at: htttp://bitty.alanwsmith.com/ - 2y1pBoEREr3eWA1ubCCOXdmRCdn",
      version: version,
    };
  }

  async connectedCallback() {
    this.dataset.uuid = getUUID();
    this.receivers = [];
    this.setIds();
    this.url = new URL(window.location.href);
    await this.makeConnection();
    if (this.conn) {
      this.conn.api = this;
      this.handleEventBridge = this.handleEvent.bind(this);
      this.watchMutations = this.handleMutations.bind(this);
      this.loadReceivers();
      this.addObserver();
      this.addEventListeners();
      await this.callBittyInit();
      this.runSendFromComponent();
    }
  }

  addEventListeners() {
    if (this.dataset.listeners) {
      this.config.listeners = this.dataset.listeners.split("|").map((l) =>
        l.trim()
      );
    }
    this.config.listeners.forEach((listener) => {
      document.addEventListener(listener, (event) => {
        if (
          event.target &&
          event.target.nodeName.toLowerCase() !== tagName &&
          event.target.dataset && event.target.dataset.send
        ) {
          event.uuid = getUUID();
          this.handleEventBridge.call(this, event);
        }
      });
    });
  }

  addObserver() {
    this.observerConfig = { childList: true, subtree: true };
    this.observer = new MutationObserver(this.watchMutations);
    this.observer.observe(this, this.observerConfig);
  }

  addReceiver(signal, el) {
    if (this.conn[signal]) {
      this.receivers.push({
        key: signal,
        f: (event) => {
          this.conn[signal](event, el);
        },
      });
    }
  }

  async callBittyInit() {
    if (typeof this.conn.bittyInit === "function") {
      if (this.conn.bittyInit[Symbol.toStringTag] === "AsyncFunction") {
        await this.conn.bittyInit();
      } else {
        this.conn.bittyInit();
      }
    }
  }

  async fetchHTML(url, subs = [], options = {}) {
    const el = document.createElement("template");
    el.innerHTML = await this.fetchTxt(url, subs, options);
    return el.content.cloneNode(true);
  }

  async fetchJSON(url, subs = [], options = {}) {
    let content = await this.fetchTxt(url, subs, options);
    return JSON.parse(content);
  }

  async fetchLines(url, subs = [], options = {}) {
    const content = await this.fetchTxt(url, subs, options);
    return content.split("\n").map((line) => line.trim());
  }

  async fetchSVG(url, subs = [], options = {}) {
    const el = document.createElement("template");
    el.innerHTML = await this.fetchTxt(url, subs, options);
    return el.content.cloneNode(true);
  }

  async fetchTemplate(url, subs = [], options = {}) {
    return await this.fetchTxt(url, subs, options);
  }

  async fetchTxt(url, subs = [], options = {}) {
    let response = await fetch(url, options);
    try {
      if (!response.ok) {
        throw new Error(`${response.status} [${response.statusText}] - ${url}`);
      } else {
        let content = await response.text();
        subs.forEach((sub) => {
          content = content.replaceAll(sub[0], sub[1]);
        });
        return content;
      }
    } catch (error) {
      console.error(`fetchJson Error [${url}] - ${error}`);
      return undefined;
    }
  }

  // TODO: See about adding async/await here
  forward(event, signal) {
    if (!event || !event.target || !event.target.dataset) {
      event = {
        type: "bittyforward",
        target: { dataset: { forward: signal } },
      };
    }
    event.target.dataset.forward = signal;
    this.handleEvent(event);
  }

  handleEvent(event) {
    let signals = null;
    if (event.target.dataset.forward) {
      signals = event.target.dataset.forward;
      delete event.target.dataset.forward;
    } else {
      signals = event.target.dataset.send;
    }
    this.processSignals(event, signals);
  }

  handleMutations(mutationList, _observer) {
    for (const mutation of mutationList) {
      if (mutation.type === "childList") {
        if (
          mutation.addedNodes.length > 0 || mutation.removedNodes.length > 0
        ) {
          this.setIds();
          this.loadReceivers();
        }
      }
    }
  }

  async loadCSS(url, subs = [], options = {}) {
    const content = await this.fetchTxt(url, subs, options);
    const sheet = new CSSStyleSheet();
    sheet.replaceSync(content);
    document.adoptedStyleSheets.push(sheet);
    return sheet;
  }

  loadReceivers() {
    this.receivers = [];
    this.querySelectorAll(`[data-receive]`).forEach((el) => {
      el.dataset.receive.split("|").map((signal) => signal.trim()).forEach(
        (signal) => {
          this.addReceiver(signal, el);
        },
      );
    });
  }

  async makeConnection() {
    try {
      if (!this.dataset.connect) {
        if (window.BittyClass) {
          this.conn = new window.BittyClass();
        } else {
          console.error(`${tagName} error: No class to connect to.`);
        }
      } else {
        const connParts = this.dataset.connect.split("|").map((x) => x.trim());
        if (
          typeof window[connParts[0]] !== "undefined"
        ) {
          this.conn = new window[connParts[0]]();
        } else {
          const mod = await import(connParts[0]);
          if (connParts[1] === undefined) {
            this.conn = new mod.default();
          } else {
            this.conn = new mod[connParts[1]]();
          }
        }
      }
    } catch (error) {
      console.error(`${tagName} error: ${error} - ${this.dataset.connect}`);
    }
  }

  match(event, el, key = "") {
    if (key === "") {
      key = "uuid";
    }
    if (
      event.target.dataset[key] === undefined || el.dataset[key] === undefined
    ) {
      return false;
    }
    return event.target.dataset[key] === el.dataset[key];
  }

  processSignals(event, signals) {
    signals.split("|").map((signal) => signal.trim()).forEach((signal) => {
      let receiverCount = 0;
      this.receivers.forEach((receiver) => {
        if (receiver.key === signal) {
          receiverCount += 1;
          receiver.f(event);
        }
      });
      if (receiverCount === 0) {
        if (this.conn[signal]) {
          this.conn[signal](event, null);
        }
      }
    });
  }

  runSendFromComponent() {
    if (this.dataset.send) {
      this.handleEvent(
        { type: "bittytagdatasend", uuid: getUUID(), target: this },
      );
    }
  }

  setIds() {
    this.querySelectorAll("*").forEach((el) => {
      if (!el.dataset.uuid) {
        el.dataset.uuid = getUUID();
      }
    });
  }

  useTemplate(content, subs = []) {
    subs.forEach((sub) => {
      content = content.replaceAll(sub[0], sub[1]);
    });
    const el = document.createElement("template");
    el.innerHTML = content;
    return el.content.cloneNode(true);
  }
}

customElements.define(tagName, BittyJs);
