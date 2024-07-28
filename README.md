# CDA3101 cache simulator

this is a cache simulator for an assignment I have in CDA3101.

## references

This isn't how I would have implemented this simulator. I'm basically porting a simulator written in CPP over to rust.

My simulator is slower, but better!

## how is my sim better?

- ascii art
- rust (written in rust BTW)
- charts generated with code

## dependencies

### plotly

Super easy to understand plot generator (powered by JS 🤮)

heres the only documentation I have read: https://lib.rs/crates/plotly#readme-basic-usage
(like literally that page only)

### cfonts

Ascii art for the terminal (makes the simulator 10-times better).

heres the only documentation I have read: https://github.com/dominikwilkowski/cfonts/tree/released/rust
(again just copy paste what I need)

### functionality

This tool takes the same inputs as the cpp simulator I was given.

This tool returns the number of accesses, the number of cache hits, and the hit ratio.

`the plot generation is sort of left as an exercise for the user (or for future me when this is due)`

### implementation details

This cache simulator doesn't actually care about data. Really just wants to know if we got a hit or not.

##### trace files

file where each line is of the format
s 0x0000AA40 12
l 0x0000AA40 1
s 0x0000AA40 12
s 0x0000AA40 22

things to notice:
 - first char is either s or l
 - followed by space
 - followed by 8-digit hex number
 - followed by space
 - followed by decimal number

`this simulator ONLY uses the hex address BUT still enforces this file format (no clue why I did that)`

### running on your machine

1. download rust
2. clone the repo
3. cd into repo OR open in vscode
3. cargo run (in your terminal)

```no need to install anything else it's all in the rust toolchain```

### crashes

This code should only crash if you input very large numbers.

If it crashes under any other circumstance, make an issue that explains how to re-create the crash and I'll get to fixing it.

## using this code

Just credit me if you use it but feel free to fork or do whatever (I do not care at all)
