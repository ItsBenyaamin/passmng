use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

fn main() {
    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();
    let is_windows = host.contains("windows") && target.contains("windows");

    let lib_dir = env("OPENSSL_LIB_DIR").map(PathBuf::from);
    let inc_dir = env("OPENSSL_INCLUDE_DIR").map(PathBuf::from);
    let mut use_openssl = false;

    let (lib_dir, inc_dir) = match (lib_dir, inc_dir) {
        (Some(lib_dir), Some(inc_dir)) => {
            use_openssl = true;
            (lib_dir, inc_dir)
        }
        (lib_dir, inc_dir) => match find_openssl_dir(&host, &target) {
            None => {
                if is_windows && !cfg!(feature = "bundled-sqlcipher-vendored-openssl") {
                    panic!("Missing environment variable OPENSSL_DIR or OPENSSL_DIR is not set")
                } else {
                    (PathBuf::new(), PathBuf::new())
                }
            }
            Some(openssl_dir) => {
                let lib_dir = lib_dir.unwrap_or_else(|| openssl_dir.join("lib"));
                let inc_dir = inc_dir.unwrap_or_else(|| openssl_dir.join("include"));

                assert!(
                    Path::new(&lib_dir).exists(),
                    "OpenSSL library directory does not exist: {}",
                    lib_dir.to_string_lossy()
                );

                if !Path::new(&inc_dir).exists() {
                    panic!(
                        "OpenSSL include directory does not exist: {}",
                        inc_dir.to_string_lossy()
                    );
                }

                use_openssl = true;
                (lib_dir, inc_dir)
            }
        },
    };

    if is_windows && use_openssl {
        link_openssl_static(&lib_dir, &inc_dir);
    }
}

fn link_openssl_static(lib_dir: &Path, _inc_dir: &Path) {
    println!("cargo:rustc-link-lib=static=libcrypto_static");
    println!(
        "cargo:rustc-link-search=native={}",
        lib_dir.to_string_lossy()
    );
}

fn find_openssl_dir(_host: &str, _target: &str) -> Option<PathBuf> {
    let openssl_dir = env("OPENSSL_DIR");
    openssl_dir.map(PathBuf::from)
}

fn env(name: &str) -> Option<OsString> {
    let prefix = env::var("TARGET").unwrap().to_uppercase().replace('-', "_");
    let prefixed = format!("{prefix}_{name}");
    let var = env::var_os(prefixed);

    match var {
        None => env::var_os(name),
        _ => var,
    }
}
