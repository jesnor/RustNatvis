# Rust Natvis

Generates a more informative summary for enums when debugging Rust code in Windows. This requires using a debugger that reads Natvis files, for example WinDbg (doesn't work with LLDB for example).

## How to Use

Copy the [intrinsic.natvis](intrinsic.natvis) file to the `\Users\<user name>\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\etc\` folder, replacing the original file, 
and then re-build your project in debug mode.

## In VS Code

1. Install [the MS C/C++ extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode.cpptools) in VS Code.
2. Change the setting `rust-analyzer.debug.engine` to `ms-vscode.cpptools`.
3. Optionally copy the [launch.json](.vscode/launch.json) to the `.vscode\` folder in your Rust project and then launch the VS Code debugger. 
