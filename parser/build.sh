# builds and copies the lib to the examples dir
cargo build --release

echo "copying lib binary"

cp -fr target/release/libmol_swatter.so ../examples
mv -f ../examples/libmol_swatter.so ../examples/mol_swatter.so
