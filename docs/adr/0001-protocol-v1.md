# ADR 0001: Protocol version 1

- **Status:** accepted
- **Date:** 2026-11-11

## Context

Shys needs a format for encrypted secrets and for the links that open them.
Once the first release is published, this format cannot change without a new
protocol version, so the decisions and the alternatives we rejected are
recorded here. The format itself is specified in
[`docs/protocol.md`](../protocol.md).

The goals, in order of priority:

1. The server must not be able to read secrets.
2. The design must be simple enough to implement and review with confidence.
3. The server should learn as little metadata as possible.
4. A short password should offer real protection.

## Decision

Everything is derived from a single 32-byte link secret `S` that travels in
the URL fragment:

- The identifier, the encryption key and an access token are derived from
  `S` with HKDF-SHA256, using a distinct label for each value.
- The server stores a hash of the access token and releases the envelope
  only in exchange for the token. It deletes the secret after a limited
  number of failed attempts.
- With a password, the encryption key and the access token also depend on
  Argon2id(password), salted with a value derived from `S`.
- The content is encrypted with XChaCha20-Poly1305 and padded to hide its
  length.
- The protocol version alone determines every algorithm and parameter.

## Alternatives considered

### An algorithm identifier in the envelope

Rejected. A separate algorithm field creates combinations that must all be
tested and opens the door to downgrade attacks. A new algorithm means a new
protocol version.

### Argon2id parameters stored in the envelope

Rejected. A client has to run Argon2id before it can authenticate anything
it received. If the parameters came from the server, a malicious server
could request gigabytes of memory and crash the client. Secrets are
short-lived, so raising the parameters later through a new protocol version
has no legacy cost.

### A random identifier chosen by the server, placed in the URL path

Rejected in favour of an identifier derived from `S`. With the identifier in
the path, link preview bots and anyone who sees the path in a log can open
or destroy the secret. With a derived identifier the URL contains nothing
but the fragment, which is never sent over the network.

### No access token

Rejected. Without a token, anyone who intercepts the link can download the
envelope and test passwords offline without limit. With a token that depends
on the password, guesses must go through the server, which counts them. The
cost is that a legitimate recipient who mistypes the password also consumes
attempts.

### The password flag in the envelope header

Rejected. The flag travels in the link instead, so the server cannot tell
which secrets are protected with a password.

### A random salt stored in the envelope

Rejected. The access token depends on the password and must be computed
before the envelope is downloaded, so the Argon2id salt is derived from `S`.
A salt must be unique, not secret, and `S` is random and used once.

### AES-256-GCM through the Web Crypto API

Rejected in favour of XChaCha20-Poly1305 in a single Rust implementation
shared by the command-line client and the web client. This avoids two
implementations of the same format, and XChaCha20-Poly1305 has a nonce large
enough to be chosen at random and is fast and constant-time without
hardware support.

### A key-committing construction

Not adopted. XChaCha20-Poly1305 is not key-committing, and attacks on
password-based encryption exploit this. In this design they require
knowledge of `S`, and an attacker who knows `S` and holds the envelope can
already test passwords directly, so a commitment would add size and
complexity without removing an attack.

### Chunked encryption for large files

Deferred. Version 1 encrypts the content in one piece and limits it to
1 MiB of text. Files of arbitrary size need chunked authenticated
encryption, which is the part of such a design where mistakes are most
likely. It can be introduced later as a new protocol version; the `kind`
byte inside the plaintext leaves room for new content types.

### Public-key recipients

Deferred. Encrypting to a recipient's public key, including a hybrid
post-quantum key exchange, would require identities and key distribution and
would change what Shys is.

## Consequences

- The link format and the key derivation are frozen at the first release.
- Opening a secret requires a client that can run the key derivation, so
  there is no fallback for browsers with JavaScript disabled other than the
  command-line client.
- The identifier is chosen by the client, so the server must refuse to
  overwrite an existing secret.
- The server must keep a counter of failed attempts for every secret.
- A sender can revoke a secret by opening their own link.
