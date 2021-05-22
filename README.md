# bmv: Bulk Move
This is a command-line application that acts like a bulk rename tool. It works
by accepting any amount of valid file names, opening them in your `$EDITOR` (or
`vim` if `$EDITOR` is not set, can be configured through the `DEFAULT_EDITOR`
constant in `main.rs`), and renaming the ones you have changed.

To see `bmv` in action, please check out the `showcase.mp4` file.

## Building
You need `rustc` and `cargo` installed, the easiest way to get them is by
installing `rustup`.

Once `rustup` is installed, perform the following:

`git clone https://github.com/lupont/bmv.git ~/src/bmv`

`cd ~/src/bmv`

`cargo build --release`

`cargo install --path .`

The last step is optional, it installs it to your `~/.cargo/bin/` directory
which may or may not be in your `$PATH`.

## OS Support
`bmv` is tested on Linux, but should work fine on any BSD and macOS as well.
Windows is untested and unlikely to work, however the changes needed should
be minimal and mostly related to the temp file -- feel free to submit a pull
request if you'd like.

## Invocation Examples
`bmv foo.jpg bar.pdf` The editor will contain `foo.jpg` and `bar.pdf`.

`bmv *.png` The editor will contain all `png`-files in the current folder.

`bmv *` The editor will contain all files in the current folder.

`bmv ./*` Same as above, but the entries will all be prepended by `./`.

`bmv ../file*` Opens all entries beginning with `file` in the parent directory.
    The editor will contain entries with the `..` in them, and you can change
    them to any relative or absolute path.

## Missing Functionality
Currently, renaming multiple files to the same name is allowed - effectively
deleting every file but the last one. Should this be an error? A `[y/n]` choice?
Allowed? A flag?
