# raspberry-pi-sys
Low Level Bindings for Raspberry Pi APIs. 
This binding will be empty on non-linux, non-arm platforms. 

## Goals
This binding has a goal of failing on platforms that are not Raspberry Pis at runtime, even if the Raspberry Pi shared objects are not available. 
This binding will attempt to provide complete coverage, however some APIs cannot be exposed correctly as they are provided as static libaries.

## Binding Generation
Bindings are generated with the `scripts/rebuild-bindings.py` script. 
There is no support for dynamically generating bindings, though it may be added in the future.