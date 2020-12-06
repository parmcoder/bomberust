# RUSTRIS
This is a term project for my ICCS311-T1-2021.

#### Objectives
1. To apply the concept of parallelism and concurrency using amethyst game engine.
2. To study how tetris works and practice coding in rust
3. To show that game developing in rust is awesome.

#### Why rust?
As I have studied about Rust, it is capable of controlling low-level
stuffs in a way that we can apply functional and parallel programming safely.
Its rich type system and ownership model guarantee memory-safety and thread-safety.
That means we do not have to run into bugs during runtime and fix the bug at compile-time.

If I was new to programming, I will hate rust so much (Because, I will not be able to submit any assignment :P).
However, I found that it is so convenient to use than C and C++ to achieve a high performance.

#### How games developed in Rust are more optimized using than other languages?

As a game developer, I would love to develop my games on Unity and Unreal engine which used C# and C++.
But, I cannot look over the benefits of Rust. Here are some reasons why Rust is potentially another language suitable for game development.
Its emphasis on low-level memory safe programming promise a better development process, less debugging time, and better end result.

At first, C# and C++ ecosystems were as young as how Rust is. But, Rust is as good as those languages
right now though its ecosystem is still young (very young).


#### Why amethyst?

1. Good game engine with good book, https://book.amethyst.rs/master/
2. I want to use its Entity Component System (ECS) architecture to organize game logic.
3. There are people who have used this engine and enough learning materials for me to learn.

Though there are many other engines or libraries that can be used for game developing,
Amethyst performance is much better than the other library and it is capable of doing 3D works.
I want a good environment to develop a game also, and Amethyst is the friendliest game engine
since it has their own book for me to read (Yes, I read the whole book). It is quite difficult to
understand at first, but once I started this project it became much easier.

#### How is the performance?

So far, the performance from using this engine is amazing, and it is light-weight compared to its performance.
This is the showcase that people are working on https://vimeo.com/332649771.

For this project, the game runs smoothly and it starts very quickly once compiled.

#### How is this related to parallel programming?
Well, Modularity, Parallelism, and Data-driven are provided from the engine.

The engine is based on the Specs library, which is a common base on which the engine's concepts are built.
Specs automatically parallelizes system execution when there are non-conflicting system data requirements (Two Systems conflict if their SystemData needs access to the same resource where at least one of them needs write access to it).

Therefore, systems in this project run in parallel like most game engines. Without this game engine,
this game performance would have dropped significantly.

#### Milestones
1. I started from this https://arewegameyet.rs/. This website convinced me to try making game in rust.
2. Studied Amethyst by following https://book.amethyst.rs/master/pong-tutorial.html, a tutorial to create Pong.
3. Explore github repos and see how people implement tetris, I found that there are many tricks to write this game.
4. Write codes, yes... Also, I have done play testing. It is not as good as modern tetris, but bearable.
5. Refractor and finish this readme.md

## Discussion
This project can be continued to make it as good as the modern tetris, here are some tasks.
1. State transitions are difficult to handle in Rust, I cannot find how to make the game ended yet.
2. The Super Rotation System, or SRS is the current Tetris Guideline standard for how tetrominoes rotate and what wall kicks they may perform.
3. Moves detections
4. Scoring system

It is a fun project, though not related to my course that much, 
but I choose to do it. There might be some parts in this project
that can be even more optimized, there are rooms for improvement.

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
