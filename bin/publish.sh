for d in ./crates/*/; do
    cd $d
    cargo publish
    cd ../..
done