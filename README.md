# bmv: Bulk Move
This is a command-line application that acts like a bulk rename tool. It works
by accepting any amount of valid file names, opening them in your `$EDITOR` (or
`vim` if `$EDITOR` is not set, can be configured through the `DEFAULT_EDITOR`
constant in `main.rs`), and renaming the ones you have changed.

## Invocation Examples
`bmv foo.jpg bar.pdf` The editor will contain `foo.jpg` and `bar.pdf`.

`bmv *.png` The editor will contain all `png`-files in the current folder.

`bmv *` The editor will contain all files in the current folder.

`bmv ./*` Same as above.

`bmv ../file*` Opens all entries beginning with `file` in the parent directory.
    The editor will contain entries with the `..` in them, and you can change
    them to any relative or absolute path.

## Missing Functionality
Currently, renaming multiple files to the same name is allowed - effectively
deleting every file but the last one. Should this be an error? A `[y/n]` choice?
Allowed? A flag?
