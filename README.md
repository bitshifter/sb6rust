This is a Rust port of the OpenGL SuperBible 6th Edition sample code.

[![Build Status](https://travis-ci.org/bitshifter/sb6rust.svg?branch=master)](https://travis-ci.org/bitshifter/sb6rust)

The original C++ sample code can be found at
https://github.com/openglsuperbible/sb6code. The media archive used by the
samples is available on the book's website http://www.openglsuperbible.com.
Note that the provided `Makefile` will automatically download the media
archive as part of the build process.

Only a small number of the C++ samples have been ported to Rust, however the
supporting sample application classes are mostly complete and for the most
part have a similar interface. The Rust implementation should be able to load
all of the SB6 media pack models, texture and shaders.

## Building the samples

1. Make sure you have the following dependencies installed:
    * `Rust` 1.0 or later
    * `g++` 4.7 or later
    * GNU `make` 3.81 or later
    * `cmake` 2.8 or later
    * `git`
    * `curl`
    * `unzip`

2. Build the samples:
    Simply build the samples by running `make`. This will run `cargo test` and
    copy required media files to the `media` directory.

3. Run the samples:
    Run the samples from the root of the repository by specifying to path to
    the sample to run, for example `./target/debug/examples/ssao`.

### Building on Windows

To install the required prerequsites on Windows you can use
[MSYS2](http://sourceforge.net/projects/msys2/):

1. Download and run the latest MSYS2 installer.
2. From the MSYS2 terminal install the `mingw64` toolchain and the other
   required tools.

      ```sh
      $ pacman -S git make unzip

      # Choose one based on platform:
      $ pacman -S mingw-w64-i686-cmake mingw-w64-i686-gcc
      $ pacman -S mingw-w64-x86_64-cmake mingw-w64-x86_64-gcc
      ```

3. Start `mingw32_shell.bat` or `mingw64_shell.bat` from your MSYS2 install
   (i.e. `C:\msys64`), depending on whether you are using 32-bit or 64-bit
   Rust.

You are now ready to build and run the samples.

## License

The OpenGL SuperBible 6th Edition sample code is distributed under terms of the
MIT license. 

See LICENSE for details.
