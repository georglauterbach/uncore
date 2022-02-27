---
tags:
  - security
  - security policy
---

# Security Policy

**version 1.0**, 14 Jan 2022

## Supported Versions

We support the latest version of our kernel modules and the kernel with security updates. That is to say that we do **not** backport security updates to older versions as of now.

## Checks

For every pull requests, and on schedule (every Saturday), a GitHub workflow under [`.github/workflows/security.yml`][code::security-workflow] called "Security Audit" is run. This workflow will check for any security vulnerabilities introduced by crates used in this project.

## Reporting a Vulnerability

_unCORE_ takes security very seriously. We will follow the rule of [responsible disclosure] in the future. **As of now**, please just open an issue. As there is no centralized domain / mail service, opening an issue is the easiest way for now.

### Responsible Disclosure

If you discover a vulnerability in _unCORE_, please first contact one of the maintainers privately. Users who report bugs will optionally be credited for the discovery.

We urge you not to disclose the bug publicly at least until we've had a reasonable chance to fix it, and to clearly communicate any public disclosure timeline in your initial contact with us. If you do not have a particular public disclosure timeline, we will clearly communicate ours as we publish security advisories.

#### Process

1. A user privately reports a potential vulnerability.
2. The core team reviews the report and ascertain if additional information is required.
3. The core team reproduces the bug.
4. The bug is patched, and if possible the user reporting te bug is given access to a fixed version or git patch.
5. The fix is confirmed to resolve the vulnerability.
6. The fix is released.
7. The security advisory is published sometime after users have had a chance to update.

[//]: # (Links)

[code::security-workflow]: https://github.com/georglauterbach/uncore/blob/master/.github/workflows/security.yml
[responsible disclosure]: https://en.wikipedia.org/wiki/Responsible_disclosure
