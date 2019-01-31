# Nanofix
  Zero-cost low-latency FIX message libraries in Rust
</p>

<p align="center">
  <a href="https://travis-ci.com/subnano/nanofix-rs">
    <img alt="Build Status" src="https://travis-ci.com/subnano/nanofix-rs.svg?branch=master">
  </a>

  <a href="https://crates.io/crates/nanofix-rs">
    <img alt="Crates.io" src="https://img.shields.io/crates/v/nanofix-rs.svg">
  </a>
</p>

<p align="center">
  <a href="https://subnano.github.io/nanofix-rs/0.3.0-alpha.12/futures/">
    Documentation
  </a> | <a href="https://subnano.github.io/nanofix-rs/">
    Website
  </a>
</p>

## Overview
nanofix is a collection of tools written in rust that work with the [FIX Protocol](https://www.fixtrading.org/). 
The tools range from a FIX message viewer, TBA

While there exists other rust based FIX crates there was a need to have access to lower level constructs to 
build a suite of tools to work with the FIX protocol. 


## Project Layout
The root project is intended for `nanofix-rs` library developers only.
Consumers should depend on the sub crates.

Sub crates:

* [`fixv`]: A command line FIX message log parser and viewer  

## License

## TODO
* add functionality to filter (from and/or to) by MsgSeqNum
* add functionality to filter (from and/or to) by SendingTime
* amy other filters ??
* support config _(~/.fixv)_ to explicit set the different colors
* add support for excluding multi-byte MsgTypes
  
## Links
- [FIX Trading Community](https://www.fixtrading.org/) 
- [FIX on Wikipedia](https://en.wikipedia.org/wiki/Financial_Information_eXchange)

### Downloading Programs
For example, to download the FIX Viewer program, type the following command:

```curl -L -o fixv.zip https://github.com/subnano/nanofix-rs/releases/download/0.1.1/fixv.zip && unzip fixv.zip -d ~/bin && rm fixv.zip```

