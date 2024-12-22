# sdl3_gravisim
- https://github.com/bcamp1/Gravisim

# sdl2 gravisim은 변환 완료(241222)
- 요즘 러스트 버젼 1.83으로 살려냄
  - https://github.com/YoungHaKim7/sdl2_gravisim

<hr />

# (241222)sdl3-sys: Low level Rust bindings for SDL 3

- ttf 아직 구현 안된듯 기다리던가.  구현하던가.
- https://docs.rs/sdl3-sys/latest/x86_64-pc-windows-msvc/sdl3_sys/index.html
- https://docs.rs/sdl3-sys/latest/sdl3_sys/

- This version of sdl3-sys has bindings for SDL version 3.1.6-preview and earlier.

- SDL 3 is ABI stable as of the 3.1.3 preview release, but sdl3-sys is new and may have bugs. Please submit an issue at github if you have any issues or comments!

- Known issues:

```
Satellite libraries (mixer, image, ttf) aren’t available yet
There are no tests yet, except for static asserts translated from the original headers
Some less common targets are missing detection or features to enable corresponding SDL features
```


