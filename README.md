## Krmítko
[![Build Status](https://api.travis-ci.org/Pscheidl/krmitko.svg?branch=master)](https://travis-ci.org/Pscheidl/krmitko)

A simple utility to upload excel files via HTTP POST as an octet stream to an endpoint of choice.

### Usage

1. Build with cargo build --release
2. Execute the binary, passing two argument with directory to scan for excel files and a target endpoint

Execution example on Linux platform:

`./krmitko /path/to/directory http://localhost:8080/RivParser-1.0-SNAPSHOT/document/excel`

Execution example on Windows platform:

`krmitko.exe C:\some\directory http://localhost:8080/RivParser-1.0-SNAPSHOT/document/excel`

The binary built with cargo can be found in target/release folder.
