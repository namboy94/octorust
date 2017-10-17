# octorust

octorust is a python application that makes use of `rustc`, `cargo` and `gcc`
to compile applications written in rust for irtss/octopos.

# Installation

## Prerequisites

A working `rustup` is assumed to exist on the system. If not, install
it either using your package manager (`sudo pacman -S rustup` on Arch Linux,
other distros that don't have rustup packaged can use the 
[rustup installation script](https://www.rustup.rs/)).

The currently used `libcore` requires rustc nightly version 1.19.0,
which can be set using:

    $rustup install nightly-2017-06-01
    $rustup default nightly-2017-06-01
    
For x86guest support, install the `i686-unknown-linux-gnu` target using:

    $rustup target install i686-unknown-linux-gnu

A `gcc` with support for both 32-bit and 64-bit is also required (Install the
`gcc-multilib` package)

A `sparc-elf-gcc` should also be installed and in the path, but if this is
not the case, the installer will download one and store it in
`~/.octorust/toolchains/sparc-elf`.

## Installation from source

From the root directory, execute `python setup.py install --user` (The 
`--user` flag tells setuptools to install the application for the
current user instead of system-wide).

This will:

    1. Install the python application
    2. Create the directory ~/.octorust
    3. Copy octolib and any dependencies like libcore and libc to ~/.octorust
    4. Compile the libcore and libc crates and install them in the rustup
       toolchain installation directory for SPARC
    5. Install a sparc-elf-gcc if one could not be found in $PATH
    
Once installed, make sure that `~/.local/bin` is in your path,
which should make it possible to call the `octorust` command.

## Post-Installation

Once everything is installed, try to compile the sample applications
in `sample/`. For example:

    $ octorust sample/cargo_sample -o cargo_out -a x86guest -v generic
    
This will attempt to compile the sample application for the `x86guest`
architecture and the `generic` variant. The corresponding IRTSS release
will be downloaded to ~/.octorust/irtss-current. For this to work,
you need a ~/.netrc file containing the following:

machine www4.cs.fau.de
login <username>
password <password>

Once the IRTSS release has been downloaded, the application will
compile the rust crate/file using cargo/rustc and link it using the
corresponding gcc to the IRTSS release.
