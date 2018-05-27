# octorust

[![pipeline status](https://gitlab.namibsun.net/kit/invasive-rust/octorust/badges/master/pipeline.svg)](https://gitlab.namibsun.net/kit/invasive-rust/octorust/commits/master)

![Logo](resources/logo/logo-readme.png)

`octorust` is a python application that makes use of `rustc`, `cargo` and `gcc`
to compile applications written in rust for use in invasive Platforms
utilizing OctoPOS/iRTSS.

The program has been written in the course of my bachelor thesis, which can be
found [here](https://pp.ipd.kit.edu/uploads/publikationen/krumrey17bachelorarbeit.pdf).

# Installation

`octorust` has been tested to work on Arch Linux and Ubuntu Linux. It should
work on all Linux distributions and probably on MacOS and Windows, that has not been
tested though.

A script for automatic installation on Ubuntu systems has been provided. To install
`octorust` using this script, simply run `./ubuntu_setup.sh`.

## Prerequisites

A working `rustup` is assumed to exist on the system. If not, install
it either using your package manager (`sudo pacman -S rustup` on Arch Linux,
other distros that don't have rustup packaged can use the 
[rustup installation script](https://www.rustup.rs/)).

`octorust` is currently using dependencies that require the use of a specific nigthly
`rustc`-compiler. To be able to compile invasie rust applications, this nightly compiler
should be set using the following commands

    $rustup install nightly-2017-06-01
    $rustup default nightly-2017-06-01
    
For x86guest support, install the `i686-unknown-linux-gnu` target using:

    $rustup target install i686-unknown-linux-gnu

A `gcc` with support for both 32-bit and 64-bit is also required
(On most distibutions, installing the `gcc-multilib` package should to the trick)

A `sparc-elf-gcc` compiler should also be installed and in the path, but if this is
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
       toolchain installation directory
    5. Install a sparc-elf-gcc if one could not be found in $PATH
    
Once installed, make sure that `~/.local/bin` is in your path,
which should make it possible to call the `octorust` command.

## Post-Installation

Once everything is installed, you will need to install an OctoPOS/iRTSS
build. To do so, 



try to compile one of the applications inside
the `eval` directory. To do so, execute the following command:

    $ octorust --fetch-irtss -i VERSION

You can set the `-i` option to specify a release date of the iRTSS version to
use. If ommited, `octorust` will fetch the most current release.
The utilized iRTSS version during development was `2017-06-07`.

To ensure that you have the correct access rights to download iRTSS builds,
please ensure that you have a `~/.netrc` file in your home directory
with the following structure:

    machine www4.cs.fau.de
    login <username>
    password <password>

With an iRTSS build installed, we can now compile invasive rust programs.
To do so, run the following command:

    $ octorust eval/minimal-infect -a x86guest -v generic -o out -i 2017-06-07
    
This will attempt to compile the sample application for the `x86guest`
architecture and the `generic` variant. The `2017-06-07` iRTSS release
will be used. Once compiled, the `out` file will contain this program.

## Further Information

* [Changelog](CHANGELOG)
* [License](LICENSE)
* [Gitlab](https://gitlab.namibsun.net/kit/invasive-rust/octorust)
* [Github](https://github.com/namboy94/octorust)
* [Progstats](https://progstats.namibsun.net/projects/octorust)
