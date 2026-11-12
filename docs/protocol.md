# Shys Protocol, version 1

> **Status: draft.** This document may change in incompatible ways until the
> first release of Shys. From that release on, version 1 is frozen: any change
> to the formats or derivations described here requires a new protocol
> version.

## 1. Overview

Shys lets a sender share a secret with a recipient through a link. The secret
is encrypted on the sender's device. The server stores only ciphertext and
never receives the key material needed to decrypt it.

Everything a client needs is derived from one random value, the link secret
`S`, which travels in the URL fragment and is never sent to the server:

- the **identifier** under which the server stores the secret,
- the **encryption key** that protects the content, and
- the **access token** the recipient must present before the server releases
  the ciphertext.

A secret can optionally be protected with a password. In that case both the
encryption key and the access token also depend on the password.

## 2. Conventions and notation

The key words "MUST", "MUST NOT", "SHOULD", "SHOULD NOT" and "MAY" are to be
interpreted as described in RFC 2119.

- `a ‖ b` is the concatenation of the byte strings `a` and `b`.
- `len(x)` is the length of `x` in bytes.
- All integers are unsigned and encoded in big-endian byte order.
- Byte offsets start at zero.
- Labels are ASCII strings, used without a terminator.

## 3. Cryptographic primitives

| Purpose | Primitive | Reference |
| --- | --- | --- |
| Authenticated encryption | XChaCha20-Poly1305 | draft-irtf-cfrg-xchacha |
| Key derivation | HKDF-SHA256 | RFC 5869 |
| Hash | SHA-256 | FIPS 180-4 |
| Password hashing | Argon2id, version 0x13 | RFC 9106 |
| Link encoding | base64url without padding | RFC 4648, section 5 |

XChaCha20-Poly1305 uses a 32-byte key, a 24-byte nonce and a 16-byte tag.

`KDF(ikm, label, n)` is defined as HKDF-SHA256 with an empty salt:

```text
PRK = HKDF-Extract(salt = "", IKM = ikm)
KDF = HKDF-Expand(PRK, info = label, L = n)
```

`PWHASH(password, salt)` is Argon2id with a 32-byte output, no secret key and
no associated data. Its cost parameters are fixed for version 1 (see
section 13).

All random values MUST come from a cryptographically secure random number
generator provided by the operating system or the browser.

## 4. Link format

A Shys link has the form:

```text
https://<host>/<path>#<fragment>
```

The fragment is the base64url encoding, without padding, of a 34-byte link
token:

| Offset | Size | Field | Value |
| --- | --- | --- | --- |
| 0 | 1 | version | `0x01` |
| 1 | 1 | flags | bit 0 (`0x01`): password required |
| 2 | 32 | S | link secret, random |

All other bits of `flags` are reserved and MUST be zero.

The encoded fragment is exactly 46 characters long. A client MUST reject a
fragment if any of the following is true:

- it is not exactly 46 characters long,
- it contains characters outside the base64url alphabet, or padding,
- its unused trailing bits are not zero,
- the version is not `0x01`,
- a reserved flag bit is set.

The path is defined by the deployment. It MUST NOT contain the identifier or
any other value derived from `S`. Clients MUST NOT send the fragment, or `S`,
to the server.

## 5. Key derivation

Two values are derived from `S` alone, so that a client can compute them
before contacting the server:

```text
id      = KDF(S, "shys/v1/id", 16)
pw_salt = KDF(S, "shys/v1/password-salt", 16)
```

The master input `M` depends on whether a password is used:

```text
M = S                                      without a password
M = S ‖ PWHASH(password, pw_salt)          with a password
```

The encryption key and the access token are derived from `M`:

```text
K = KDF(M, "shys/v1/encryption-key", 32)
T = KDF(M, "shys/v1/access-token", 32)
```

The server never receives `T` when a secret is created. It receives the
verifier:

```text
V = SHA-256(T)
```

## 6. Envelope format

The envelope is the byte string stored by the server.

| Offset | Size | Field | Value |
| --- | --- | --- | --- |
| 0 | 4 | magic | `0x53 0x48 0x59 0x53` ("SHYS") |
| 4 | 1 | version | `0x01` |
| 5 | 24 | nonce | random |
| 29 | `P` + 16 | ciphertext | see below |

The ciphertext is the XChaCha20-Poly1305 encryption of the padded plaintext
(section 7) under the key `K` and the nonce, including the 16-byte tag. The
associated data is the first 29 bytes of the envelope (magic, version and
nonce).

`P` is the length of the padded plaintext. An envelope is therefore `P` + 45
bytes long.

## 7. Plaintext format and padding

The padded plaintext has the following structure:

| Offset | Size | Field | Value |
| --- | --- | --- | --- |
| 0 | 1 | kind | `0x00`: UTF-8 text |
| 1 | 4 | length | `L`, the length of the content |
| 5 | `L` | content | the secret |
| 5 + `L` | `P` - 5 - `L` | padding | zero bytes |

All other values of `kind` are reserved. For kind `0x00` the content MUST be
valid UTF-8. `L` MUST NOT exceed 1,048,576 bytes.

The padded length is `P = pad_len(5 + L)`, where:

```text
pad_len(n):
  if n <= 1024: return 1024
  e    = floor(log2(n))
  s    = floor(log2(e)) + 1
  mask = 2^(e - s) - 1
  return (n + mask) AND NOT mask
```

