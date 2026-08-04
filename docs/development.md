# Development

All contributions are welcome. Large contributions should first be opened and
discussed in an [issue](https://github.com/BoringOrng/voxy/issues).

---

### Related links

[Contribution guidelines](contribution.md)

[LLM Policy](llm-policy.md)

[Mod Development](mod-development.md)

---

## Getting Started

> [!Note]
> If you're just here to run the project, follow the steps at
> [Via Rustup](#via-rustup), and run:
>
> ```Bash
> cargo run --release
> ```

### The Nix Way

Nix is by far the recommended method to install and develop the project, as it
will always be up-to-date with the project standards. First, install nix
[here](https://nixos.org/download/), and run:

```bash
nix develop --experimental-features "nix-command flakes"
```

> [!Note]
Here the usage of the flag `experimental-features` is necessary (unless globally
enabled) to enter the development environment. It's recommended to use a tool like
[direnv](https://direnv.net/) to automatically enter the development environment
for you.

### Via Rustup

Voxy is a game developed using Rust, and the best way to manage all the languages
tools is via rustup. You can install that [here](https://rust-lang.org/learn/get-started/).
From there, run the following commands **In the project folder**.

```bash
rustup install nightly
rustup component add rustc-codegen-cranelift-preview
rustup override set nightly
```

> [!Warning]
We cannot guarantee that your nightly installation will work for this project,
but 99.9% of the time it should. In the case that it doesn't, please open an
[issue](https://github.com/BoringOrng/voxy/issues). Alternatively, see
[The Nix Way](#the-nix-way).

---

> [!Tip]
It's recommended that you use a faster linker. By default the Nix
flake installs and configures the [wild linker](https://github.com/wild-linker/wild).
For an in-depth guide on how to do this, see
[here](https://bevy.org/learn/quick-start/getting-started/setup/#alternative-linkers).

---

## Building And Running

Voxy, like many projects written in Rust, uses Cargo as the package manager. That
means you can build and run the project using the standard Cargo commands.

Before running it is recommended you first
[verify your cargo installation](#verify-your-cargo-installation).

If you still have issues, it may be because you're missing dependencies. For
information on installing the correct dependencies, see
[here](https://bevy.org/learn/quick-start/getting-started/setup/#installing-os-dependencies)

> [!Note]
If you're using Nix, this is likely not the case.

---

A reference for commands you'll end up running often

```bash
cargo run
RUST_BACKTRACE=full cargo run
cargo run --profile=debugging
```

### Verify Your Cargo Installation

If you followed along with [Getting Started](#getting-started), you should have
Cargo installed. Verify with the following command:

```bash
cargo --version
```

> `cargo X.X.X-nightly (<hash> XXXX-XX-XX)`

> [!Important]
If that didn't succeed, or the cargo version isn't appended with `-nightly`,
please revisit [Getting Started](#getting-started).

### Build Profiles

Voxy uses multiple build profiles, each with a specific niche, defaulting to the
`dev` profile.

| Profile | third-party optimized | third-party debug info | own optimization | own debug info |
| --- | --- | --- | --- | --- |
| dev | O3 | None | O0 | line-tables |
| debugging | O3 | full | O0 | full |
| release | O3 | None | O3+lto | None |

Most of the time, as a developer you will be using the `dev` profile. The reason
the distinction exists is because on the first compile, you're compiling an
entire game-engine and Voxy. While debug-information isn't code, the cost of
including it quickly builds up.

> [!Tip]
> While debugging is more difficult on the `dev` profile, this is largely
> mitigated by prefixing the run command with `RUST_BACKTRACE=full`. e.g.
>
> ```bash
> RUST_BACKTRACE=full cargo run
> ```

## Tooling

Voxy uses the standard Rust suite of tools, as well as `cargo-sort`. You should
configure your editor to use `cargo clippy` as the default Rust linter, and if
you modify any `Cargo.toml` files, make sure to run `cargo sort -w` before committing
your changes. Additionally, make sure `rustfmt` is set as the default Rust
formatter, and if not using format-on-save, make sure to run it before committing.

If you're not using Nix, you must install `cargo-sort`, that can be done as follows:

```bash
cargo install cargo-sort
```

> [!Important]
Make sure the path `~/.cargo/bin` is in your `$PATH` environment variable,
otherwise the binary wont be available.
