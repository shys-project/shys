# Security Policy

Shys is a tool for sharing secrets, so security reports are our highest
priority. Thank you for helping keep Shys and its users safe.

## Project status

Shys is under early development and has not been audited. There are no
releases yet, so security fixes are applied to the `main` branch only. This
section will list the supported versions once releases exist.

## Reporting a vulnerability

**Please do not report security vulnerabilities through public issues,
discussions or pull requests.**

Report them privately through GitHub's private vulnerability reporting:

<https://github.com/shys-project/shys/security/advisories/new>

Please include as much of the following as you can:

- A description of the vulnerability and its impact.
- The affected component (`shys-core`, `shys-cli`, `shys-server` or
  `shys-web`) and the version or commit.
- Steps to reproduce the issue, or a proof of concept.
- Any suggested fix or mitigation.

Never include real secrets or real Shys links in your report: a Shys link
contains the decryption key.

## What to expect

- We will acknowledge your report within 7 days.
- We will keep you informed while we investigate and fix the issue.
- Once a fix is available, we will publish a security advisory and credit you,
  unless you prefer to remain anonymous.

We ask that you give us a reasonable amount of time to fix the issue before
disclosing it publicly. We aim to resolve reports within 90 days.

Shys does not run a bug bounty program.

## Scope

In scope:

- Flaws in the cryptographic design or implementation.
- Anything that allows the server or a third party to read a secret.
- Retrieving a secret more than once, or after it has expired.
- Leaks of keys or plaintext through logs, caches, headers or the network.
- Vulnerabilities in the server, the command-line client or the web client.

Out of scope, because they are known limitations of the design:

- Compromised devices of the sender or the recipient.
- A malicious server operator serving modified web client code. This is an
  inherent limitation of encryption in the browser; the command-line client
  is not affected by it.
- Recipients forwarding or copying a secret after reading it.
- Volumetric denial-of-service attacks.

## Safe harbor

We will not take legal action against researchers who act in good faith:
who report vulnerabilities privately through the process above, avoid
accessing or destroying other people's data, and give us reasonable time to
fix the issue before disclosing it.
