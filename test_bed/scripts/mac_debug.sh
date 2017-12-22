#!/bin/bash
cargo build --manifest-path=../generator/Cargo.toml && ../generator/target/debug/ui_gen -a ../generator/src/api.def -c src/qt/c_api.h -q src/qt/wrui_qt.cpp -b src/qt/qt_api_gen.h -r src/wrui/src/lib.rs -f src/wrui/ffi_gen.rs && tundra macosx-clang-debug

