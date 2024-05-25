# Roll Command Syntax

This document is strictly about how to format dice rolling commands to do what you want. For an explanation of how Sixball keeps track of and manipulates dice under the hood, refer back to [the main readme](/README.md) (or I might split that off into its own file; we'll see when I actually write it).

The quick reference is [here](#quick-reference).

## Basic Structure

Every roll command looks like this:

> ~[command name] [repetitions (optional)] \# [roll string] : [comment (optional)]

Sixball will repeat the original roll string and comment and reply with the results. For example:

> ~roll 1d20+5: Example roll  
> Output:  
> 1d20+5 (Example roll):  
> 14 (1d20 -> [9])  

> ~genroll a2p1d2: Genesys roll example  
> Output:  
> a2p1d2 (Genesys roll example):  
> Blank (use verbose or tray commands for details)  

For obvious reasons, sixball needs the command name and the roll string to be able to do anything. The colon separator and the comment are optional and don't affect the roll itself in any way (but you do need the colon if you want to add a comment). The same goes for the hash separator and repetion number (the latter affects how the roll is **handled** but not the roll itself).

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

Conversions are notated the same way as other dice operators but don't take an argument to their right, as their behavior is generally predefined.

#### Quick Reference

Below is a summary of the currently supported operations and the way they are written:

|  Syntax   |  |  Operation  |  Example  | Note |
| --- | --- | --- | --- | --- |
|  (Any number)   | |  Numerical argument  |  20  |
|  [\*, ...]   | |  Array argument  |  [1, 2, 3]  |
|  d   | |  Die roll  |  1d20  |
|  e   | |  Explode...  |  6d10e10  |
| |  ea  |  Additively  |    |
| |  eo  |  Once  |    |  Default behavior
| |  er  |  Recursively  |    |
|  k   | |  Keep...  |  4d6k3  |
| |  ke  |  Exact  |    |
| |  kh  |  Highest  |    |  Default behavior
| |  kl  |  Lowest  |    |
|  r   | |  Reroll...  |  2d6r1  |
| |  rb  |  Keep better  |    |
| |  ro  |  Once  |    |  Default behavior
| |  rr  |  Recursively  |    |
| |  rb  |  Keep worse  |    |
|  t   | |  Target number  |  6d6t5  |
|  b   | |  Botch number  |  4d10t6b1  |
|  &   | |  Merge  |  2d6 & 3d8  |
|  + - \* / ^   | |  Mathematical operations  |  2 + 2  | Refer to [math section of main readme](/README.md#math)
|  (Various)   | |  Conversions  |    | Refer to [individual documentation](#conversions)

### Repetitions and Comments

There are two optional additions to the roll string that aren't handled by the roller itself: Specifying a number of times the same roll should be repeated, and adding a comment.

Both of these are separated from the actual roll string by separator characters. These can be changed in the config file; by default they are a hash (\#) and colon (:) respectively. The number of repetitions has to come before the roll and the comment after.

If you prepend a number separated by a hash to the roll string, Sixball will process the roll that many times and collect all the rolls into a single output. For example:

> ~roll 6#3d6: Stats in order  
> Output:  
> 3d6 (Stats in order):  
> 1: 12 (3d6 -> [6, 5, 1])  
> 2: 10 (3d6 -> [2, 4, 4])  
> 3: 7 (3d6 -> [3, 1, 3])  
> 4: 8 (3d6 -> [1, 2, 5])  
> 5: 7 (3d6 -> [1, 3, 3])  
> 6: 10 (3d6 -> [1, 5, 4])

Sixball will accept at most 16 simultaneous rolls.

Comments have no effect on the way a roll is processed and are simply added to the roll and output. You can use them for labeling what a roll is for or whatever else you like. There's no inherent limitation to how long a comment can be or what can go in it besides the chat client's character limit and other players' willingness to put up with nonsense. Since Sixball only looks for the leftmost hash and colon separators in the roll input, it should even be safe to use those characters in the comment, but I have not bothered to test this exhaustively, so no guarantees.

### Formatting, Whitespace, and Typos

As a rule, Sixball makes no distinction between uppercase and lowercase and allows arbitrary whitespace within the roll string (including line breaks, tabs, and [anything the Unicode standard considers whitespace](https://en.wikipedia.org/wiki/Template:Whitespace_(Unicode))). Since roll strings can get complicated, this allows you to break them up as you see fit to help you write and/or read them correctly. Specialized functions (but not roll itself) may even ignore arbitrary characters in between meaningful ones, but this is a side effect and the only thing I'm committed to supporting for all commands is whitespace and case insensitivity. 

In general, Sixball is meant to be as forgiving of mistakes as I can reasonably make it. There are three hard rules:

 - There **must** be at least one whitespace between the command name and the roll string. Otherwise, Sixball won't be able to tell them apart from some other command it doesn't recognize.
 - If you want to add a comment to the roll, it **must** be separated from the roll string by a colon (:). This is so that Sixball can tell the comment apart from another portion of the roll string (whitespace before or after the colon doesn't matter). The same is true of repetitions and the hash (\#) separator.
 - If you want to repeat the roll multiple times, the repeat number and hash separator **must** come before the roll; if you're adding a comment, the comment and colon separator **must** come after.

Each roll command has **aliases**, which are commands that Sixball treats as equivalent. These include abbreviations or intuitive alternate names as well as typos I expect to be common. For example, ~roll, ~r, ~rill, ~rol, and ~rll all do the same thing without skipping a beat.

## Operations in Detail

This section is an in-depth explanation of each roll operation and how it works. 

Fundamentally, dice operations work similarly to mathematical ones. You have a left-hand argument, an operator, and a right-hand argument. This expression resolves to some value that can be used as the argument for another operator. When an operation is to be resolved, Sixball looks at the arguments it has and attempts to convert them as necessary. If it can't, it aborts and throws an error.

The distinction between different operations may seem arbitrary because the way I've named them frames them as conceptual categories, but on a technical level, they're really defined by what arguments they take:

 - **Mathematical operations** and **Dice** take two numerical arguments, one on the left and one on the right.
 - **Modifiers** take one dicepool (on the left) and one numerical argument (on the right).
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

### Modifiers

Modifiers are dice operators that act on a pool of dice and modify it according to the numerical arguments given.

It's important to note that modifiers are strictly applied in order and fully resolved before the next operation is applied. This is true of all operations, but it's especially important to keep in mind for modifiers that involve rerolling dice. For example, if I want to roll 5d10, reroll all 1s and explode all 10s, I can either reroll 1s and then explode 10s or the other way around. In the former case, if I rerolled any 1s into 10s, those will also explode, but if any 10s explode into 1s, those won't get a reroll. In the latter case, the opposite is true. If I really want to do it recursively, I would have to do so manually. You could approximate that behavior by chaining several redundant operators together, but there's no way to keep going indefinitely. I do plan to eventually add commands that will let you add operations to past rolls, but that's off in the future.

There are four types of modifiers currently implemented:

 - Explode (Roll extra dice and add them to the pool)
 - Keep (Keep only specific dice from the pool and discard the rest)
 - Reroll (Roll specific dice in the pool again, replacing the old result)
 - Target (Modify the result of the pool by counting dice with specific values as successes or failures rather than adding all results)

#### Explode

**Base notation:** e  
**Sub-operations:** ea, eo, er

The explode modifier takes the dicepool to its left and picks out those dice that show one of the numbers given by the argument on its right. For each of those dice, it will roll an extra die with the same number of sides and add it to the pool.

The right-hand argument gives the number or numbers to be exploded. For example:

> 5d6e6 -> Roll 5d6, explode all dice that came up 6
> 6d10e[9, 10] -> Roll 6d10, explode all dice coming up either 9 or 10

The explosion types currently supported are Additive (ea), Once (eo), and Recursive (er). If you just use e without a specifier, Sixball defaults to explode once (so e is equivalent to eo).

As a reminder, chaining operations together (e.g. 6d10e9e10) and giving an array argument (e.g. 6d10e[9, 10]) are not equivalent. The former will be resolved in order from left to right (so the e10 would see any extra 10s resulting from the e9), while the latter will be resolved in one go.

Because explosions actually increase the size of the dice pool, and recursive explosions in particular are theoretically unbounded, there are extra restrictions on them to prevent abuse. Sixball won't let you do a recursive or additive explosion if the number of arguments you give is greater than half the maximum die size in the target pool:

> ~roll 4d4er[2, 3, 4]  
> Output:  
> ☢ Roll error! ☢ (Okay, let's slow down here... (｡･_･｡)ﾉ ﾁｮｲﾏﾁ｡ That's too explosive for my tastes! (Be nice and don't try to go infinite))

If you really really need to do this for some reason, just do it manually.

##### Additive

**Notation:** ea

Example:

> ~roll 5d10ea10  
> Output:  
> 5d10ea10:  
> 42 (5d10 -> [9, 9, 10, 4, 4], explode additive 10 -> [9, 9, 16, 4, 4])

Explode additive adds the newly rolled die's result to that of the die that exploded instead of adding the die to the pool as a separate die. This can and usually does result in a die showing a result greater than the number of sides it has. This modifier is chiefly (exclusively, that I know of) used in Legend of the Five Rings up to 4th edition.

Additive rerolls are also recursive, so if you get another 10, it will keep exploding and adding the result to the old die. 

Additive rerolls are more computationally complex than they sound intuitively, so this operation can be fairly slow—don't be surprised if Sixball takes a few seconds.

##### Once

**Notation:** eo or e

Example:

> ~roll 5d10e10  
> Output:  
> 5d10e10:  
> 33 (5d10 -> [10, 2, 6, 1, 4], explode once 10 -> [10, 2, 6, 1, 4, 10])

This is the default behavior for the explode modifier with no further specification. For each die in the original pool whose result matches one of the explode arguments, one extra die is rolled and added to the pool. That's it. If that extra die's result happens to match one of the arguments as well, it doesn't keep exploding.

##### Recursive

**Notation:** er

Example:

> ~roll 6d6er6  
> Output:  
> 6d6er6:  
> 28 (6d6 -> [6, 4, 3, 1, 4, 3], explode recursive 6 -> [6, 4, 3, 1, 4, 3, 6, 1])

Like all the other explode modifiers, this checks for dice that match one of its arguments and adds an extra die to the pool for each of those. If any of the extra dice also match one of the arguments, those dice explode again, and so on until the argument stops showing up.

Only either indefinite recursion or no recursion at all (with eo) is supported.

#### Keep

**Base notation:** k  
**Sub-operations:** ke, kh, kl

The keep modifier selects specific dice in a pool and discards the rest, returning a smaller pool which consists only of the kept dice. For example:

> 4d6k3 -> Roll 4d6, keep highest 3 results
> 2d20k1 -> Roll 1d20 twice and keep the higher result

The keep types currently supported are Exact (ke), High (kh), and Low (kl). If you just use k without a specifier, Sixball defaults to keep high (so k is equivalent to kh).

Which dice are kept is determined by the sub-operation and the argument given to it. Note that the arguments don't work the same for all sub-operations, and not all sub-operations accept all arguments.

##### Exact

**Notation:** ke

Example:

> ~roll 5d4ke[2, 3]  
> Output:  
> 5d4ke[2, 3]:  
> 8 (5d4 -> [3, 3, 4, 1, 2], keep exactly [2, 3] -> [3, 3, 2])

Keep exact keeps only those dice whose result exactly matches one of its right-hand arguments, regardless of how many dice that is. 

Keep exact accepts both single numbers and arrays as arguments, and they work just like they do with explode and reroll modifiers: the argument(s) are those numbers a die's result has to match in order to be kept.

##### High

**Notation:** kh

Example:

> ~roll 4d6kh3  
> Output:  
> 4d6kh3:  
> 7 (4d6 -> [2, 2, 1, 3], keep highest 3 -> [2, 2, 3])

Keep high keeps the N dice showing the largest results, where N is the right-hand argument.

Keep high only accepts a single number as its right-hand argument, as an array doesn't make a lot of sense with this operation:

> ~roll 4d6kh[2, 3]  
> Output:  
> ☢ Roll error! ☢ (ぇ━(*´･д･)━!!! I don't know what to do with this! (Failed to find an argument or wrong argument))

Because of the way Sixball stores dice, if there are dice of multiple different sizes in the pool, keep high will preferentially keep larger dice (i.e. dice with more sides) over smaller ones if it has to pick between dice that came up the same number. It will still return a smaller die showing a greater result before a larger die showing a lesser result.

Note that keep high and keep low do not preserve the order of dice in a pool. The dice will be sorted in ascending order of result in the output pool.

If you try to keep more dice than there are in the pool, keep high will just give back the entire pool unmodified. If you keep 0 dice, you get an empty pool.

##### Low

**Notation:** kl

Example:

> ~roll 2d20kl1  
> Output:  
> 2d20kl1:  
> 7 (2d20 -> [20, 7], keep lowest 1 -> [7])

Keep low keeps the N dice showing the smallest results, where N is the right-hand argument.

Keep low only accepts a single number as its right-hand argument, as an array doesn't make a lot of sense with this operation:

> ~roll 4d6kl[2, 3]  
> Output:  
> ☢ Roll error! ☢ (ぇ━(*´･д･)━!!! I don't know what to do with this! (Failed to find an argument or wrong argument))

Because of the way Sixball stores dice, if there are dice of multiple different sizes in the pool, keep low will preferentially keep smaller dice (i.e. dice with fewer sides) over larger ones if it has to pick between dice that came up the same number. It will still return a larger die showing a lesser result before a smaller die showing a greater result.

Note that keep high and keep low do not preserve the order of dice in a pool. The dice will be sorted in ascending order of result in the output pool.

If you try to keep more dice than there are in the pool, keep low will just give back the entire pool unmodified. If you keep 0 dice, you get an empty pool.

#### Reroll

**Base notation:** r  
**Sub-operations:** rb, ro, rr, rw

The reroll modifier takes specified dice in the dicepool to its left and rolls them again, replacing the previous result (or making a decision as to which result should be used).

The right-hand argument gives the number or numbers that should be rerolled. For example:

> 4d6r1 -> roll 4d6, and if any 1s appear, roll those dice again  
> 2d6r[1, 2] -> roll 2d6 and reroll any dice that come up 1 or 2

The reroll types currently supported are Better (rb), Once (ro), Recursive (rr), and Worse (rw). If you just use r without a specifier, Sixball defaults to reroll once (so r is equivalent to ro).

As a reminder, chaining operations together (e.g. 2d6r1r2) and giving an array argument (e.g. 2d6r[1, 2]) are not equivalent. The former will be resolved in order from left to right (so the r2 would see any 2s the r1 might have rerolled into), while the latter will be resolved in one go.

Unlike explosions, rerolls are bounded and safe, so there are no restrictions on arguments (not even for recursive rerolls).

##### Better

**Notation:** rb

Example:

> ~roll 4d6rb1  
> Output:  
> 4d6rb1:  
> 18 (4d6 -> [1, 6, 4, 3], reroll keep better 1 -> [5, 6, 4, 3])

Reroll better rolls a die again and always uses the "better" of the two results. The terminology here assumes that higher is better, so strictly speaking it actually uses the roll with the greater result. If lower is better in your use case, you'll want reroll worse instead.

The lesser result is discarded, so there's no way to check what both rolls were.

##### Once

**Notation:** ro

Example:

> ~roll 2d6ro[1, 2]  
> Output:  
> 2d6ro[1, 2]:  
> 8 (2d6 -> [2, 1], reroll once [1, 2] -> [2, 6])

This is the default behavior for the reroll modifier with no further specification. For each die in the original pool whose result matches one of the reroll arguments, that die is rolled again and its result replaced with the new one, regardless of what the new result is. The original result is discarded, but obviously, you know it was one of the reroll arguments.

##### Recursive

**Notation:** rr

Example:

> ~roll 5d10rr[3, 4]  
> Output:  
> 5d10rr[3, 4]:  
> 38 (5d10 -> [2, 10, 10, 9, 4], reroll recursively [3, 4] -> [2, 10, 10, 9, 7])

Recursive reroll finds dice whose results match one of its arguments and rerolls them, but guarantees that the new result won't match any of the arguments. This imitates the process of rerolling a physical die until it stops coming up a certain number or numbers, but in a single step. The resulting probability distribution can be shown to be identical.

If you try to make Sixball recursively reroll all possible values a die can show, such as with 1d2rr[1, 2], it won't reroll the die at all.

##### Worse

**Notation:** rw

Example:

> ~roll 4d6rw6  
> Output:  
> 4d6rw6:  
> 12 (4d6 -> [1, 5, 4, 6], reroll keep worse 6 -> [1, 5, 4, 2])

Reroll worse rolls a die again and always uses the "worse" of the two results. The terminology here assumes that higher is better, so strictly speaking it actually uses the roll with the lesser result. If lower is better in your use case, you'll want reroll better instead.

The greater result is discarded, so there's no way to check what both rolls were.

#### Target

**Notation:** t, b

The target modifier alters a roll such that its total value will be calculated from the number of dice that meet or exceed a certain target number or number(s). Use it for success-counting dicepool mechanics like World of Darkness or Shadowrun.

> 6d6t5 -> Roll 6d6 and count 5 or higher as successes (Shadowrun 5e)
> 5d10t[1, 1, 1, 2] -> Roll 5d10 and count 7-9 as 1 success and 10 as 2 (Exalted)

There are two kinds of target modifier, Target (t) and Botch (b). They are identical, except that target treats its successes as positive numbers and botch as negative numbers (i.e. botch can subtract successes).

A single-number arguments gives the target number for the roll. Any die which comes up that number or higher is counted as a success for Target. For a Botch, dice which come up that number or lower are counted as negative successes. So you can do this, for example, for a difficulty 6 roll in one of the WoD 20th anniversary edition games:

> ~roll 5d10t6b1  
> Output:  
> 5d10t6b1: 
> 1 (5d10 -> [4, 8, 9, 2, 1], success on 6 or higher -> 2 successes, subtract success on 1 or lower -> 1 successes)

Array arguments also work differently from how they do for most other modifiers. Instead of defining what numbers count as successes, an array can be used to define a "success map" by explicitly giving the number of successes each face on a die is worth, from smallest to lowest. This is mostly for systems where different die results are worth different amounts of successes like Exalted. Write the array going from left to right and lowest die result to highest. For Exalted, that would be:

> 6d10t[0, 0, 0, 0, 0, 0, 1, 1, 1, 2] -> no successes for 1-6, one success for 7-9, two successes for 10

To simplify that notation a little, if you provide fewer values than the dice in the pool have sides, Sixball will help you out by assuming all sides that weren't specified are worth zero successes. For Target, Sixball will assume the numbers you give it are for the highest results on the die starting from the right. For Botch, Sixball will use the lowest results on the die, reading the array from the left. So the following is equivalent to the above:

> 6d10t[1, 1, 1, 2] -> no successes for 1-6 (implicit), one success for 7-9, two successes for 10

### Conversions

Conversions take a dicepool and transform the way it behaves in some predefined fashion. They do not take a second argument and just go after any expression that results in a pool of dice. This category is originally intended for operations that alter a pool's return value (table lookups like dice with symbols instead of numbers on them or hit locations), hence the name, but other behavior could be implemented too, as long as it acts on a dicepool and doesn't need another argument.

I don't expect you to use conversions with the roll command often; that is very much what specialized commands are for. But they're available.

#### Genesys Dice

**Base notation:** g  
**Sub-operations:** ga, gb, gc, gd, gp, gs

Genesys Dice converts regular, everyday dice with numbers on them to the ~~weird special dice with symbols on them~~ narrative dice used in the Genesys system. The converted dicepool is modified such that instead of a number, its result is treated as a collection of symbols, which are calculated by comparing the numbers rolled on the dice with the table provided on page 10 of the Genesys Core Rulebook. For example:

> ~roll 2d8ga  
> Output:  
> 2d8ga:  
> 1 Success, 1 Advantage (2d8 -> [3, 5], Ability: [[Success], [Advantage]])

The symbols will be added together and cancel out as per the rules for that. The breakdown lists each individual die's result in case you need to know.

Each conversion operator needs to go with the correct kind of die (d8 for ability dice, etc) for correct results. Sixball won't stop you from converting arbitrary dice to the wrong kind of narrative dice, you just won't get the right results. Any die faces without a conversion defined in the table are treated as blank results.

To roll multiple narrative dice together, you'll need to split them up into multiple pools (one for each kind) so Sixball can know which is which. Because narrative die results aren't numbers, they can't be added together, so you'll need to use the merge operator instead to combine all the pools into a single one **after** converting them to narrative dice, like so:

> ~roll 2d8ga & 2d12gp & 2d8gd: 2 ability dice, 1 proficiency die, and 2 difficulty dice  
> Output:  
> 2d8ga & 2d12gp & 2d8gd (2 ability dice, 1 proficiency die, and 2 difficulty dice):  
> 2 Advantages, 1 Success, 1 Triumph (2d8 -> [7, 5], Ability: [[Success, Advantage], [Advantage]]; 2d12 -> [9, 12], Proficiency: [[Success, Advantage], [Triumph]]; 2d8 -> [8, 1], Difficulty: [[Failure, Threat], [Blank]])

This way, all the result symbols are added up and canceled out correctly. As you can see, this gets verbose quickly, so in general, you'll want to use [the dedicated genroll command](#genroll-genesys-narrative-dice) instead, which takes simplified input and translates it into this syntax for you. That command will also hide the operations breakdown by default to make the output less wordy.

There is no default behavior for the g operator by itself without a specifier (it will return an error). The syntax for the different dice is:

 - gb: Boost (Blue d6)
 - ga: Ability (Green d8)
 - gp: Proficiency (Yellow d12)
 - gs: Setback (Black d6)
 - gd: Difficulty (Purple d8)
 - gc: Challenge (Red d12)

### Combinations

Combinations are operations that combine two dicepools (or any two operations) in some way not supported by other basic functionality. This is a very broad category in theory, but right now there is just one: The merge operator.

#### Merge

**Notation** &

Merge is like a generalized addition between two pools of dice. Where regular addition (with the + operator) between two dicepools treats both of them as numbers and returns another number, the merge operator preserves both dicepools in their entirety and returns another dicepool consisting of all the dice in both mashed together.

The main application for this is when you need to combine results that can't be added together mathematically, such as [narrative dice](#genesys-dice) (see that section for an example).

While I don't know of a system that would call for this, you can also use merging to get very granular with nested dice operations, for example:

> ~roll (2d6kh1 & 3d8kl2)kh1  
> Output:  
> (2d6kh1 & 3d8kl2)kh1:  
> 6 (2d6 -> [6, 1], keep highest 1 -> [6]; 3d8 -> [8, 6, 1], keep lowest 2 -> [1, 6], keep highest 1 -> [6])

Here, we have rolled 2d6 and kept the higher of the two, then taken the lowest two of 3d8, and finally of those three dice we selected the highest one. This isn't restricted to keeping dice; we can do similar tricks with arbitrary operations.

In contexts that don't call for preserving the full dicepool, the merge operator works equivalently to addition, but less efficiently.

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
