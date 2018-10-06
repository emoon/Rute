[![Build Status](https://travis-ci.org/emoon/Rute.svg?branch=master)](https://travis-ci.org/emoon/Rute)
[![Build status](https://ci.appveyor.com/api/projects/status/sd72ee6sbm17b0l3/branch/master?svg=true)](https://ci.appveyor.com/project/emoon/rute/branch/master)

# Rute

Rute is a UI library implemented on top of [Qt](https://www.qt.io) for Rust. There are already several such projects such as [rust-qt](https://github.com/rust-qt) and [qmlrs](https://github.com/cyndis/qmlrs) so what maske Rute different?

## Overview

Rute takes a different approach than many in the way the API is generated:

* Instead of parsing the C++ headers or doing everything manually an API description file is being used.
* A C API is generated as an intermediate step but is also useable from pure C code
* All C++ generated code that is needed for the C bindings gets (optionally) generated to a shared object (dll/dylib/so) that the Rust/C API can use.
  * Only a single symbol with function pointers are exposed. This make it a good fit to pass this struct around to shared objects used by an application.
  * This also allows host application allow to use plugins (shared objects) and just pass down a single pointer for the whole UI interface.

## Details

For details on how this works see [this](https://github.com/emoon/Rute/blob/master/details.md) (WIP and a bit out of date)

## Status

Rute is currently in early development and isn't really useable except for some [basic examples](https://github.com/emoon/Rute/tree/master/rute/examples)

### Pros

* Doesn't require the user to build all bindings which some other generators do.
* Cross-platform. There is no need to generate specific bindings for each platform.
* Can keep all C++ generated code inside a single shared object. There is no need for a Rust or C program to link with any C++ code.
* The separation allows a host program to load the dll and pass the interface around very cheap (single pointer)
* Separation also allows for plugins (shared objects) to use this without linking directly with Qt making them agnostic to Qt version used.
* Slots and signals and virtual overrides works directly from Rust/C and can be set directly on objects.
* No code generation is needed to bind with Slots/signals as it is with Qt C++
* Provides a safer API than the regular C++ one.

### Cons

* Signals and slots can't easily be defined as in C++ and is only present for the ones defined in the API.
* Currently very WIP. Has (known) memory leaks that needs to be fixed.
* Added layers will impact performance. How much it will be in practice needs to be tested.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Remember if you are planning to release an application using Rute you will have use a Qt licence as well which you can read about [here](https://doc.qt.io/qt-5.6/licensing.html)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

