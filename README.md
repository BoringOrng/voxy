![Github Pull Requests](https://img.shields.io/github/issues-pr/BoringOrng/voxy)
![GitHub Issues](https://img.shields.io/github/issues/BoringOrng/voxy)
![GitHub last commit](https://img.shields.io/github/last-commit/BoringOrng/voxy)

---

# Voxy

Voxy is a voxel game built with [Bevy](https://bevy.org/), placing a focus on
extensibility via modding.

## Installation

The project is still very early in development, with no gameplay features.
However, if you still wish to play the game in it's very early state, go
[here](docs/development.md#via-rustup).

## Roadmap

> [!Important]
This may not always be up-to-date, and may change on a whim.

```
Generate Climate data -> |
                         | -> Worldgen -> Torus or ring world -> |
Biome mod format ------> |                                       | -> |
                                                                 |    |
Physics mod format --------------------------------------------> |    |
                                                                      |
                                             ??? <- Physics <---------|

Player mod format -----------------> | Inventory
                                     | Hotbar
UI mod format -> Base UI textures -> | Main menu
                                     | Crafting mod format -> Crafting

Better dev tools
Benchmarks
Testing
CI
```

## Development

> [!Warning]
LLM usage is allowed, with [these](docs/llm-policy.md) restrictions. Regardless
of whether or not you use LLMs in your workflow, you should read these if you're
looking to contribute.

---

For mod development, see [here](docs/mod-development.md).

For Voxy development, see [here](docs/development.md).

> [!Important]
Contribution details and guidelines can be found [here](docs/contribution.md).
