# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.2.0] - 2025-01-02

### Changed
* BREAKING: allow using any keyring, not only session. Functions are renamed from get_session_* to get_*
* BREAKING: store secrets as bytes instead of Unicode strings


## [0.1.0] - 2024-12-31

### Added

* Setting session secret (UTF8 strings only are supported)
* Getting session secret by name
* Invalidating session secret
