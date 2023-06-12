# ReqLog

A very simple utility that just logs the details of HTTP requests sent to it.

It logs:

* Method
* Path
* Raw path (path + query params)
* Parsed query params
* Headers
* Body

## Usage

Clone the repo:

    git clone https://github.com/caerphoto/reqlog.git
    cd reqlog

Start the server. Mac/Linux:

    RUST_LOG=info cargo run

Windows:

    set RUST_LOG=info
    cargo run

Then just point any HTTP requests at http://localhost:9015
