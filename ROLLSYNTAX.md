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

This section is an in-depth explanation of each roll operation and how it works. I use "operation" here to mean both everything you can do in general and a specific type of thing you can do with dice in particular because I haven't straightened out my terminology yet and "tokens" seems less intuitive if you don't know the code backend. Suggestions welcome.

Fundamentally, dice operations work similarly to mathematical ones. You have a left-hand argument, an operator, and a right-hand argument. This expression resolves to some value that can be used as the argument for another operator. When an operation is to be resolved, Sixball looks at the arguments it has and attempts to convert them as necessary. If it can't, it aborts and throws an error.

The distinction between different operations may seem arbitrary because the way I've named them frames them as conceptual categories, but on a technical level, they're really defined by what arguments they take:

 - **Mathematical operations** and **Dice** take two numerical arguments, one on the left and one on the right.
 - **Operations** take one dicepool (on the left) and one numerical argument (on the right).
 - **Conversions** take one dicepool (on the left) and no numerical arguments.
 - **Combinations** take two dicepools, one on the left and one on the right.

### Arguments

I am assuming here that mathematical operations are self-explanatory enough to skip. You use them exactly like you would in a calculator. The roll command supports all math the dedicated calc command does. However, while Sixball can theoretically process any number a computer can handle, you can't roll 1.5 dice, so the way the roll command works with numbers deserves some explanation. There is also an extra type of numerical argument, arrays, that rolls can use but calc doesn't support (yet?).

#### Numbers

So as to not bury the lede: **You can roll up to 255 dice at a time with up to 255 sides each.** Any numerical arguments given to roll operations similarly cap out at 255. However, the final output of a roll can be any number and you can do any math you like with the results of a roll. In other words:

A-Ok:
> ~roll sin(1d6\*pi/2)  
> Output:  
> sin(1d6\*pi/2):  
> 1 (1d6 -> [1])

