# pioneer_rust_cli

Rust CLI to control a Pioneer Receiver (AVR) via telnet.
(First project in Rust, to get to learn the language.)

Same functionality as the original Python version, [https://github.com/turibe/pioneer_python_cli].

Tested on a Pioneer SC-1222-K Amp.

License: MIT.

Disclaimer: *Use at your own risk.*

## Usage:

1. Find out your AVR's IP address.
2. Update src/main.rs
3. cargo run

## Some commands:

- `up`              [volume up]
- `down`            [volume down]
- `<integer>`       [if positive, increase volume this number of times, capped at 10]
- `-<integer>`      [if negative, decrease volume this number of times, capped at -30]

- `<input_name>`    [switch to given input]

- `mode X`          [choose audio modes; not all modes will be available]
- `mode help`       [help with modes]
- `modes`           [help with modes]
- `inputs`          [show inputs]
- `surr`            [cycle through surround modes]
- `stereo`          [stereo mode]
- `status`          [print status]
- `quit` or `exit`  [quit]

- Can also use control-C to exit.

If you have customized your input names, `learn` gets them from the AVR, after which they are available as commands.
`save` saves a json file that can be loaded in the future at startup.
