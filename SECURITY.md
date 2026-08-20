# Security Policy

## Reporting a vulnerability

If the issue is in Nerminal's own changes, report it privately through this
repository's [security advisories](https://github.com/klebeer/nerminal/security/advisories/new).
Please do not open a public issue or pull request first.

If the issue is in Warp itself and also affects upstream, report it to the Warp
team at security@warp.dev or through
[their advisory form](https://github.com/warpdotdev/Warp/security/advisories/new).
Nerminal cannot fix upstream code for anyone but its own users.

## Scope

Nerminal makes no network requests: its server URLs point at the loopback
discard port and telemetry, crash reporting and autoupdate are compiled out of
the shipped configuration. Reports that depend on Nerminal contacting a remote
host are almost certainly describing upstream Warp instead.
