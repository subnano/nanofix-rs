nanofix-rs
----------

### Requirement
nanofix is a collection of tools written in rust that work with the [FIX Protocol](https://www.fixtrading.org/). 
The tools range from a FIX message viewer, TBA

While there exists other rust based FIX crates there was a need to have access to lower level constructs to 
build a suite of tools to work with the FIX protocol. 

Command|Description
:---|:---|
**fixv** | A command line FIX message viewer useful for analysing raw messages and log files

## FIX Viewer
### Downloading
To download the program, type the following command:


```curl -L -o fixv.zip https://github.com/subnano/nanofix-rs/releases/download/0.1.1/fixv.zip && unzip fixv.zip -d ~/bin && rm fixv.zip```

### TODO
* add functionality to filter (from and/or to) by MsgSeqNum
* add functionality to filter (from and/or to) by SendingTime
* amy other filters ??
* support config _(~/.fixv)_ to explicit set the different colors
* add support for excluding multi-byte MsgTypes
  
### Links
- [FIX Trading Community](https://www.fixtrading.org/) 
- [Wikipedia](https://en.wikipedia.org/wiki/Financial_Information_eXchange)