No es bueno:
> ~roll 69d(4/20)  
> Output:  
> ☢ Roll error! ☢ (ぇ━(*´･д･)━!!! I don't know what to do with this! (Failed to find an argument or wrong argument))

More precisely, while the basic calculator treats all numbers the same, the roll command recognizes two kinds of numbers: Those that can be used as arguments for dice-related operations and all other numbers. What are the numbers allowed as arguments? Any positive integer between 0 and 255, inclusive. (Yes, Sixball will let you roll 0d0. It's just a 0 with extra steps.)

Note: Sixball does its best to convert numbers and recognize valid arguments, but because of the way computers work, rarely an expression you might expect to evaluate to an integer isn't recognized as one due to rounding errors. For example:

> ~roll 4.0d4  
> Output:  
> ☢ Roll error! ☢ (ぇ━(*´･д･)━!!! I don't know what to do with this! (Failed to find an argument or wrong argument))

But:
> ~roll (4.0+2.0)d6  
> Output:  
> (4.0+2.0)d6:  
> 22 (6d6 -> [5, 2, 4, 6, 2, 3])

This may also affect output:
> ~roll sin(1d6\*pi/2)  
> Output:  
> sin(1d6\*pi/2):  
> -0.00000000000000024492935982947064 (1d6 -> [4])

You have to try pretty hard to make this happen in general, however, as you might guess from my very contrived examples.

#### Arrays

An array is a collection of numbers like a vector. Arrays are enclosed in brackets ([ and ]) and the numbers inside separated by commas (,). Currently, arrays can contain only integers between 0 and 255, inclusive. Example:

> [1, 2, 3, 5, 7]

The array syntax is a little stricter than the rest, in that it does not support nested math. That is, [1+2, 3] can't be parsed into [3, 3], it will just throw an error. Whitespace is still allowed.

The point of arrays is to be able to pass more than one number at once as an argument to an operator. What exactly that means depends on the specific operation. When used this way, they go where you would otherwise put a single number:

> 5d10r[3, 4]

The above will roll 5d10 and then reroll any 3s and 4s in the pool once. This is **not** equivalent to

> 5d10r3r4

The above would roll 5d10, then reroll any 3s in that pool, then reroll any 4s in **that** pool, including any 4s that may have been the result of a 3 being rerolled.

Note that several operations behave differently depending on whether their argument is a single number or an array, so a single number argument is also not necessarily equivalent to an array argument containing only one number.

#### Dice as Arguments

Dicepools, and all operations that act on a dicepool, can be treated as either a dicepool or a numerical argument, as demanded by context. That is to say, if a dicepool is on the left side of a dice operation, which expects a pool of dice, it will be treated as, well, a dicepool. If a dicepool is used in an addition, it will be treated like a number.

Dicepools always convert to a single number and never to an array, even though they come with an array of numbers, so to speak, built in. This is because, in general, you expect to use the total result of a die roll as a number and Sixball isn't smart enough to judge when you might want to use it as an array instead.

That number is, by default, the sum of all results in the dicepool. This **can** be greater than 255, though if it goes on to be used as an argument for another dice operation it will be capped at 255. It won't be capped if all you do with it is normal math.

### Dice

**Base notation:** d

The dice operator is the basic notation for dicepools. There is only one variant of this operator, so no need to specify additional options. It can take either number or array arguments in any combination, but its behavior is different depending on which combination you're using:

|  Operation   |  Example  |  Behavior  |
--- | --- | ---|
| {number}d{number} | 1d20 | Standard notation, rolls X Y-sided dice. In the example: roll a single d20 |
| {number}d{array} | 1d[8, 10, 12] | Will roll the specified number of dice for each of the die sizes in the array. In the example: 1d8, 1d10, 1d12 as a single pool |
| {array}d{number} | [1, 2, 3]d6 | Sums the values in the array together and rolls that many dice of the specified kind. The example is therefore equivalent to 6d6 |
| {array}d{array} | [3, 3, 2]d[6, 8, 10] | Matches the elements of the first array to those of the second and rolls the specified number and kind of dice each time. In the example: 3d6, 3d8, 2d10 as a single pool |

The result is always treated as a single dicepool. The array-based options can be thought of as a more compact notation for the merge operator, but the latter is more powerful since it allows manipulating the dice pools individually before merging them as well as telling apart, say, different sets of dice of the same size.

Within a dicepool with different sizes of die, the dice are always ordered in ascending order of die size, but for dice with equal number of sides the order they were rolled in is preserved. NB: Because of this, if you use the keep operation with dice of multiple different sizes, larger dice (for keep high) or smaller ones (for keep low) will be prioritized. For example, if I roll 1d[8, 10, 12] and get [7, 7, 5] in order, keeping the highest of these will always give me the d10 that's showing a 7 and never the d8 with the same result. If this default behavior is undesired, you'll need to work out a more precise command using the merge operator, or just roll the dice and leave the keeping to human decision.

### Operations

#### Explode

TBA

#### Keep

TBA

#### Reroll

TBA

#### Target

TBA

### Conversions

#### Genesys Dice

TBA

### Combinations

#### Merge

TBA

## Roll Commands

### Genroll: Genesys Narrative Dice

**Aliases:** gr, genesys, groll

This command is for rolling narrative dice as used by various Genesys games. The command looks like this:

> ~genroll a2 p2 d2 : 2 ability dice, 2 proficiency dice, 2 difficulty dice  
> Output:  
> a2 p2 d2 (2 ability dice, 2 proficiency dice, 2 difficulty dice):  
> 1 Advantage, 1 Success (use verbose or tray commands for details)

The scheme is a letter representing the type of die followed by a number representing the amount of that type you want to roll. As usual, whitespace is optional but allowed.

The letters are as follows:

 - b: Boost (Blue d6)
 - a: Ability (Green d8)
 - p: Proficiency (Yellow d12)
 - s: Setback (Black d6)
 - d: Difficulty (Purple d8)
 - c: Challenge (Red d12)

Sixball by design keeps track of all the rolls and conversions, which combined with the more complex than usual result format is likely to clutter the output, so the breakdown is hidden by default.

Under the hood, this command takes each valid set of letter + number and converts it to

> {number}d{sides}g{letter}

For example,

> a2 => 2d8ga

Then it links all of them together with merge (&) operators, so all told, our original example

> genroll a2 p2 d2

becomes

> roll 2d8ga&2d12gp&2d8gd

To unpack, this will roll 2d8, then do a table lookup as per Genesys Core Rulebook p. 10 to convert the rolled numbers to the corresponding sides on the ability die. Then it does the same for 2d12 proficiency dice and 2d8 difficulty dice. Finally, all those results are merged together with the merge operator (simple addition won't do it since the results aren't numbers).

## On Randomness

Sixball uses [the default RNG provided by Rust's rand crate](https://rust-random.github.io/book/guide-rngs.html), which is a cryptographically secure pseudo-random number generator currently using the ChaCha block cipher. In a nutshell, this means it is **not** truly random, but its output has been rigorously analyzed and proven to be functionally indistinguishable from true randomness. If you feel like you're rolling too many snake eyes, take it up with the researchers.
