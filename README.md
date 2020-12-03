# RUSTRIS
This is a term project for my ICCS311-T1-2021.

Objectives
1. To apply the concept of parallelism and concurrency using amethyst game engine.
2. To study how tetris works and practice coding in rust
3. To show that game developing in rust is awesome.

Why rust?
TBA

How games developed in Rust are more optimized using than other languages?
TBA

Why amethyst?
TBA

How is the performance?
TBA

Pros and Cons?
TBA

Milestones
TBA

Discussion
TBA

## Quickstart

- Clone the repository

```bash
git clone https://github.com/parmcoder/rustris-amethyst
cd rustris-amethyst
```

- Build and run the project

```bash
cargo run
```

#### For Mac Users

This starter uses vulkan as a renderer by default. You'll want to change the backend to use `metal`, which can be done by opening the `Cargo.toml` file and changing

```toml
[features]
default = ["vulkan"]
```

to

```toml
[features]
default = ["metal"]
```

If using OSX and Metal you will require full XCode installed from the Appstore in order to compile metal shaders.
After install you may be required to run this command `sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer` [reference gfx-rs issue](https://github.com/gfx-rs/gfx/issues/2472)

#### For Linux Users

You might need to install some dependencies. Please refer to [this section](https://github.com/amethyst/amethyst#dependencies) of the README for more details.