ChangeLog
=========

0.1.3 - 25-05-2022
------------------
Changed
* replaced flag `--command` with clap subcommands (see README)
* replaced `--jsonstruct` option with `--json` when `to_message_pack` subcommand is invoked

0.1.2 - 13-05-2022
------------------
Changed
* removed `default-features=false` for ureq
  now tls is enabled

0.1.1 - 11-05-2022
------------------
Changed
* removed `nonce` from args, now it is random 

Added
* `to_message_pack` string and json structures conversion

Improved
* added proper content type to ureq post request

0.1.0 - 05-04-2022
------------------
Added
* `create_unit_tx` functionality




All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com).

Type of changes

* Added: for new features.
* Changed: for changes in existing functionality.
* Deprecated: for soon-to-be removed features.
* Removed: for now removed features.
* Fixed: for any bug fixes.
* Security: in case of vulnerabilities.

This project adheres to [Semantic Versioning](http://semver.org).

Given a version number MAJOR.MINOR.PATCH
* MAJOR incremented for incompatible API changes
* MINOR incremented for new functionalities
* PATCH incremented for bug fixes

Additional labels for pre-release metadata:
* alpha.x: internal development stage.
* beta.x: shipped version under testing.
* rc.x: stable release candidate.
