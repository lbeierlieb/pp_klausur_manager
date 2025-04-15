# pp_klausur_manager

A tool to help organize the exams of the Programmierpraktikum.

## How to run

In the checked out repository, you can execute the `pp_klausur_manager` directly with the command:
```
$ nix run -- <room>
```
For a Rust development shell to compile the program yourself, run:
```
$ nix develop
$ cargo build
```
To spin up a virtual testing environment consisting of one control machine and three student computers, execute:
```
$ nix run .#checks.x86_64-linux.virtual-exam-computer-pool.driverInteractive
>>> test_script()
```
