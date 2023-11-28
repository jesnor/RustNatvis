# Rust Natvis

Generates a more informative summary for Rust enums when debugging Rust code in Windows. This requires using a debugger that uses Natvis files, for example WinDbg (doesn't work with LLDB for example).

## How to Use

Copy the [intrinsic.natvis](intrinsic.natvis) file to the `\Users\<user name>\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\etc\` folder, replacing the original file, 
and then re-build your project in debug mode.

## Use WinDbg in VS Code

Install [the MS C/C++ extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode.cpptools) in VS Code. 
In VS Code change the setting `rust-analyzer.debug.engine` to `ms-vscode.cpptools`. You can also copy the [launch.json](launch.json) to the `.vscode\` folder in your Rust project and then launch the VS Code debugger. 
