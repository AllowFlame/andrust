# andrust

Very simple command line tool to setup configuration for android native binaries

### Why this is useful?

If you want to use native binary in your android project, and you don't like to write it in C/C++, writing in rust can be choosable.

Similarly for iOS, lipo is very helpful to make iOS universal binaries.

(https://github.com/TimNN/cargo-lipo)

However, for android, there is no such tool and the configuration is quite bothersome.

So, you can just run andrust and write the code in rust.

### How to use

1. go root path of rust project which should be compiled as android library
2. run andrust

andrust will check NDK home, if it is not set or NDK is not present in the system, download and install it.

(https://developer.android.com/ndk/downloads)

