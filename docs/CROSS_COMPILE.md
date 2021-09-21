# write once run anywhere - 러스트?!

안드로이드 팀에서 기존 C/C++로 개발되던 안드로이드 OS 개발에 [Rust도 사용하기로 했다는 글](https://security.googleblog.com/2021/04/rust-in-android-platform.html)을 본지도 벌써 6개월이 다 되어감에 따라 안드로이드 앱 개발을 할 때 일부 라이브러리들을 Rust로 구현했던 경험을 정리해본다.



### 준비사항

- Android  SDK & NDK
- Rustup

안드로이드 SDK는 안드로이드용 라이브러리와 앱을 만들 예정이니 필요하다. NDK는 Rust로 작성한 코드를 안드로이드 대상 플랫폼으로 크로스 컴파일 할 때 사용된다. 마지막으로 Rustup은 Rust를 설치하는 방법은 여러가지가 있지만 Rustup이 가장 쉽고 편하게 버전 및 툴체인들을 관리할 수 있어서 이 글에서는 Rustup을 사용하여 Rust를 관리하는 시스템 기준으로 작성됐다.



### Rust가 지원하는 플랫폼

우선 rust가 지원하는 platform을 확인해보면 아래화면같이 나온다.

```shell
$ rustc --print target-list | pr -tw100 --columns 3
aarch64-apple-darwin             i586-pc-windows-msvc             riscv64gc-unknown-none-elf
aarch64-apple-ios                i586-unknown-linux-gnu           riscv64imac-unknown-none-elf
aarch64-apple-ios-macabi         i586-unknown-linux-musl          s390x-unknown-linux-gnu
aarch64-apple-ios-sim            i686-apple-darwin                s390x-unknown-linux-musl
aarch64-apple-tvos               i686-linux-android               sparc-unknown-linux-gnu
aarch64-fuchsia                  i686-pc-windows-gnu              sparc64-unknown-linux-gnu
aarch64-linux-android            i686-pc-windows-msvc             sparc64-unknown-netbsd
aarch64-pc-windows-msvc          i686-unknown-freebsd             sparc64-unknown-openbsd
aarch64-unknown-freebsd          i686-unknown-haiku               sparcv9-sun-solaris
aarch64-unknown-hermit           i686-unknown-linux-gnu           thumbv4t-none-eabi
aarch64-unknown-linux-gnu        i686-unknown-linux-musl          thumbv6m-none-eabi
aarch64-unknown-linux-gnu_ilp32  i686-unknown-netbsd              thumbv7a-pc-windows-msvc
aarch64-unknown-linux-musl       i686-unknown-openbsd             thumbv7a-uwp-windows-msvc
aarch64-unknown-netbsd           i686-unknown-uefi                thumbv7em-none-eabi
aarch64-unknown-none             i686-uwp-windows-gnu             thumbv7em-none-eabihf
aarch64-unknown-none-softfloat   i686-uwp-windows-msvc            thumbv7m-none-eabi
aarch64-unknown-openbsd          i686-wrs-vxworks                 thumbv7neon-linux-androideabi
aarch64-unknown-redox            mips-unknown-linux-gnu           thumbv7neon-unknown-linux-gnueab
aarch64-uwp-windows-msvc         mips-unknown-linux-musl          thumbv7neon-unknown-linux-muslea
aarch64-wrs-vxworks              mips-unknown-linux-uclibc        thumbv8m.base-none-eabi
aarch64_be-unknown-linux-gnu     mips64-unknown-linux-gnuabi64    thumbv8m.main-none-eabi
aarch64_be-unknown-linux-gnu_ilp mips64-unknown-linux-muslabi64   thumbv8m.main-none-eabihf
arm-linux-androideabi            mips64el-unknown-linux-gnuabi64  wasm32-unknown-emscripten
arm-unknown-linux-gnueabi        mips64el-unknown-linux-muslabi64 wasm32-unknown-unknown
arm-unknown-linux-gnueabihf      mipsel-sony-psp                  wasm32-wasi
arm-unknown-linux-musleabi       mipsel-unknown-linux-gnu         wasm64-unknown-unknown
arm-unknown-linux-musleabihf     mipsel-unknown-linux-musl        x86_64-apple-darwin
armebv7r-none-eabi               mipsel-unknown-linux-uclibc      x86_64-apple-ios
armebv7r-none-eabihf             mipsel-unknown-none              x86_64-apple-ios-macabi
armv4t-unknown-linux-gnueabi     mipsisa32r6-unknown-linux-gnu    x86_64-apple-tvos
armv5te-unknown-linux-gnueabi    mipsisa32r6el-unknown-linux-gnu  x86_64-fortanix-unknown-sgx
armv5te-unknown-linux-musleabi   mipsisa64r6-unknown-linux-gnuabi x86_64-fuchsia
armv5te-unknown-linux-uclibceabi mipsisa64r6el-unknown-linux-gnua x86_64-linux-android
armv6-unknown-freebsd            msp430-none-elf                  x86_64-pc-solaris
armv6-unknown-netbsd-eabihf      nvptx64-nvidia-cuda              x86_64-pc-windows-gnu
armv7-apple-ios                  powerpc-unknown-linux-gnu        x86_64-pc-windows-msvc
armv7-linux-androideabi          powerpc-unknown-linux-gnuspe     x86_64-sun-solaris
armv7-unknown-freebsd            powerpc-unknown-linux-musl       x86_64-unknown-dragonfly
armv7-unknown-linux-gnueabi      powerpc-unknown-netbsd           x86_64-unknown-freebsd
armv7-unknown-linux-gnueabihf    powerpc-unknown-openbsd          x86_64-unknown-haiku
armv7-unknown-linux-musleabi     powerpc-wrs-vxworks              x86_64-unknown-hermit
armv7-unknown-linux-musleabihf   powerpc-wrs-vxworks-spe          x86_64-unknown-illumos
armv7-unknown-netbsd-eabihf      powerpc64-unknown-freebsd        x86_64-unknown-l4re-uclibc
armv7-wrs-vxworks-eabihf         powerpc64-unknown-linux-gnu      x86_64-unknown-linux-gnu
armv7a-none-eabi                 powerpc64-unknown-linux-musl     x86_64-unknown-linux-gnux32
armv7a-none-eabihf               powerpc64-wrs-vxworks            x86_64-unknown-linux-musl
armv7r-none-eabi                 powerpc64le-unknown-linux-gnu    x86_64-unknown-netbsd
armv7r-none-eabihf               powerpc64le-unknown-linux-musl   x86_64-unknown-none-hermitkernel
armv7s-apple-ios                 riscv32gc-unknown-linux-gnu      x86_64-unknown-none-linuxkernel
asmjs-unknown-emscripten         riscv32gc-unknown-linux-musl     x86_64-unknown-openbsd
avr-unknown-gnu-atmega328        riscv32i-unknown-none-elf        x86_64-unknown-redox
bpfeb-unknown-none               riscv32imac-unknown-none-elf     x86_64-unknown-uefi
bpfel-unknown-none               riscv32imc-unknown-none-elf      x86_64-uwp-windows-gnu
hexagon-unknown-linux-musl       riscv64gc-unknown-linux-gnu      x86_64-uwp-windows-msvc
i386-apple-ios                   riscv64gc-unknown-linux-musl     x86_64-wrs-vxworks
```

여기서 지원하는 플랫폼은 target triple 형태로 표시된다.

target triple은 아래와 같이 구성된다.

- Architecture : 바이너리가 어떤 instruction set 위에서 돌아가야하는지를 나타낸다.
- Vendor : 보통 디바이스의 제조사를 이야기하며 linux는 embedded linux까지 포함하여 대부분 unknown으로 표시된다.
- System : 대부분 많이 알고 있는 Operating System을 의미한다.
- ABI : mac과 bsd에서는 복수의 ABI를 제공하지 않기때문에 대부분 생략되어 "triple"에는 속하지 않지만 linux나 windows등에서는 compiler에 따라  function name manglingling 규칙이 등이 달라 Binary Interface를 표시해준다.



### 안드로이드 툴체인 설치

위에서 보면 알 수 있듯이 rust에서 지원가능한 android system은 다음과 같다.

`aarch64-linux-android`, `arm-linux-androideabi`, `armv7-linux-androideabi`, `i686-linux-android`, `x86_64-linux-android`

위의 툴체인을 설치해보자

```shell
$ rustup target add aarch64-linux-android arm-linux-androideabi armv7-linux-androideabi i686-linux-android x86_64-linux-android
```



정상적으로 설치가 됐다면 installed targets for active toolchain에  표시가 될 것이다.

```shell
$ rustup show
Default host: x86_64-unknown-linux-gnu
rustup home:  /home/username/.rustup

installed toolchains
--------------------

stable-x86_64-unknown-linux-gnu (default)

installed targets for active toolchain
--------------------------------------

aarch64-linux-android
arm-linux-androideabi
armv7-linux-androideabi
i686-linux-android
i686-pc-windows-msvc
x86_64-linux-android
x86_64-pc-windows-msvc
x86_64-unknown-linux-gnu

active toolchain
----------------

stable-x86_64-unknown-linux-gnu (default)
rustc 1.54.0 (a178d0322 2021-07-26)
```



### 안드로이드 NDK 설정

rust code를 cross compile하는 경우 최종 결과물을 얻기 위해서는 안드로이드 NDK에서 제공하는 ar(archiver)와 linker를 사용해야한다.

NDK의 버전이 r19이하라면 standalone tool을 따로 빌드해주는 script를 실행하여 사용해야했지만 최근 NDK는 더 이상 standalone 툴을 따로 만들어줄 필요가 없게 업데이트 됐다. 해당 내용은 [다음링크](https://developer.android.com/ndk/guides/standalone_toolchain)에서 확인할 수  있다.



### 코드 작성

rust는 기본적으로 c와의 연동(ffi)가 매우 훌륭하므로 문자열을  반환하는 셈플 코드를 작성해보자.

아래 코드를 `src/lib.rs`에 작성한다. 기본적으로 CLI 프로그램을 만들때의 엔트리포인트는 `main.rs`이지만 우리가 만드는 것은 실행파일 포멧이 아닌 library형태이기 때문에 `main.rs`는 사용하지 않는다.

```rust
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn rust_greeting(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };

    CString::new("Hello ".to_owned() + recipient)
        .unwrap()
        .into_raw()
}
```

위 코드는 레퍼런스로 참고한  사이트에서 예로 든 샘플코드이다.

일반적인 rust코드와 다른 점은 `#[no_mangle]`이라는 매크로, `extern "C"`구문, 그리고 `ffi::{CStr, CString}`사용등이 있다.

일반적으로 rust는 문자열을 다룰 때 str의 reference인 &str와 String을 사용하는데 CStr과 CString은 c와의 연동을 위한 자료형으로 생각하면 된다.

no_mangle 매크로의 경우 컴파일러마다 함수 이름을 변경하는 방식이 모두 다르므로 표준 C에 맞게 함수명 변경을 하지 않도록 하는것이다. 이는 C++와 C를 연동할 때도 같은 처리를 하는 매크로를 지원하고 있다.



위 코드와 함께 Cargo.toml에 어떤 형식의 라이브러리를 빌드할지에 대한 정보도 제공해준다.

정적 라이브러리와  동적 라이브러리 둘다 빌드를 할 것이기 때문에 Cargo.toml은 아래와 같은 형태로 구성될 것이다.

```toml
[package]
name = "rust_android"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

[lib]
name = "rust_android_lib"
crate-type = ["staticlib", "cdylib"]
```



### build 해보기

이제 작성한 코드를 빌드를 해서 안드로이드용 정적 라이브러리 또는 동적 라이브러리를 얻을 것이다.

러스트 코드를 빌드를 플랫폼별로 빌드를 할 때 NDK의 ar(archiver)와 linker를 사용한다고 했는데 이 설정은 $project_root/.cargo/config 파일로 설정해준다.

```toml
[target.aarch64-linux-android]
ar = "$NDK_TOOL_ROOT/arm64/bin/aarch64-linux-android-ar"
linker = "$NDK_TOOL_ROOT/arm64/bin/aarch64-linux-android-clang"

[target.armv7-linux-androideabi]
ar = "$NDK_TOOL_ROOT/arm/bin/arm-linux-androideabi-ar"
linker = "$NDK_TOOL_ROOT/arm/bin/arm-linux-androideabi-clang"

[target.i686-linux-android]
ar = "$NDK_TOOL_ROOT/x86/bin/i686-linux-android-ar"
linker = "$NDK_TOOL_ROOT/x86/bin/i686-linux-android-clang"

[target.x86_64-linux-android]
ar = "$NDK_TOOL_ROOT/xx86_6486/bin/x86_64-linux-android-ar"
linker = "$NDK_TOOL_ROOT/x86_64/bin/x86_64-linux-android-clang"
```

애석하게도 config파일은 환경변수를 인식하지 못한다.

셈플처럼 설정을 해주고 `$NDK_TOOL_ROOT`를 system 환경변수로 등록해줘도 저 코드는 동작하지 않을것이다. 그러므로  시스템마다 full_path를 입력해줘야한다.

만약 해당 코드를 여러 플랫폼에서 사용해야한다면 [build.rs](https://doc.rust-lang.org/cargo/reference/build-scripts.html)를 활용해 config파일을 build시에 생성해주는것도 고려할만 할 것이다.



config를 마무리 했다면 다음 command로 라이브러리들을 각 플랫폼에 맞게 빌드한다.

```shell
$ cargo build --target aarch64-linux-android --release
$ cargo build --target armv7-linux-androideabi --release
$ cargo build --target i686-linux-android --release
$ cargo build --target x86_64-linux-android --release
```

빌드가 성공했다면 /target 디렉토리에 각 플랫폼별 디렉토리가 생성돼있고 `librust_android_lib.a`와 `librust_android_lib.so`파일을 찾을 수 있을 것이다.



### JNI로 연결하기

안드로이드에서는 `a`나 `so`파일을 사용하려면 NDK를 이용하여 bridge를 만들어줘야한다. iOS와의 공용으로 사용하려면 C/C++ bridge와 JNI까지 모두 손수 작성할 수도 있겠지만 이 글에서는 안드로이드만 다룰것이기 때문에 [jni-rs](https://github.com/jni-rs/jni-rs) crate을 사용하기로 한다.



`jni-rs`를 사용하기 위해 `Cargo.toml`에 아래 의존성(dependencies)을 추가해준다.

```toml
[target.'cfg(target_os="android")'.dependencies]
jni = { version = "0.5", default-features = false }
```

jni는 안드로이드의 연결에서만 사용할 것이므로(해당 라이브러리를 안드로이드 이외에 JVM이 돌아가는 시스템에 연결하고  싶다면 `target_os`부분을 추가하거나 제거한다) `target_os`부분을 추가했다.

추가로 `crate-type`부분을 아래처럼 수정해준다.

```toml
crate-type = ["dylib"]
```

위에서 `staticlib`과 `cdylib`은 c bridge를 위한 library를 만들 때 사용하지만 이번에는 jni로 c bridge까지를 한꺼번에 묶은 형태의 라이브러리를 만들것이다.

`Cargo.toml`의 설정이 완료됐다면 `src/lib.rs`에 아래 코드를 추가한다.

```rust
#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jstring};

    #[no_mangle]
    pub unsafe extern fn Java_com_getmiso_greetings_RustGreetings_greeting(env: JNIEnv, _: JClass, java_pattern: JString) -> jstring {
        // Our Java companion code might pass-in "world" as a string, hence the name.
        let world = rust_greeting(env.get_string(java_pattern).expect("invalid pattern string").as_ptr());
        // Retake pointer so that we can use it below and allow memory to be freed when it goes out of scope.
        let world_ptr = CString::from_raw(world);
        let output = env.new_string(world_ptr.to_str().unwrap()).expect("Couldn't create java string!");

        output.into_inner()
    }
}
```

첫번째 매크로는 `target_os`를 설정해주는것으로 다른 target들에서는 해당 코드는 빌드에 포함하지 않는다는 의미이다.

두번째 줄에 사용된 `non_snake_case`허용에 대한 매크로가 없다면 코드를 컴파일 할 때 `Java_com_getmiso_greetings_RustGreetings_greeting`함수의 이름 때문에 warning이 나올것이다. 이는 rust의 naming convention과 맞지 않은 이름이기 때문이며 문제는 저 이름이 JNI를 사용하기 위해서는 package명과 class, method name을 맞춰줘야하는 강제적 사항이기 때문에 매크로로 해당 이름을 허용하는 것이다. Rust의 name convention에 대한 정리는 [이곳](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)에서 볼 수 있다.



이제 아까와같이 각 플랫폼별로 다시 컴파일을 하면 `so`파일을 얻을 수 있을것이다.

여기까지 했으면 rust로 만드는 라이브러리는 작업이 끝났다.

다음은 이 라이브러리를 안드로이드앱에서 사용하는 방법에 대해 알아보자.









Reference

https://security.googleblog.com/2021/04/rust-in-android-platform.html

https://github.com/japaric/rust-cross

https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html

https://developer.android.com/ndk/downloads

https://developer.android.com/ndk/guides/standalone_toolchain

https://doc.rust-lang.org/cargo/reference/build-scripts.html

https://github.com/jni-rs/jni-rs

https://doc.rust-lang.org/1.0.0/style/style/naming/README.html

https://developer.android.com/training/articles/perf-jni
