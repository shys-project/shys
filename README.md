# Shys

**Share your secrets.**

Shys (pronounced like *shies*, /ʃaɪz/) is a self-hostable service for sharing
secrets such as passwords, API keys or private notes through one-time links
that self-destruct.

> [!WARNING]
> Shys is under early development and has not been audited.
> Do not use it to share real secrets yet.

Secrets are encrypted on your device before they leave it. The decryption key
travels in the link's fragment (the part after `#`), which browsers never send
to the server, so the server only ever stores data it cannot read.

## Design goals

- **Client-side encryption.** The server never sees your secrets or your keys.
- **Burn after reading.** Secrets are deleted on first read and expire
  automatically.
- **Tor as a first-class citizen.** Available as an onion service as well as on
  the clear web, with a command-line client that works over Tor.
- **One cryptographic core.** Written in Rust and shared by the command-line
  client and the web client, compiled to WebAssembly.
- **Easy to self-host.** Native packages for Debian and Arch Linux.

## License

Shys uses different licenses for different parts of the project:

- `shys-core` and `shys-cli` are dual-licensed under MIT or Apache-2.0, at your option.
- `shys-server` and `shys-web` are licensed under AGPL-3.0-or-later.

Everything else (documentation, configuration) is dual-licensed under MIT or
Apache-2.0. See the `LICENSES/` directory and `REUSE.toml` for details.
