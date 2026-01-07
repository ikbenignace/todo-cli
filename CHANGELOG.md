# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2025-01-07

### Added
- Optional project parameter support (e.g., `todo PROJECT "summary"`)
- Shell completion support for bash, zsh, fish, and PowerShell
- `completion` command to generate shell completion scripts
- Installation scripts for shell completions
- Enhanced help messages with examples

### Changed
- Default project behavior: MS is now the default project
- Updated README with shell completion instructions

## [0.2.0] - 2025-01-07

### Added
- Automatic ticket assignment to creator (@me)
- Automatic status transition to "Selected for Development"
- Added regex dependency for ticket key parsing

## [0.1.0] - 2025-01-07

### Added
- Initial release
- Simple CLI wrapper for Atlassian CLI (acli)
- Create Story-type tickets in project MS
- Check if acli is installed and authenticated
- Display helpful installation instructions for all platforms
