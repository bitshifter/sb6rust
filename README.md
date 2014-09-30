This is a Rust port of the OpenGL SuperBible 6th Edition sample code.

The original C++ sample code can be found at
https://github.com/openglsuperbible/sb6code. The media archive used by the
samples can be downloaded from the book's website at
http://www.openglsuperbible.com.

Only a small number of the C++ samples have been ported to Rust, however the
supporting sample application classes are mostly complete and for the most
part have a similar interface. The Rust implementation should be able to load
all of the SB6 media pack models, texture and shaders.

## Building the samples

1. Make sure you have the following dependencies installed:
    * `rust` [recent nightly][rust_nightly]
    * `cargo` [recent nightly][cargo_nightly]
    * `g++` 4.7 or later
    * GNU `make` 3.81 or later
    * `cmake` 2.8 or later
    * `git`

2. Build the samples:
    Simply build the samples by running `make`. This will run cargo build and
    copy required media files to the `target` directory.

3. Copy media files:
    Unzip the media archive from http://www.openglsuperbible.com/example-code/
    into the `target` directory.

3. Run the samples:
    `cd` to the `target` directory and run the sample you want.

[rust_nightly]: http://www.rust-lang.org
[cargo_nightly]: http://crates.io

### Building on Windows

To install the required prerequsites on Windows you can use
[MSYS2](http://sourceforge.net/projects/msys2/):

1. Download and run the latest MSYS2 installer.
2. From the MSYS2 terminal install the mingw64 toolchain and the other required
   tools:

        $ pacman -S mingw-w64-i686-toolchain
        $ pacman -S mingw-w64-i686-cmake
	$ pacman -S base-devel

3. Start `mingw32_shell.bat` from where you installed MSYS2 (i.e. `C:\msys64`).

## TODO

Running `cargo clean` deletes the target directory which is where we copied
the media to run the samples. Ideally cargo build would make sure that media
is copied to the right place.

## License

The OpenGL SuperBible 6th Edition sample code is distributed under terms of the
MIT license. 

See LICENSE for details.
