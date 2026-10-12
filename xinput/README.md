# CrossOver XInput DLL

This directory builds the 64-bit Windows `xinput1_3.dll` used by the
CrossOver joystick backend. The C source and export table are included so
the DLL can be rebuilt without a precompiled binary.

From the repository root:

```sh
nix develop -c sh xinput/build.sh
```

The output is `target/crossover-joystick/xinput1_3.dll`. With a MinGW-w64
cross compiler already on `PATH`, run `sh xinput/build.sh` directly.
`MINGW_CC` can specify an alternative compiler executable or path.

With the game closed, copy the DLL beside `SeaOfStars.exe` in the Sea of
Stars installation directory (back up any existing DLL first). In the
CrossOver bottle's Wine Configuration, add a library override for
`xinput1_3` set to **Native then Builtin**. Restart the game after installation.
The build script only creates the DLL; it does not modify the game or bottle.

The shim exposes controller slot 0; other slots are disconnected. It reads
`tas-xinput-state.bin` beside the DLL, which the Rust joystick backend writes
atomically beside the attached game executable. The file contains exactly
16 bytes: `TAS1`, a little-endian 16-bit button mask, one byte per trigger,
and four little-endian signed 16-bit stick axes (left X/Y, right X/Y).
Missing, unreadable, or invalid input produces a neutral controller state.
The packet number advances when controller state changes. Rumble is ignored.

Exports include the XInput 1.3 functions by name and ordinal, plus
`XInputGetStateEx` at ordinal 100. This is a virtual controller replacement;
it does not forward input from physical controllers.
