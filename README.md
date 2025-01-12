## About

**BAUx2** is an **esoteric programming language** inspired by FUWAMOCO of Hololive English Advent!

### ✨ Highlights

- 💡 Intuitive syntax with keywords from FUWAMOCO streams!
- 💠 Written in Rust!
- 🤖 Included in a custom IDE! (Check Releases for the latest release of BAUDOL IDE.)

BAUx2 is an interpreted Rust-written programming language with a readable syntax.

---
## Features

### 💡 Syntax
BAUx2 has several basic keywords and functions that can be called.

#### BAU
BAU is BAUx2's system print function. It supports strings and variables.
```BAU "BAU BAU Ruffians!"```
```BAU $variable```
#### WA
WA is the initialisation keyword. It supports three variable types:
- KIRA -> String
- BAULEAN -> Boolean using FLUFFY/FUZZY
- MOE -> Int/Double
```WA KIRA string "Haeh?"```
```WA BAULEAN boolean FLUFFY```
```WA MOE number 80```
```WA MOE numbervar $variable```
#### CO
CO is the re-assignment keyword for initialised variables. The value re-assigned MUST correspond to the variable type.
Both WA and CO support arithmetic expressions, like so:
```CO y "$x * 2"```
#### PE, ROPE, RO
PE, ROPE and RO correspond to if/elif/else statements in typical languages. PE and ROPE support arithmetic expressions and BAULEANs.
```
PE FLUFFY
  BAU "Mogojan"
RO
  BAU "Mango Jam"
```
```
PE "$result == 555"
  BAU "Another Pero sighting!"
ROPE "$result == 455"
  BAU "That's not Pero..."
RO
  BAU "Where's Pero, Ruffians?"
```
#### PONDE, ENDPONDE
PONDE is the keyword for number-based looping.
```
PONDE 4
  BAU $num
  BAU "BAU BAU!"
  CO num "$num + 1"
ENDPONDE
```
ENDPONDE is used in nested code to declare the end of a loop. Otherwise, the rest of the code will be looped as well.
#### FUWA, MOCO
FUWA and MOCO are class keywords but are more of a debug feature and formatting structure.
```
FUWA > className
  BAU "This is a class, Ruffians, but we're not learning anything..."
MOCO
```
With this class structure, code can be segmented into titled parts.
#### CHIHUAHUA
CHIHUAHUA is a debug keyword that can be called at the start of your code. 

When paired with a proper class structure, it outputs the start and end of a class.

Example output:
```
Class: Fuwawer
Ruffians!
555
End class
```
### 💠 Rust-written
While BAUx2 is interpreted and runs line-by-line, it is written in Rust which enables faster runtimes.

### 🤖 BAUDOL IDE
BAUDOL is the official IDE for BAUx2 coding. It currently only has an input field, 'Run' button and output box.
To-Do:
- Tab key
- Undo/Redo button
- File I/O system with autosaving
- Improve GUI, add cooler FWMC aesthetics

---

## Sample Code

Sample code will be periodically updated in the source code. Here's a sample which uses all of BAUx2's current functions excluding CHIHUAHUA.
```
FUWA > Fuwawer
    WA KIRA fuwawa1 "Moco-chan!!"
    WA KIRA fuwawa2 "Kono kyoku wa kawaiku utaou ne tte itta desho!!"
    WA BAULEAN baul FLUFFY
    WA MOE result "5 * 111"
    BAU "Ruffians!"
    BAU $result
MOCO

FUWA > Mogojan
    PE "$result == 555"
        BAU "Hear the howling of my soul!"
        BAU "Ready..."
        WA MOE num 1
        PONDE 4
            BAU $num
            BAU "BAU BAU!"
            CO num "$num + 1"
            CO result "$result - 55"
        ENDPONDE
        BAU "---"
    RO
        BAU "Bau Bau..."

MOCO

BAU $fuwawa1
BAU $fuwawa2
BAU "---"
BAU $result
```
Output:
```
Ruffians!
555
Hear the howling of my soul!
Ready...
1
BAU BAU!
2
BAU BAU!
3
BAU BAU!
4
BAU BAU!
---
Moco-chan!!
Kono kyoku wa kawaiku utaou ne tte itta desho!!
---
335
```

## Installation

### User
Download the latest version of BAUDOL in Releases.
#### Windows
Run the BAUx2 executable. It is common for Windows Defender to block the app from running. 
It should be alright to press "Run Anyway". The IDE will then open along with a shell window. Do not close the shell window whilst running BAUDOL as it is crucial to app processes.

### Development
If you're looking to modify, fork or simply take a peek at BAUx2's source code, download the source code and open the project in your IDE of choice (I'm using RustRover).
You can then browse the code. Note that BAUx2 relies on Druid for the BAUDOL interface.
BAUx2's code is quite simple, so you can easily alter the keywords to create your own themed language if you wish.

---

## Credits

Thanks, of course, to FUWAMOCO of Hololive English Advent for their design influence and inspiration! BAU BAU!!

Resources used for BAUx2 Version 1.0:

- rust docs
- stack overflow


Lastly, thank you for trying out BAUx2, the fluffiest and fuzziest esolang!
---

#### BAUx2 by EasybuttonDev
