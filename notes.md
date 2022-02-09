```
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass
$env:RUST_BACKTRACE = "1";
yarn tauri dev
```
