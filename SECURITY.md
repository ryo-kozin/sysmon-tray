# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | Yes       |

## Reporting a Vulnerability

If you discover a security vulnerability, please report it responsibly:

1. **Do not** open a public issue
2. Email the maintainer or use GitHub's private vulnerability reporting feature
3. Include a description of the vulnerability and steps to reproduce

We will acknowledge receipt within 48 hours and aim to release a fix within 7 days for critical issues.

## Security Measures

- Content Security Policy (CSP) restricts script and resource loading
- Tauri capabilities grant only the minimum required permissions
- No network access — all monitoring is local
- Configuration is stored in the user's OS config directory with standard file permissions
