# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- Initial mock pipeline execution in the Dashboard
  - Triggered by "Protect" button
  - Simulates steps: parsing, string encryption, function obfuscation, rebuild
  - Displays progress bar with real-time updates
  - Logs progress messages into the UI
  - Generates a temporary output file (`.obscura-log`) as a mock artifact
- Application state updated with:
  - `processing` flag
  - `progress` field
  - `pipeline_rx` (mpsc receiver for pipeline messages)
- Utility method `push_log` for timestamped logging
- Basic message passing between background thread and UI (mpsc channel)

### Changed
- Dashboard now shows progress bar and allows clearing logs
- "Protect" button disabled while pipeline is running

### Fixed
- Resolved borrow checker conflicts in pipeline message polling by using `Option::take` pattern

---