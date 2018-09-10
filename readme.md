[![Build Status](https://travis-ci.org/emoon/Rute.svg?branch=experiments)](https://travis-ci.org/emoon/Rute)
[![Generator Coverage](https://codecov.io/gh/emoon/Rute/branch/master/graph/badge.svg)](https://codecov.io/gh/emoon/Rute)

# Rute

Rute is a Qt binding wrapper and generator for Rust and C. It allows usage of the [Qt](https://www.qt.io) C++ API from Rust and C. There are already several such projects such as [rust-qt](https://github.com/rust-qt) and [qmlrs](https://github.com/cyndis/qmlrs) so what maske Rute different?

## Overview

Rute takes a different approach than many in the way the API is generated:

* Instead of parsing the C++ headers or doing everything manually an API description file is being used.
* A C API is generated as an intermediate step but is also useable from pure C code
* All C++ generated code that is needed for the C bindings gets generated to a shared object (dll/dylib/so) that the Rust/C api can use.
  * Only a single symbol with function pointers are exposed. This make it a good fit to pass this struct around to shared objects used by an application.
  * This allows host application allow to use plugins (shared objects) and just pass down a single pointer for the whole UI interface.

## Details

For details on how this works see [this](https://github.com/emoon/Rute/blob/master/details.md) (WIP)

## Status

Rute is currently being used in [HippoPlayer](https://github.com/emoon/HippoPlayer) and is very tailored for what I need. It's definitely not a "one size fits all" approach but something that works for me. Here is a list of what I consider pros and cons with it

### Pros

* Doesn't require the user to build all bindings which some other generators do.
* Cross-platform. There is no need to generate specific bindings for each platform.
* Keeps all C++ generated code inside a single shared object. There is no need for a Rust or C program to link with any C++ code.
* The separation allows a host program to load the dll and pass the interface around very cheap (single pointer)
* Separation also allows for plugins (shared objects) to use this without linking directly with Qt making them agnostic to Qt version used.
* Slots and signals and virtual overrides works directly from Rust/C and can be set directly on objects.
* No code generation is needed to bind with Slots/signals as it is with Qt C++

### Cons

* Limited. Only exposes a given part of the API.
* Some functions needs to be implemented manually in C++ but only the C++ and not the rest of the bindings.
* The Rust API is currently quite unsafe while it looks safe. This is something I want to address.
* Signals and slots can't easily be defined as in C++ and is only present for the ones defined in the API.
* Currently very WIP. Has (known) memory leaks that needs to be fixed.
* Currently the code for the generator is quite ugly and something that needs to be cleaned up.

### Usage

I hope to set up some basic test case at some point but currently to see usage see [HippoPlayer](https://github.com/emoon/HippoPlayer)
