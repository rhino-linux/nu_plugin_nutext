# nutext
A library that brings gettext support to Nushell!

## Usage
There are three commands, `tregister`, `_`, and `tprint`. First you must register your path and program name with `tregister` like so:

```bash
tregister /usr/share/locale/ my-program
```

Then you can start using `tprint` like so:

```bash
tprint "Hello World! I am {name}. I am {years} years old" { name: "Elsie", years: 19 }
```

Or if you need to get the translated string alone:

```bash
let my_var = (_ "Hello {world}" { world: "World" })
```

I also have provided the tool `xnutext`, not too disimilar to `xgettext`, which will extract all translatable strings from your nu programs. There are two catches to using this:

1. Nutext must be installed and functional.
2. Your program must be syntactically correct according to Nu: `xnutext` will not extract text if the program cannot be ran.

After you have verified these, you should run:

```bash
xnutext list of files -o po/program.pot
```
