# NumbersRepresentation
This repository presents a numbers vizualizer in a polar base. This small project has been entirely made in Rust ! 
This is an idea that has been brought to me after watching a video about the Dirichlet Theorem from the Youtube Channel 3blue1Brown. You can check the video [right here](https://www.youtube.com/watch?v=EK32jo7i5LQ&ab_channel=3Blue1Brown).

## Prerequisites

The project is built with following libraries, tools and utilitaries:

* [Rust](https://www.rust-lang.org/)
* [SDL2](https://www.libsdl.org/index.php)
* [SDL2_ttf](https://www.libsdl.org/projects/SDL_ttf/)
* [SDL2_image](https://www.libsdl.org/projects/SDL_image/)
* [SDL2_gfx](https://www.libsdl.org/index.php)

### Install Rust

As it is built with Rust, in order to build the project, you need to have the compiler installed !
As a reminder, here is how you install it:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install SDL tools

* On MacOS:

Assuming you have homebrew installed, please type the following command in your terminal:
```bash
brew install sdl2 sdl2_gfx sdl2_image sdl2_ttf
````

* On Ubuntu:

In order to install the SDL tools on your Ubuntu, please type the following command in your terminal:
```bash
sudo apt install libsdl2-2.0-0 libsdl2-gfx-1.0-0 libsdl2-image-2.0-0 libsdl2-ttf-2.0-0
```

## Usage

First, you need to clone the repository. If you have an ssh key registered, please type following command:
```bash
git clone git@github.com:LilianSchall/NumbersRepresentation.git && cd NumbersRepresentation
```

* Build the project:

With Rust and all its tools with it installed, type the following command:
```bash
cargo build
```

* Run the project:

Simply type the following command:
```bash
cargo run
```

## Contacts

If you have any question about the project, feel free to ask me on social media !

* [Twitter](https://twitter.com/lilixns)
* [Instagram](https://www.instagram.com/404lilian/)
* [LinkedIn](https://www.linkedin.com/in/lilian-schall-456338206/)
