# Roll Command Syntax

This document is strictly about how to format dice rolling commands to do what you want. For an explanation of how Sixball keeps track of and manipulates dice under the hood, refer back to [the main readme](/README.md) (or I might split that off into its own file; we'll see when I actually write it).

The quick reference is [here](#quick-reference).

## Basic Structure

Every roll command looks like this:

> ~[command name] [roll string] : [comment (optional)]

Sixball will repeat the original roll string and comment and reply with the results. For example:

> ~roll 1d20+5: Example roll  
> Output:  
> 1d20+5 (Example roll):  
> 14 (1d20 -> [9])  

> ~genroll a2p1d2: Genesys roll example  
> Output:  
> a2p1d2 (Genesys roll example):  
> Blank (use verbose or tray commands for details)  

For obvious reasons, sixball needs the command name and the roll string to be able to do anything. The colon separator and the comment are optional and don't affect the roll itself in any way (but you do need the colon if you want to add a comment).

### Dice Operations

A roll string consists of **arguments** and **operators** which Sixball parses to decide what to do.

**Arguments** can be either numbers (3.14, 69, 420, etc) or arrays of numbers (written like [1, 2, 3]).

**Operators** are everything else and are typically represented as one or more letters or special characters. Each operator needs one or more arguments to tell it what to operate on. Currently supported operators are:

 - Basic mathematical operations (addition, subtraction, multiplication, division, power, square root) and select common mathematical functions.
 - Dice, the most important one! XdY means roll X Y-sided dice, as is the standard. A resolved die roll also acts as an argument for other operations.
 - Explode, i.e. for specified dice, roll more of the same kind and add them to the total.
 - Keep specified dice from a pool and discard the rest.
 - Reroll specified dice in a pool.
 - Target number and botch. Count the dice showing a specified result and return the result as successes instead of adding up the values of the dice.
 - Merge two arbitrary dicepools together and treat them as a single one henceforth.
 - Conversion operators, which take a dicepool and convert it to some predefined non-numerical result (e.g. Genesys's narrative dice or hit location tables). I don't expect you to use these directly most of the time, so they are only explained in their own section.

Expressions are resolved following standard order of operations: In order of precedence, and left to right for multiple operators of equal precedence (except for exponentiation (a^b^c), which goes right to left). Mathematical operations have their usual precedence rules. All dice operations have precedence over all mathematical operations, and die rolls themselves (i.e. XdY) take precedence over everything else. Parentheses can be used to change the order of operations like you would in any mathematical expression: Everything inside the parens is resolved before anything outside.

Here's a simple order of operations as an example:

> (2d6r[1, 2]+5)*1.5

This would roll two six-sided dice, then reroll any 1s or 2s in the result once, add five, and finally multiply the whole thing by 1.5.

Mathematical operations are notated the way you would write them in a formula. The merge operator is written as an ampersand (&) and acts similarly to these. Indeed, it is basically a fancy form of addition.

Die rolls are always written as XdY. X and Y are usually single numbers, but may be arrays or the result of another dicepool (1d10d10) or the result of an expression in parentheses (like (1+2)d6).

The dice operators need to come after a dicepool on the left and be followed by an argument on the right, like 4d6k3 (roll 4d6, keep highest 3). The result of a dice operation is another, modified dicepool, so you can chain them together as much as you like. All dice operators are written as a letter, and most allow you to further specify their behavior with optional extra letters. For example, "r" by default rerolls dice showing the specified number(s) once and replaces the old result with the new, while "rr" rerolls recursively, i.e. if you get the same result again, you keep rerolling until you get a different one.

Conversions are notated the same way as dice operators but don't take an argument to their right, as their behavior is generally predefined.

#### Quick Reference

Below is a summary of the currently supported operations and the way they are written:

 - Mathematical operations: Refer back to the math section of the main readme. The ones you're likely to use when rolling dice are written how you expect.
 - d: Roll dice.
 - &: Merge.
 - e: Explode dice showing the number or numbers specified by the argument to the right.
    - ea: Explode additive: Instead of adding the new die to the pool, its value is added to that of the die that exploded. This is also recursive (see below).
    - eo: Explode once (i.e. if the new die shows the same number, do not explode it too). This is the default.
    - er: Explode recursively: If the new die shows the same number, it explodes again, and so on.
 - k: Keep dice specified by the argument to the right.
    - ke: Keep exact. Rather than keeping a certain number of dice, keeps all dice that show the number or numbers given as an argument, and no more.
    - kh: Keep high. Keeps the highest N dice, where N is the argument. This is the default.
    - kl: Keep low. Keeps the lowest N dice, where N is the argument.
 - r: Reroll dice specified by the argument to the right.
    - rb: Reroll better. Roll a die again and keep whichever roll is higher.
    - ro: Reroll once. Roll a die again and replace it with the new one regardless of result. This is the default.
    - rr: Reroll recursively. Roll a die again until it doesn't show any of the numbers in the argument.
    - rw: Reroll worse. Roll a die again and keep whichever roll is lower.
 - t or b: Target and Botch. These are variants of the same operation, but represented as separate letters. Given a single number as an argument, target will count dice equal to or above that number and botch will count dice equal to or below that number. See the individual entry for details.
    - t: Target number. Count dice showing the specified number(s) and treat the pool's value as the number of dice beating the target number (successes) instead of the sum of die faces.
    - b: Botch. As target number, but dice showing the specified number(s) are counted as negative successes. Generally used only together with target number.

### Formatting, Whitespace, and Typos

As a rule, Sixball makes no distinction between uppercase and lowercase and allows arbitrary whitespace within the roll string (including line breaks, tabs, and [anything the Unicode standard considers whitespace](https://en.wikipedia.org/wiki/Template:Whitespace_(Unicode))). Since roll strings can get complicated, this allows you to break them up as you see fit to help you write and/or read them correctly. Specialized functions (but not roll itself) may even ignore arbitrary characters in between meaningful ones, but this is a side effect and the only thing I'm committed to supporting for all commands is whitespace and case insensitivity. 

In general, Sixball is meant to be as forgiving of mistakes as I can reasonably make it. There are two hard rules:

 - There **must** be at least one whitespace between the command name and the roll string. Otherwise, Sixball won't be able to tell them apart from some other command it doesn't recognize.
 - If you want to add a comment to the roll, it **must** be separated from the roll string by a colon (:). This is so that Sixball can tell the comment apart from another portion of the roll string (whitespace before or after the colon doesn't matter).

Each roll command has **aliases**, which are commands that Sixball treats as equivalent. These include abbreviations or intuitive alternate names as well as typos I expect to be common. For example, ~roll, ~r, ~rill, ~rol, and ~rll all do the same thing without skipping a beat.

## Operations in Detail

TBA

## Roll Commands

TBA

## On Randomness

Sixball uses [the default RNG provided by Rust's rand crate](https://rust-random.github.io/book/guide-rngs.html), which is a cryptographically secure pseudo-random number generator currently using the ChaCha block cipher. In a nutshell, this means it is **not** truly random, but its output has been rigorously analyzed and proven to be functionally indistinguishable from true randomness. If you feel like you're rolling too many snake eyes, take it up with the researchers.
