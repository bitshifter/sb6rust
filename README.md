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
[Rust](http://www.rust-lang.org) and the [Cargo](http://crates.io) build system.

The application sample framework depends on
[GLFW](http://www.glfw.org/download) 3.x. Either install it through your
system's package manager, download a prebuilt binary or compile it from source.

If you compile GLFW with CMake on Linux you must invoke CMake with
`-DCMAKE_C_FLAGS=-fPIC`.

## Compiling the and running the samples

To build the samples simple run:

~~~
cargo build
~~~

If GLFW is not installed to a standard location you will need to specify the
directory containing the GLFW libraries when you build the SB6 samples:

~~~
LIBRARY_PATH=path/to/glfw/lib/directory cargo build
~~~

Cargo will output the sample executables into the target directory.

Make sure you have unpacked the media archive downloaded from
http://www.openglsuperbible.com/example-code/ into the target directory before
running the samples.

## License

The OpenGL SuperBible 6th Edition sample code is distributed under terms of the
MIT license. 

See LICENSE for details.
