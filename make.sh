

rustup toolchain install nightly
rustup component add rust-src --toolchain nightly


# build
RUSTFLAGS="-Zlocation-detail=none" cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort  --target $(rustc +nightly -Z unstable-options --print target-spec-json | jq -r '."llvm-target"') --release

LOCATION=target/$(rustc +nightly -Z unstable-options --print target-spec-json | jq -r '."llvm-target"')/release/hexchk



upx -9 $LOCATION



# run
$LOCATION ascii.txt
ls -l $LOCATION

# now for the C
tcc -s -fno-stack-protector -ffunction-sections -fdata-sections -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-math-errno -fno-unroll-loops -fmerge-all-constants -fno-ident -flto -o hexchk_c hexchk.c

strip -s hexchk_c

./hexchk_c ascii.txt -c
ls -l hexchk_c
