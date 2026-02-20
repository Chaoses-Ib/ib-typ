param(
    [string]$prof = 'debug',
    [string]$opt = 'O0',
    [switch]$r
)
if ($r) {
    $prof = 'debug'
    $opt = 'O3'
}
$target_dir = Join-Path $PSScriptRoot "../../target/wasm32-unknown-unknown/$prof"
$pkg_dir = Join-Path $PSScriptRoot "pkg"

cargo rustc -p ib-typ-ide `
    --target wasm32-unknown-unknown `
    --crate-type cdylib `
    --profile $($prof -eq 'debug' ? 'dev' : $prof)
wasm-opt $target_dir/ib_typ_ide.wasm `
    -$opt `
    --enable-bulk-memory-opt --enable-nontrapping-float-to-int `
    -o $pkg_dir/ib_typ_ide_bg.wasm
