# Roll Command Syntax

This document is strictly about how to format dice rolling commands to do what you want. For an explanation of how Sixball keeps track of and manipulates dice under the hood, refer back to [the main readme](/README.md) (or I might split that off into its own file; we'll see when I actually write it).

## Basic Structure

Every roll command looks like this:

> ~[command name] [roll string] : [comment (optional)]

For example:

> ~roll 1d20+5: Example roll
> Output:
> 1d20+5 (Example roll):
> 14 (1d20 -> [9])
