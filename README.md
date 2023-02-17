# Anakata

A overengineered "simple" markdown viewer.

## Usage

```
Usage: anakata [FILE]

Arguments:
  [FILE]

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Development

Right now, this makes heavy use of the same tooling used by the `create-tauri-app` template for Tauri + Yew.
Refer to their respective docs for a probably more accurate requirement lists, but a probable list is something like:
 - a working rust install
 - [tauri cli](https://crates.io/crates/tauri-cli)
 - [trunk](https://trunkrs.dev/)

Run `cargo tauri dev` to spin up the development environment.
Refer to the Tauri [guides](https://tauri.app/v1/guides/) or CLI [docs](https://tauri.app/v1/api/cli) for more.

## Unsolicited Answers to Unasked Questions:

**Q: What is this for?**

**A:** For some reason, I couldn't find a standalone markdown viewer that didn't have a vestigial editor attached.
I like my terminal text editor very much thank you.

**Q: Why not render in a browser like the billion other markdown viewers?**

**A:** I'd rather not drag my browser into this mess, it hogs enough resources on its own.

**Q: Why not use a terminal markdown editor then?**

**A:** Because browsers and CSS do very weird things sometimes, so I'd rather see the actual output.

**Q: Tauri and Yew seems rather excessive, why use those? Isn't Tauri basically just another browser?**

**A:** It was an excuse to play around with Tauri; I plan to use it for a future project.
Yew is here because I like it and I'd rather not write javascript if I can help it.

Yes Tauri basically uses the same thing as a browser, can't really get away from that probably.
Maybe some day I'll gather memory usage statistics to show the difference, and maybe this wouldn't have been a waste of time.

**Q: Will this support X feature?**

**A:** Probably not.
Check `TODO.md` for the list of items I'm actually planning to work on before I consider this "done".

**Q: Why is this called "Anakata"**

**A: Because "kata" and "ana" are the names of the directions in the fourth dimension.
This doesn't relate at all to the purpose of this project, but it sounds kinda cool I guess.

## Third-Party

Some files in this repo have been sourced from other projects.
Each example has been listed below:

 - Markdown Test File sourced from [mxstbr/markdown-test-file](https://github.com/mxstbr/markdown-test-file) [MIT]
 - Pico CSS sourced from [picocss/pico](https://github.com/picocss/pico) [MIT]