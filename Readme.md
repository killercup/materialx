# MaterialX Parser in Rust and Support in Bevy

Collection of crates to support [MaterialX](https://materialx.org) in [Bevy](https://bevyengine.org/) (and potentially other Rust projects).

## Current Status

Some basic functions work as proof-of-concept.
Expect continuous changes to every aspect of this project.

## Crates

- [materialx-parser](materialx-parser2/README.md): Parser for MaterialX (`.mtlx`) files
- [bevy-materialx-importer](./bevy-materialx-importer/README.md): Support MaterialX as assets in Bevy
- [materials-downloader](./resources/downloader/README.md): Download free MaterialX files from different sites for testing

## Demo + Testing

Our test approach is very simple:
Given a bunch of MaterialX files, try to parse and render them.

Run the demo using `cargo run` in `demo/`
(change to the folder for bevy to find all demo assets).

How to get MaterialX files for testing?
Aside from the official [MaterialX spec repository][1],
you can find open source MaterialX files on multiple platforms,
e.g. [AMD's GPU Open MaterialX Library](https://matlib.gpuopen.com/)
or [ambientCG](https://ambientcg.com/).
In [`resources/downloader`](resources/downloader/README.md)
you can find a script to download a bunch of test files from there.

[1]: https://github.com/AcademySoftwareFoundation/MaterialX/tree/v1.39.0/resources/Materials/Examples
