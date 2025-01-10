# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

### ⚙️ Miscellaneous Tasks

- Corrected rust dir for tails image
- Corrected rust target for tails image
- Use different tails img
- Removed tails steps to aid testing
- Added git cliff for changelog
- New tails build
- Changed to self hosted
- Removed comment from package install
- Changed tails repo
- Changed tails build
- Changed tails build to use rake
- Changed tails build to use rake
- Changed tails build to use rake
- Changed tails build to use rake
- Changed tails build to use rake
- Changed tails build to use rake
- Commit mixer to tails
- Commit mixer to tails
- Commit mixer to tails
- Commit mixer to tails
- Commit mixer to tails
- Commit mixer to tails
- Commit mixer to tails
- Changed tails build to use rake
- Changed tails build to use rake
- Changed tails build to use rake

## [0.3.0] - 2024-12-30

### 🚀 Features

- Added new type for seed plus constructor and tests
- Implement obfuscation and deobfuscation of seeds
- Added shamir sharing
- Generate and split shamir shares to file
- Added additional encryption to shares (optional)
- Added integration tests
- Added better integration tests
- Add cross compilation for Rust
- Added file shredding
- Added file name override

### 🐛 Bug Fixes

- Restructued repo
- Added cargo lock
- Corrected rust nix build file
- Added hex encoding to key file
- Fixed nix build

### ⚙️ Miscellaneous Tasks

- Update deps
- Clippy
- Added cross comp release build
- Removed arm from build
- Removed arm from build
- Switched mac arch
- Install openssl for each arch
- Adjusted windows openssl install
- Install perl for windows build and repointed upload paths
- Removed vendored ssl
- Macos upload all target file
- Macos upload all target file
- Windows check openssl dir
- Changed upload paths
- Changed upload paths
- Changed upload paths
- Added list files to workflow
- Changed ssl dir
- Changed ssl dir
- Changed ssl dir
- Changed ssl dir
- Changed ssl dir
- Changed ssl dir
- Target windows gnu
- Target windows gnu
- Target windows gnu
- Use cross for windows build
- Added comments and tests
- Comments and tests
- Added workflow to build tails OS image

### Build

- Corrected build file
- Added build tomls
- Use vendored ssl

## [0.0.2] - 2024-03-21

### ⚙️ Miscellaneous Tasks

- Download binaries to root path

## [0.0.1] - 2024-03-21

### ⚙️ Miscellaneous Tasks

- Added artefact download and list

## [0.0.0] - 2024-03-21

### 🚀 Features

- Restructure build workflows and configuration
- Improve GitHub Actions and test coverage
- Rust: added cli validator and tests for valid pin numbers

### 🚜 Refactor

- Refactor build configurations and remove unnecessary files
- Refactor project structure and file naming

### 📚 Documentation

- Add new files and update paths in documentation.
- Update documentation build process
- Improve documentation build and deployment
- Update language codes for internationalization
- Update configuration for site URL and root path
- Update book.toml configuration
- Update documentation build process for multiple translations
- Update and deploy documentation to Netlify
- Refactor project configuration for default site URL
- Remove unused code from theme documentation files
- Update theme and add various scripts and links
- Update theme layout and navigation features

### ⚙️ Miscellaneous Tasks

- Update Mixer Flake configuration and build process
- Change file permissions and modes for multiple files
- Added golang repo
- Refactor and improve Nix expressions and dev environment configuration
- Add `result` file and update flake files
- Remove unnecessary configuration files
- Refine the .gitignore file with updated ignore rules
- Update import paths, package details, and pre-commit config path
- Update licenses and parameters across multiple files
- Update GitHub Actions workflows and dependencies
- Refactor GitHub Pages deployment step
- Update deployment action and remove unused option
- Update GitHub Pages deployment directory
- Update Build and Deployment Process for Documentation
- Update documentation workflows and dependencies
- Improve documentation workflow
- Update GitHub Actions and toolchain versions for documentation workflow
- Refactor GitHub Actions workflow for documentation deployment
- Increase returned recording amount and improve GitHub actions
- Update GitHub Actions and dependencies
- Update documentation site URL
- Update build and deployment configurations for documentation
- Update deployment configuration for GitHub Actions workflows
- Remove unnecessary deployment actions for documentation pipelines
- Update GitHub workflow and Netlify deployment configuration
- Improve GitHub Actions workflow for documentation build
- Remove unnecessary line from GitHub workflow
- Update GitHub actions and improve code quality
- Update dependencies and remove unused environment variable
- Refactor GitHub Actions workflow for documentation redirection.
- Improve documentation build workflow
- Update GitHub actions workflow for better compatibility
- Changed rust toolchain action in release workflow
- Changed rust test and build commands in release workflow
- Wrapped references to working directory in quotes
- Changed working paths for literals

### Build

- Added ocaml, haskell and golang to the build workflow
- Changed nix build step to build packages rather than apps
- Refactor flake.nix file: Update go package name
- Changed pathing in golang to be consistent with other langs
- Changed docs build to any push
- Changed docs flow to checkout without submods
- Removed token write perms in workflow
- Corrected language code for cs_CZ
- Corrected lang build step
- Added token permissions
- Work around for bad pathing in mdbook translations
- Work around for bad pathing in mdbook translations
- Reset mdbook workaround
- Added an intial release file for rust
- Removed download artefact step
- Restricted docs build to changes in docs path only
- Restricted build to changes to src or flake
- Corrected publish branch
- Added workflow dispatch on release workflow for testing
- Renamed release yml
- Corrected working dir pathing in release

<!-- generated by git-cliff -->
