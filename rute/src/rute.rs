use libloading::{Library, Symbol};
use std::path::Path;
use std::fs;
use std::rc::Rc;
use auto::ffi;

pub struct Rute {
    _lib: Rc<Library>,
    pub ffi_api: *const ffi::RuteFFI,
}


#[cfg(target_os = "windows")]
const LIB_NAME: &str = "rute.dll";

#[cfg(target_os = "macos")]
const LIB_NAME: &str = "librute.dylib";

#[cfg(target_os = "linux")]
const LIB_NAME: &str = "librute.so";

impl Rute {
    ///
    /// Create a instance of Rute. By default this will try to load the dynamic library version
    /// by searching for it in various locations.
    ///
    pub fn new() -> Result<Rute, String> {
        let path = Path::new(env!("OUT_DIR")).join(LIB_NAME);

        Self::new_from_path(path.to_str().unwrap())

        //let path1 = shared_lib();
    }

    //
    // Create from a given RustFFI pointer. This allows users to link with a static
    // version of Rute instead of using the dynamic one
    //
    //fn new_from_ffi(ffi: *const ffi::RuteFFI) {
    //
    //}

    /// Specify path to load the librute from. Notice that this shouldn't include the
    /// actually file name but only the path to the filename. This make it easier for the user
    /// as filename may be different depending on platform
    pub fn new_from_path(path: &str) -> Result<Rute, String> {
        let data = match fs::metadata(path) {
            Err(e) => {
                return Err(format!("Unable to find file {}: error {}", path, e));
            }

            Ok(data) => data,
        };

        // Make sure we are trying ot open a file

        if !data.is_file() {
            return Err(format!("Rute: Path is not a file: {}", path));
        }

        // Load the library

        let lib = match Library::new(path) {
            Err(e) => return Err(format!("Found Rute sharedlib at {} but was unable to open it.
                                         This can be because of the Qt sharedlibs/dlls not being in path.
                                         Make sure they are and try again: error {}", path, e)),
            Ok(lib) => Rc::new(lib),
        };

        // Find the entry symbol in the library

        let rute_get;

        unsafe {
            let t: Result<Symbol<unsafe extern "C" fn() -> *const ffi::RuteFFI>, ::std::io::Error> = lib.get(b"rute_get\0");

            rute_get = match t {
                Ok(func) => func,
                Err(e) => {
                    return Err(format!("Loaded Rute sharedlib {} but was unable to find \"rute_get\" function in the lib: error {}", path, e));
                }
            };
        }

        Ok(Rute {
            _lib: lib.clone(),
            ffi_api: unsafe { rute_get() },
        })
    }
}

