export class BrowserMediaCoreAdapter {
  constructor(options = {}) {
    this.endpoint = options.endpoint ?? "/api/adapter";
  }

  async send(request) {
    const response = await fetch(this.endpoint, {
      method: "POST",
      headers: {
        "content-type": "application/json",
      },
      body: JSON.stringify(request),
    });

    const payload = await response.json();
    if (!response.ok || payload.ok === false) {
      throw new Error(payload.message ?? `adapter request failed with status ${response.status}`);
    }

    return payload;
  }

  async createProjectDocument({ projectId, name, aspectRatio, paths }) {
    return this.send({
      type: "create_project_document",
      project_id: projectId,
      name,
      aspect_ratio: aspectRatio,
      snapshot_path: paths.snapshotPath,
      event_log_path: paths.eventLogPath,
    });
  }

  async getDocumentState(paths) {
    return this.send({
      type: "get_document_state",
      snapshot_path: paths.snapshotPath,
      event_log_path: paths.eventLogPath,
    });
  }

  async applyCommands(commands, options = {}) {
    return this.send({
      type: "apply_commands",
      snapshot_path: options.paths.snapshotPath,
      event_log_path: options.paths.eventLogPath,
      save: options.save ?? true,
      commands,
    });
  }
}
