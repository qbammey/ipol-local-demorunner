# Locally run IPOL demos

## <span style="color:red"> Warning</span>
This is a work in progress, still lacking many (most) features and a proper documentation. Most limitations described here are only temporary until the relevant features are added. Contributions are welcome.

## Summary
This project can help you locally install and run an IPOL demo

## Requirements
(May change in future release)
This project does not handle git and docker authentication, instead it assumes your git and docker environment is already working.

It has only been tested in a Linux environment, and will not work yet in non-POSIX systems due to some OS-specific commands.

## Build
Compile within the project with `cargo build --release`. The executable will be located in `target/release/ipol_local_demorunner`.
The project can also be compiled directly from github with `cargo build --git https://github.com/qbammey/ipol-local-demorunner`.

## Install a demo
To install a demo, you need a JSON DDL file representing the demo. Your git environment needs to have access to the repository indicated in the DDL.

Usage : `ipol_local_demorunner install path/to/ddl.json demo-name`. 

A docker image will be build with the name `ipol-"$demo-name"`, overwriting any potential docker image with that name.

All repositories will be saved in `$HOME/.ipol`.

## Run a demo
One a demo has been installed, you can run it locally:

`ipol_local_demorunner run demo-name output/folder/ path/to/first/input.png path/to/second/input.png ... --param1name param1value --param2name param2value ...` 

The order of inputs is as defined in the DDL. Inputs will be converted to the expected format, but the channels and precision are not converted yet by this local demo runner.

The names of the parameters is as defined by the `id` field in the DDL.

All outputs will be stored in the specified output folder, as well as the standard output. The demorunner will fail if the specified output folder already exists.

## Which demos can I run?
* Only Docker-based demos are supported.
* Demos with non-image inputs (video, data, ...) are not supported yet
* Demos that require a GPU are untested with this project. They will most certainly not work if your environment is not equipped with a GPU.
* As this is a work in progress, other demos might be unusable for various reasons. Do not hesitate to report error messages.

## Changelog
* v0.1.0 (alpha) : Execution now happens directly within the output folder, parallel instances are possible.
* v0.0.0 (alpha) : initial release
