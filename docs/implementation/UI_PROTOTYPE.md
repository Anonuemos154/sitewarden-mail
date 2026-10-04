# Desktop UI prototype

The React shell now contains a synthetic local product prototype for:
- unified inbox/list;
- Safe View message display;
- explainable risk signals;
- non-clickable link inspection;
- Security Center;
- Cleanup plan;
- Quarantine;
- provider/account hub;
- rule/productivity/settings surfaces.

All addresses/messages are `.invalid` synthetic data. The UI intentionally does not claim real provider connectivity. Codex should connect these surfaces to typed Tauri commands after the Rust/provider layers compile and pass tests.
