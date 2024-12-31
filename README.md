# nutext
A library that brings gettext support to Nushell!

## Usage
There are two commands, `tregister` and `tprint`. First you must register your path and program name with `tregister` like so:

```bash
tregister /usr/share/locale/ my-program
```

Then you can start using `tprint` like so:

```bash
tprint "Hello World! I am {name}. I am {years} years old" { name: "Elsie", years: 19 }
```
