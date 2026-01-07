# GitHub Actions Workflows

This directory contains the GitHub Actions workflows for automated releases.

## Workflows

### bump-version.yml
Triggered when `Cargo.toml` is pushed to the main branch. It:
1. Compares the version in the current commit with the previous commit
2. If the version has changed, creates a new git tag (e.g., `v0.2.0`)
3. Pushes the tag to trigger the release workflow

**Note:** Commits with `[skip ci]` in the message are skipped.

### release.yml
Triggered when a version tag is pushed (e.g., `v0.2.0`). It:
1. Builds the Rust project for multiple platforms
2. Creates release binaries for:
   - macOS x86_64 (Intel)
   - macOS ARM64 (Apple Silicon)
   - Linux x86_64 (Intel/AMD)
   - Linux ARM64 (ARM)
   - Windows x86_64 (64-bit)
3. Uploads artifacts and creates a GitHub release with release notes

## Release Process

1. Developer updates version in `Cargo.toml`
2. Developer commits and pushes to main
3. `bump-version.yml` runs and creates a git tag
4. `release.yml` runs and builds binaries for all platforms
5. GitHub release is created with all binaries attached
