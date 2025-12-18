param(
    [string]$prof = 'debug',
    [string]$opt = 'O0',
    [switch]$r
)
if ($r) {
    $prof = 'release-os'
    $opt = 'O3'
}
cargo build --target wasm32-unknown-unknown `
    --profile $($prof -eq 'debug' ? 'dev' : $prof)
wasm-opt ./target/wasm32-unknown-unknown/$prof/ib_typ.wasm `
    -$opt `
    --enable-bulk-memory-opt --enable-nontrapping-float-to-int `
    -o ./src/ib_typ.wasm
