This is a Rust port of the OpenGL SuperBible 6th Edition sample code.

The original C++ sample code can be found at
https://github.com/openglsuperbible/sb6code. The media archive used by the
samples can be downloaded from the book's website at
http://www.openglsuperbible.com.

Only a small number of the C++ samples have been ported to Rust, however the
supporting sample application classes are mostly complete and for the most
part have a similar interface. The Rust implementation should be able to load
all of the SB6 media pack models, texture and shaders.

## Prerequisites

The samples can be compiled with nightly versions of
[Rust](http://www.rust-lang.org) and the [Cargo](http://crates.io) build
system.

### Windows specific prerequisites

To easily build on Windows you can use
[MSYS2](http://sourceforge.net/projects/msys2/):

1. Download and run the latest MSYS2 installer.
2. From the MSYS2 terminal install the mingw64 toolchain and the other required
   tools:

        $ pacman -S mingw-w64-i686-toolchain
        $ pacman -S mingw-w64-i686-cmake
		$ pacman -S base-devel

3. With that now start `mingw32_shell.bat` from where you installed MSYS2
   (i.e. `C:\msys64`).

## Compiling the and running the samples

To build the samples simple run:

~~~
cargo build
~~~

Cargo will output the sample executables into the target directory.

Make sure you have unpacked the media archive downloaded from
http://www.openglsuperbible.com/example-code/ into the target directory before
running the samples.

## TODO

The media archive doesn't contain shaders, currently these have to be copied
from the SB6 example code.

Running `cargo clean` deletes the target directory which is where we copied
the media to run the samples. Ideally cargo build would make sure that meida
is copied to the right place.

## License

The OpenGL SuperBible 6th Edition sample code is distributed under terms of the
MIT license. 

See LICENSE for details.
