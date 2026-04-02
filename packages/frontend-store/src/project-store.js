export class ProjectStore {
  constructor(options = {}) {
    this.adapter = options.adapter;
    this.state = null;
    this.snapshotPath = options.snapshotPath ?? "target/demo-ui/snapshot.json";
    this.eventLogPath = options.eventLogPath ?? "target/demo-ui/event-log.json";
    this.listeners = new Set();
    this.lastResponse = null;
  }

  getState() {
    return this.state;
  }

  subscribe(listener) {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  }

  notify() {
    for (const listener of this.listeners) {
      listener(this.state, this.lastResponse);
    }
  }

  updateFromResponse(response) {
    this.lastResponse = response;
    if (response?.state) {
      this.state = response.state;
    }
    this.notify();
    return this.state;
  }

  async createProject({ projectId, name, aspectRatio }) {
    const response = await this.adapter.createProjectDocument({
      projectId,
      name,
      aspectRatio,
      paths: this.paths(),
    });
    return this.updateFromResponse(response);
  }

  async load() {
    const response = await this.adapter.getDocumentState(this.paths());
    return this.updateFromResponse(response);
  }

  async dispatch(commands, options = {}) {
    const response = await this.adapter.applyCommands(commands, {
      save: options.save ?? true,
      paths: this.paths(),
    });
    return this.updateFromResponse(response);
  }

  async selectClips({ clipIds = [], trackId = null }) {
    return this.dispatch([
      {
        type: "set_selection",
        clip_ids: clipIds,
        track_id: trackId,
      },
    ]);
  }

  async setPlayhead(playheadMs) {
    return this.dispatch([
      {
        type: "set_playhead",
        playhead_ms: playheadMs,
      },
    ]);
  }

  async setViewport(viewport) {
    return this.dispatch([
      {
        type: "set_viewport",
        viewport,
      },
    ]);
  }

  async undo() {
    return this.dispatch([{ type: "undo" }]);
  }

  async redo() {
    return this.dispatch([{ type: "redo" }]);
  }

  paths() {
    return {
      snapshotPath: this.snapshotPath,
      eventLogPath: this.eventLogPath,
    };
  }
}
