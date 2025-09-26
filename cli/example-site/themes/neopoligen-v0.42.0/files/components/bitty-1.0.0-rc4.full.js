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
      version: [1, 0, 0, "rc4"],
    };
  }

  async connectedCallback() {
    this.dataset.uuid = getUUID();
    this.receivers = [];
    this.setIds();
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
          event.target.nodeName.toLowerCase() !== "bitty-js" &&
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

  error(message) {
    console.error(`bitty-js error: ${message} on element ${this.dataset.uuid}`);
  }

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
    if (event.target.dataset.forward) {
      this.processSignals(event.target.dataset.forward);
      delete event.target.dataset.forward;
    } else if (event.target.dataset.send) {
      this.processSignals(event.target.dataset.send);
    }
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

  loadReceivers() {
    this.receivers = [];
    this.querySelectorAll(`[data-receive]`).forEach((el) => {
      el.dataset.receive.split("|").forEach((signal) => {
        this.addReceiver(signal, el);
      });
    });
  }

  async makeConnection() {
    try {
      if (!this.dataset || !this.dataset.connect) {
        this.error("Missing data-connect attribute");
        return;
      }
      if (
        window.bittyClasses &&
        typeof window.bittyClasses[this.dataset.connect] !== "undefined"
      ) {
        this.connPath = null;
        this.connClass = this.dataset.connect;
        this.conn = new window.bittyClasses[this.connClass]();
      } else {
        const connectionParts = this.dataset.connect.split("|");
        this.connPath = connectionParts[0];
        const mod = await import(this.connPath);
        if (connectionParts[1] === undefined) {
          this.connClass = "default";
          this.conn = new mod.default();
        } else {
          this.connClass = connectionParts[1];
          this.conn = new mod[this.connClass]();
        }
      }
    } catch (error) {
      this.error(error);
    }
  }

  processSignals(signals) {
    signals.split("|").forEach((signal) => {
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
        { type: "bittysend", uuid: getUUID(), target: this },
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
}

customElements.define("bitty-v1-rc4", BittyJs);