Every secret with up to 1,019 bytes of content produces an envelope of
exactly 1,069 bytes. Above that size the padding adds at most 6.25% to the
length.

| Content length `L` | Padded length `P` | Envelope length |
| --- | --- | --- |
| 0 | 1,024 | 1,069 |
| 1,019 | 1,024 | 1,069 |
| 1,020 | 1,088 | 1,133 |
| 2,043 | 2,048 | 2,093 |
| 2,044 | 2,176 | 2,221 |
| 5,000 | 5,120 | 5,165 |
| 65,536 | 67,584 | 67,629 |
| 1,048,576 | 1,081,344 | 1,081,389 |

## 8. Creating a secret

1. Generate 32 random bytes `S` and 24 random bytes for the nonce. `S` MUST
    be used for one secret only.
2. Derive `id`, `K` and `T` as described in section 5, and compute `V`.
3. Build the padded plaintext and encrypt it to produce the envelope.
4. Send `id`, `V`, the envelope and the requested lifetime to the server.
5. Build the link from the link token, setting flag bit 0 if a password was
    used.

Implementations MUST NOT offer a way to encrypt with a caller-supplied `S` or
nonce, other than for testing against the test vectors.

## 9. Opening a secret

1. Parse and validate the fragment as described in section 4.
2. Derive `id`. If flag bit 0 is set, ask for the password.
3. Derive `K` and `T`, and present `id` and `T` to the server.
4. On success the server returns the envelope. Reject it unless its length is
    between 1,069 and 1,081,389 bytes, its magic is correct and its version is
    `0x01`.
5. Decrypt the ciphertext with `K`, the nonce and the associated data.
6. Validate the padded plaintext: `kind` is `0x00`, `L` is within the limit,
    the plaintext length equals `pad_len(5 + L)`, every padding byte is zero,
    and the content is valid UTF-8.

The padded plaintext MUST NOT be examined before the decryption in step 5
has succeeded.

A failure in step 5 or step 6 MUST be reported as a single generic error. An
implementation MUST NOT reveal whether the key, the password or the data was
at fault. Failures in steps 1 and 4 concern public values and MAY be reported
specifically.

## 10. Server obligations

For each secret the server stores `id`, `V`, the envelope, an expiry time and
a counter of failed attempts.

When a secret is created, the server:

- MUST reject the request if a secret with the same `id` already exists. It
  MUST NOT overwrite an existing secret.
- MUST reject envelopes shorter than 1,069 bytes or longer than its
  configured maximum, which MUST NOT exceed 1,081,389 bytes.

When a client presents `id` and a token `T'`, the server:

- MUST compare SHA-256(`T'`) with `V` in constant time.
- If they match, MUST delete the secret and return the envelope as a single
  atomic operation, so that the envelope is returned at most once.
- If they do not match, MUST increment the counter of failed attempts and
  MUST delete the secret when the counter reaches the configured limit.
- MUST respond in the same way for a secret that never existed, has expired
  or has already been opened.

The server MUST delete a secret when it expires. It MUST NOT log tokens,
verifiers or envelopes.

## 11. Limits

| Constant | Value |
| --- | --- |
| Link secret `S` | 32 bytes |
| Identifier `id` | 16 bytes |
| Encoded fragment | 46 characters |
| Minimum padded length | 1,024 bytes |
| Maximum content length | 1,048,576 bytes |
| Minimum envelope length | 1,069 bytes |
| Maximum envelope length | 1,081,389 bytes |

## 12. Security considerations

**The link is the secret.** Anyone who obtains the link can open a secret
that has no password. Links MUST be shared over a channel the sender trusts.
Clients SHOULD remove the fragment from places where it could persist, such
as the browser history, and MUST NOT log links.

**What the server learns.** The server learns `id`, `V`, the envelope length,
the creation and access times, and network metadata about its clients. It
does not learn the content, `S`, `K`, or whether a secret has a password.

**Password guessing.** An attacker who has the link but not the password
cannot test password guesses offline, because the envelope is released only
in exchange for the correct token. Each wrong guess counts as a failed
attempt, and the server deletes the secret when the limit is reached. A
legitimate recipient who mistypes the password also consumes attempts. An
attacker who has both the link and the server's data can test guesses
offline; Argon2id exists to make that expensive.

**Parameters fixed by version.** A client must run Argon2id before it can
authenticate anything it received. For that reason the cost parameters are
fixed by the protocol version and are not taken from data supplied by the
server, which could otherwise exhaust the client's memory.

**Key commitment.** XChaCha20-Poly1305 is not key-committing. Attacks that
exploit this property require knowledge of `S`. An attacker who knows `S` and
holds the envelope can already test passwords directly, so the property gives
no additional advantage here.

**Quantum resistance.** Version 1 uses only symmetric cryptography with
256-bit keys and hash functions with 256-bit outputs. It does not rely on
public-key cryptography.

**Web clients.** A web client runs code delivered by the server. A malicious
or compromised server can deliver code that leaks `S`. This protocol cannot
prevent that; the command-line client is not affected.

**No compression.** Content MUST NOT be compressed before encryption.

## 13. Open items

The following values will be fixed before this document leaves draft status:

- The Argon2id cost parameters (memory, iterations and parallelism).
- The normalization applied to passwords before hashing, and their minimum
  and maximum length.
- The default limit of failed attempts.
- The HTTP API, including the textual encoding of `id`, `V` and `T`.

## 14. Test vectors

To be added.
