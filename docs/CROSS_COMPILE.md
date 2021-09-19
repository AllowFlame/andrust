# write once run anywhere - 러스트?!

안드로이드 팀에서 기존 C/C++로 개발되던 안드로이드 OS 개발에 [Rust도 사용하기로 했다는 글](https://security.googleblog.com/2021/04/rust-in-android-platform.html)을 본지도 벌써 6개월이 다 되어감에 따라 안드로이드 앱 개발을 할 때 일부 라이브러리들을 Rust로 구현했던 경험을 정리해본다.



### 준비사항

- Android  SDK & NDK
- Rustup

안드로이드 SDK는 안드로이드용 라이브러리와 앱을 만들 예정이니 필요하다. NDK는 Rust로 작성한 코드를 안드로이드 대상 플랫폼으로 크로스 컴파일 할 때 사용된다. 마지막으로 Rustup은 Rust를 설치하는 방법은 여러가지가 있지만 Rustup이 가장 쉽고 편하게 버전 및 툴체인들을 관리할 수 있어서 이 글에서는 Rustup을 사용하여 Rust를 관리하는 시스템 기준으로 작성됐다.



### Rust가 지원하는 플랫폼

```shell
$ rustc --print target-list | pr -tw100 --columns 3
aarch64-apple-ios                i686-pc-windows-gnu              x86_64-apple-darwin
aarch64-linux-android            i686-pc-windows-msvc             x86_64-apple-ios
aarch64-unknown-linux-gnu        i686-unknown-dragonfly           x86_64-pc-windows-gnu
arm-linux-androideabi            i686-unknown-freebsd             x86_64-pc-windows-msvc
arm-unknown-linux-gnueabi        i686-unknown-linux-gnu           x86_64-rumprun-netbsd
arm-unknown-linux-gnueabihf      i686-unknown-linux-musl          x86_64-sun-solaris
armv7-apple-ios                  le32-unknown-nacl                x86_64-unknown-bitrig
armv7-unknown-linux-gnueabihf    mips-unknown-linux-gnu           x86_64-unknown-dragonfly
armv7s-apple-ios                 mips-unknown-linux-musl          x86_64-unknown-freebsd
asmjs-unknown-emscripten         mipsel-unknown-linux-gnu         x86_64-unknown-linux-gnu
i386-apple-ios                   mipsel-unknown-linux-musl        x86_64-unknown-linux-musl
i586-unknown-linux-gnu           powerpc-unknown-linux-gnu        x86_64-unknown-netbsd
i686-apple-darwin                powerpc64-unknown-linux-gnu      x86_64-unknown-openbsd
i686-linux-android               powerpc64le-unknown-linux-gnu
```







Reference

https://security.googleblog.com/2021/04/rust-in-android-platform.html

https://github.com/japaric/rust-cross

https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html


