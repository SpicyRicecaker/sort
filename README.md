# Installation MacOS

Install rust via https://www.rust-lang.org/tools/install, or use the following snippet below copied from the website:

```
curl –proto ‘=https’ –tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Next install [`sdl2`](https://github.com/Rust-SDL2/rust-sdl2)

```shell
brew install sdl2
# add this library to path by adding this line to your ~/.zshrc
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
```

Then simply run the application
```
cargo run
```
