function ffibuild() {
    set -eu
    cargo build
    cbindgen --lang c --config cbindgen.toml --crate testlib --output rustffi.h
    mv rustffi.h csrc 
    cp ./target/debug/libtestlib.so csrc 
    pushd csrc 
    gcc main.c -L . -l testlib 
    ./a.out 
    popd
}
ffibuild