[![Open in Dev Containers](https://img.shields.io/static/v1?label=Dev%20Containers&message=Open&color=blue&logo=visualstudiocode)](https://vscode.dev/redirect?url=vscode://ms-vscode-remote.remote-containers/cloneInVolume?url=https://github.com/mlund/eleven)

# Eleven Interpreter for MEGA65 - The rust version

Template for projects using the mos-hardware crate.

## Getting started

The project requires [rust-mos](https://github.com/mrk-its/rust-mos) and
is setup to build for C64 by default.
A docker image of rust-mos is [available](https://hub.docker.com/r/mrkits/rust-mos) if you
do not fancy compiling LLVM.
If you want to start a new project which uses `mos-hardware`, there's a
[Github Template](https://github.com/mlund/mos-hardware-template).

### Docker and Visual Studio Code

The easiest way is to use the provided `.devcontainer.json` configuration for vscode
by clicking the _Dev Containers Open_ badge above, assuming you have VSC and Docker installed.
You can also do this manually:

1. Start Docker
2. Go to the `eleven/` directory
3. Type `code .` and follow instructions to open in dev container
