# builds and copies the lib to the examples dir
cargo build --release

echo "copying lib binary"

cp -fr target/release/libmol_swatter.so ../bin/mol_swatter.so
