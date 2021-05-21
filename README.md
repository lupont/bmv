# bmv: Bulk Move
This is a command-line application that acts like a bulk rename tool. It works
similarly to the standard `mv` command, but it opens the arguments in your
editor and once you save, the files are renamed.

## Invocation Examples
`bmv foo.jpg bar.pdf` The editor will contain `foo.jpg` and `bar.pdf`.
`bmv *.png` The editor will contain all `png`-files in the current folder.
`bmv *` The editor will contain all files in the current folder.
`bmv ./*` Same as above.
