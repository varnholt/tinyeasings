# tinyeasings

[![CI](https://github.com/varnholt/tinyeasings/actions/workflows/ci.yml/badge.svg)](https://github.com/varnholt/tinyeasings/actions/workflows/ci.yml)

Easing functions for C++23 in a single header, following the formulas from
[easings.net](https://easings.net).

- one header, no dependencies: copy `include/tinyeasings/tinyeasings.hpp` into your project
- or use it as a CMake library: `add_subdirectory`, `FetchContent` or `find_package`
- `constexpr` and `noexcept`, works with `float`, `double` and `long double`
- every curve returns exactly 0 at x = 0 and exactly 1 at x = 1
- select a curve at runtime with the `tinyeasings::Curve` enum

## Usage

```cpp
#include <tinyeasings/tinyeasings.hpp>

// x is expected in [0, 1]
const float y = tinyeasings::easeOutBounce(0.5f);

// constant-evaluated where the math allows it
static_assert(tinyeasings::easeInOutCubic(0.5) == 0.5);

// runtime selection, e.g. from a UI or a config file
const auto curve = tinyeasings::Curve::InOutElastic;
const double z = tinyeasings::ease(curve, 0.25);
std::println("{}: {}", tinyeasings::name(curve), z);

for (const auto c : tinyeasings::all_curves) { /* ... */ }
```

Inputs are not clamped. The back and elastic curves deliberately go outside
[0, 1] partway through the animation.

## API

Everything lives in namespace `tinyeasings`.

| | |
|---|---|
| `T easeIn<Family>(T x)`<br>`T easeOut<Family>(T x)`<br>`T easeInOut<Family>(T x)` | one function per curve; `Family` is `Sine`, `Quad`, `Cubic`, `Quart`, `Quint`, `Expo`, `Circ`, `Back`, `Elastic` or `Bounce`; `T` is any `std::floating_point` |
| `enum class Curve` | `InSine`, `OutSine`, `InOutSine`, ... `InOutBounce` (30 values) |
| `T ease(Curve curve, T x)` | evaluates the selected curve; an out-of-range `Curve` value falls back to linear |
| `std::string_view name(Curve curve)` | e.g. `"easeOutBounce"`; empty for an out-of-range value |
| `all_curves`, `curve_count` | all `Curve` values in declaration order, e.g. to fill a combo box |

## Integration

The header needs C++23. CI builds it with GCC 14, Clang 18 and MSVC; the header alone
also works with GCC 13. The tests and the example use `<print>`, so they need GCC 14,
Clang 18 with libstdc++ 14, or Visual Studio 2022 17.7 or newer.

### Single header

Copy `include/tinyeasings/tinyeasings.hpp`, then `#include "tinyeasings.hpp"`.

### CMake FetchContent

```cmake
include(FetchContent)
FetchContent_Declare(tinyeasings GIT_REPOSITORY https://github.com/varnholt/tinyeasings.git GIT_TAG main)
FetchContent_MakeAvailable(tinyeasings)

target_link_libraries(my_app PRIVATE tinyeasings::tinyeasings)
```

### CMake add_subdirectory

```cmake
add_subdirectory(external/tinyeasings)
target_link_libraries(my_app PRIVATE tinyeasings::tinyeasings)
```

### Installed package

```sh
cmake -S . -B build
cmake --install build --prefix /some/prefix
```

```cmake
find_package(tinyeasings 2 REQUIRED)
target_link_libraries(my_app PRIVATE tinyeasings::tinyeasings)
```

The tests and the example build only when tinyeasings is the top-level project.
Use `TINYEASINGS_BUILD_TESTS`, `TINYEASINGS_BUILD_EXAMPLES` and `TINYEASINGS_INSTALL` to override that.

## Building the tests and the plot example

```sh
cmake -S . -B build
cmake --build build
ctest --test-dir build
./build/examples/tinyeasings_plot easeOutBounce
```

```
easeOutBounce
1 |. . . . . . . . . . .** . . . . . . . . . *** . . . ********
  |                    ** *                ***  ********
  |                    *   **             **
  |                   *      **        ****
  |                  **        *********
  |                 **
  |                 *
  |                *
  |               **
  |              **
  |             **
  |            **
  |           **
  |         ***
  |        **
  |      ***
  |   ****
0 |****. . . . . . . . . . . . . . . . . . . . . . . . . . . .
  +------------------------------------------------------------
```

## Curves

Each curve has its graph next to an animation. The white dot moves at the eased
speed; the small grey dot above it moves at constant speed for comparison.

| | in | out | in/out |
|---|---|---|---|
| **Sine** | <img src="doc/0_easeInSine.png" width="120" alt="easeInSine graph"> <img src="doc/easeInSine.gif" width="120" alt="easeInSine animation"> | <img src="doc/5_easeOutSine.png" width="120" alt="easeOutSine graph"> <img src="doc/easeOutSine.gif" width="120" alt="easeOutSine animation"> | <img src="doc/10_easeInOutSine.png" width="120" alt="easeInOutSine graph"> <img src="doc/easeInOutSine.gif" width="120" alt="easeInOutSine animation"> |
| **Quad** | <img src="doc/15_easeInQuad.png" width="120" alt="easeInQuad graph"> <img src="doc/easeInQuad.gif" width="120" alt="easeInQuad animation"> | <img src="doc/21_easeOutQuad.png" width="120" alt="easeOutQuad graph"> <img src="doc/easeOutQuad.gif" width="120" alt="easeOutQuad animation"> | <img src="doc/25_easeInOutQuad.png" width="120" alt="easeInOutQuad graph"> <img src="doc/easeInOutQuad.gif" width="120" alt="easeInOutQuad animation"> |
| **Cubic** | <img src="doc/1_easeInCubic.png" width="120" alt="easeInCubic graph"> <img src="doc/easeInCubic.gif" width="120" alt="easeInCubic animation"> | <img src="doc/6_easeOutCubic.png" width="120" alt="easeOutCubic graph"> <img src="doc/easeOutCubic.gif" width="120" alt="easeOutCubic animation"> | <img src="doc/11_easeInOutCubic.png" width="120" alt="easeInOutCubic graph"> <img src="doc/easeInOutCubic.gif" width="120" alt="easeInOutCubic animation"> |
| **Quart** | <img src="doc/16_easeInQuart.png" width="120" alt="easeInQuart graph"> <img src="doc/easeInQuart.gif" width="120" alt="easeInQuart animation"> | <img src="doc/22_easeOutQuart.png" width="120" alt="easeOutQuart graph"> <img src="doc/easeOutQuart.gif" width="120" alt="easeOutQuart animation"> | <img src="doc/26_easeInOutQuart.png" width="120" alt="easeInOutQuart graph"> <img src="doc/easeInOutQuart.gif" width="120" alt="easeInOutQuart animation"> |
| **Quint** | <img src="doc/2_easeInQuint.png" width="120" alt="easeInQuint graph"> <img src="doc/easeInQuint.gif" width="120" alt="easeInQuint animation"> | <img src="doc/7_easeOutQuint.png" width="120" alt="easeOutQuint graph"> <img src="doc/easeOutQuint.gif" width="120" alt="easeOutQuint animation"> | <img src="doc/12_easeInOutQuint.png" width="120" alt="easeInOutQuint graph"> <img src="doc/easeInOutQuint.gif" width="120" alt="easeInOutQuint animation"> |
| **Expo** | <img src="doc/17_easeInExpo.png" width="120" alt="easeInExpo graph"> <img src="doc/easeInExpo.gif" width="120" alt="easeInExpo animation"> | <img src="doc/23_easeOutExpo.png" width="120" alt="easeOutExpo graph"> <img src="doc/easeOutExpo.gif" width="120" alt="easeOutExpo animation"> | <img src="doc/27_easeInOutExpo.png" width="120" alt="easeInOutExpo graph"> <img src="doc/easeInOutExpo.gif" width="120" alt="easeInOutExpo animation"> |
| **Circ** | <img src="doc/3_easeInCirc.png" width="120" alt="easeInCirc graph"> <img src="doc/easeInCirc.gif" width="120" alt="easeInCirc animation"> | <img src="doc/8_easeOutCirc.png" width="120" alt="easeOutCirc graph"> <img src="doc/easeOutCirc.gif" width="120" alt="easeOutCirc animation"> | <img src="doc/13_easeInOutCirc.png" width="120" alt="easeInOutCirc graph"> <img src="doc/easeInOutCirc.gif" width="120" alt="easeInOutCirc animation"> |
| **Back** | <img src="doc/18_easeInBack.png" width="120" alt="easeInBack graph"> <img src="doc/easeInBack.gif" width="120" alt="easeInBack animation"> | <img src="doc/20_easeOutBack.png" width="120" alt="easeOutBack graph"> <img src="doc/easeOutBack.gif" width="120" alt="easeOutBack animation"> | <img src="doc/28_easeInOutBack.png" width="120" alt="easeInOutBack graph"> <img src="doc/easeInOutBack.gif" width="120" alt="easeInOutBack animation"> |
| **Elastic** | <img src="doc/4_easeInElastic.png" width="120" alt="easeInElastic graph"> <img src="doc/easeInElastic.gif" width="120" alt="easeInElastic animation"> | <img src="doc/9_easeOutElastic.png" width="120" alt="easeOutElastic graph"> <img src="doc/easeOutElastic.gif" width="120" alt="easeOutElastic animation"> | <img src="doc/14_easeInOutElastic.png" width="120" alt="easeInOutElastic graph"> <img src="doc/easeInOutElastic.gif" width="120" alt="easeInOutElastic animation"> |
| **Bounce** | <img src="doc/19_easeInBounce.png" width="120" alt="easeInBounce graph"> <img src="doc/easeInBounce.gif" width="120" alt="easeInBounce animation"> | <img src="doc/24_easeOutBounce.png" width="120" alt="easeOutBounce graph"> <img src="doc/easeOutBounce.gif" width="120" alt="easeOutBounce animation"> | <img src="doc/29_easeInOutBounce.png" width="120" alt="easeInOutBounce graph"> <img src="doc/easeInOutBounce.gif" width="120" alt="easeInOutBounce animation"> |

To regenerate the animations from the header (needs Pillow):

```sh
./build/examples/tinyeasings_plot --csv > curves.csv
python tools/make_gifs.py curves.csv doc
```

## Rust

**[Interactive Rust demo](https://varnholt.github.io/tinyeasings/)** — [native and browser build instructions](rust/demo/README.md).

[`rust/`](rust/README.md) contains the native `tinyeasings` Rust crate: no dependencies and no C++ toolchain required. It includes examples, tests against the C++ implementation, and Cargo packaging. See the [Rust README](rust/README.md) for the API and local installation.

## License

[CC0 1.0](LICENSE)
