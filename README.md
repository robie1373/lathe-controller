# `Lathe-Controller`
An embedded rust implementation of a control box to adapt treadmill motors using the 
MC-2100-WAM power supply to a variable speed lathe.

## Features
* Variable speed adjusted by rotary encoder
* Controller powered by MC-2100
* Seven segment display to show current RPM
* Speed feedback supplied by Hall effect sensor
* 

## Hardware
ToDo

## Circuit
ToDo

## Dependencies
* ? [rotary encoder](https://crates.io/crates/rotary-encoder-hal) 
* ? [switch](https://crates.io/crates/switch-hal)
* ? [debouncer](https://crates.io/crates/debouncr)
* ? [maybe this 7-segment display](https://crates.io/crates/adafruit-7segment)

## Important data
For debugging, a clue is here: https://www.visualmicro.com/forums/YaBB.pl?num=1674043049


The key trick is getting the PWM signal from the controller to the PS correct. 
The timing of the PWM is 51ms per cycle. By adjusting the duty cycle, you can adjust the speed of the motor.

I normally think about PWM in terms of cycles per second, but in this case it is easier to think in terms of 
high-delay + low-delay = 51ms

Then take input from the user to indicate how the duty cycle should be adjusted. When the rotary encoder is
turned one direction, we increase the portion of the 51ms  which is a high signal and reduce the portion of low 
signal to maintain the 51ms timing. Turning the rotary encoder the other way does the opposite.

Cool? Cool. 

This is actually the second time I've coded this project. Last time I did in in normal arduino using a cloud based IDE / hardware platfrom which will remain nameless. As I can't access that code anymore (angry face) I am taking this "opportunity" to learn a little bit about embedded rust.

The last time I did this I found a great post on the Internet where somebody had figured out the timeing of the PWM signal. I would love to credit them, but I have long ago lost that page and any memory of their name. 

Luckily, the knowledge remains and I found a new source here:
Timing details from [this tread](https://forum.allaboutcircuits.com/threads/treadmill-motor-in-lathe-using-mc-2100-pwm.51061/)
```
rajeshgautam
 Joined Apr 8, 2011  3
Jun 13, 2011

The information about 51 msec is correct. Although arduino does not support that timing in a standard way (as mentioned here) I generated the PWM with desired frequency in the main loop using digitalWrite HIGH/LOW at appropriate times using this:

void loop()
{
digitalWrite(ledPin, HIGH);
delay(30);
digitalWrite(ledPin, LOW);
delay(21);
}

By changing the HIGH/LOW durations and keeping total time at 51, the motor runs as different speed. Thanks for all who took time to post.

Arduino is powered externally from MC2100 wires. A slight doubt is that the MC2100's voltage is around 13 VDC, that is on the higher side of recommended (12 VDC), although its well under safe limits (20 VCD). I hope it does't damage the ardiuno board. An earlier arduino board had stopped responding, god only knows the region.

Next action item is adding buttons to change this duty cycle and speed. I have attached a 16x2 LCD display to arduino, and now plan to process speed pulses from motor and show its speed. The incline mechanism will be similar as mentioned by MC2100's documentation.

Cheers.```



-----------------
# `cortex-m-quickstart`

> A template for building applications for ARM Cortex-M microcontrollers

This project is developed and maintained by the [Cortex-M team][team].

## Dependencies

To build embedded programs using this template you'll need:

- Rust 1.31, 1.30-beta, nightly-2018-09-13 or a newer toolchain. e.g. `rustup
  default beta`

- The `cargo generate` subcommand. [Installation
  instructions](https://github.com/ashleygwilliams/cargo-generate#installation).

- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M
  targets. Run:

``` console
$ rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
```

## Using this template

**NOTE**: This is the very short version that only covers building programs. For
the long version, which additionally covers flashing, running and debugging
programs, check [the embedded Rust book][book].

[book]: https://rust-embedded.github.io/book

0. Before we begin you need to identify some characteristics of the target
  device as these will be used to configure the project:

- The ARM core. e.g. Cortex-M3.

- Does the ARM core include an FPU? Cortex-M4**F** and Cortex-M7**F** cores do.

- How much Flash memory and RAM does the target device has? e.g. 256 KiB of
  Flash and 32 KiB of RAM.

- Where are Flash memory and RAM mapped in the address space? e.g. RAM is
  commonly located at address `0x2000_0000`.

You can find this information in the data sheet or the reference manual of your
device.

In this example we'll be using the STM32F3DISCOVERY. This board contains an
STM32F303VCT6 microcontroller. This microcontroller has:

- A Cortex-M4F core that includes a single precision FPU

- 256 KiB of Flash located at address 0x0800_0000.

- 40 KiB of RAM located at address 0x2000_0000. (There's another RAM region but
  for simplicity we'll ignore it).

1. Instantiate the template.

``` console
$ cargo generate --git https://github.com/rust-embedded/cortex-m-quickstart
 Project Name: lathe-controller
 Creating project called `lathe-controller`...
 Done! New project created /tmp/lathe-controller

$ cd lathe-controller
```

2. Set a default compilation target. There are four options as mentioned at the
   bottom of `.cargo/config`. For the STM32F303VCT6, which has a Cortex-M4F
   core, we'll pick the `thumbv7em-none-eabihf` target.

``` console
$ tail -n9 .cargo/config.toml
```

``` toml
[build]
# Pick ONE of these compilation targets
# target = "thumbv6m-none-eabi"    # Cortex-M0 and Cortex-M0+
# target = "thumbv7m-none-eabi"    # Cortex-M3
# target = "thumbv7em-none-eabi"   # Cortex-M4 and Cortex-M7 (no FPU)
target = "thumbv7em-none-eabihf" # Cortex-M4F and Cortex-M7F (with FPU)
# target = "thumbv8m.base-none-eabi"   # Cortex-M23
# target = "thumbv8m.main-none-eabi"   # Cortex-M33 (no FPU)
# target = "thumbv8m.main-none-eabihf" # Cortex-M33 (with FPU)
```

3. Enter the memory region information into the `memory.x` file.

``` console
$ cat memory.x
/* Linker script for the STM32F303VCT6 */
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 40K
}
```

4. Build the template application or one of the examples.

``` console
$ cargo build
```

## VS Code

This template includes launch configurations for debugging CortexM programs with Visual Studio Code located in the `.vscode/` directory.  
See [.vscode/README.md](./.vscode/README.md) for more information.  
If you're not using VS Code, you can safely delete the directory from the generated project.

# License

This template is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, the [Cortex-M team][team], promises
to intervene to uphold that code of conduct.

[CoC]: https://www.rust-lang.org/policies/code-of-conduct
[team]: https://github.com/rust-embedded/wg#the-cortex-m-team
