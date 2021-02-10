# AdventOfCode2018
My progress through [Advent of Code](https://adventofcode.com/)

I am going to write each day in either:

- Go
- Rust
- Haskell
- Python 3
- Java

# Updates:

## Day 6:
Currently I have done them all in python first then if I come up will a 
solution fast in haskell I do it after in haskell. Yesterdays task could 
have easily been done in haskell but I was still more comfortable in python.

## Day 12:
![Visualization](https://github.com/McRaeAlex/AdventOfCode2018/blob/master/Visualizations/Day12.png)

In order to solve this problem I used a number of tools and techniques. For the
first part I just simulated it by writing the algorithm. For the second part
50000000000 generations is to large to simulate so instead using excel I graphed
the number of plants per generation. I noticed it was linear so I simulated it
up to a certain generation  until it became linear, then calculated the rest
using the function from excel.

## Day 13:
I want to do a visualization for the first part of this maybe the second after.
I need to write to a file each step and then write a python script that converts
the .txt files into pngs and the finally converts them to a gif.

## Day 14:
This one did not feel great. Immediately after reading the problem I thought 
about haskell and how this was perfect for a infinite list but when I got down
to writing it, I did not have the haskell knowledge to do it. I did not know 
how to force lazy evaluate. After almost giving up I found someone elses solution
on Reddit which looked almost identical (except theirs worked) to my solution.
I figured out that using a sequence was necessary to force lazy evaluation and with
some minor tweaks fixed the errors in my code. I was nice to know I was on the 
right track but frustrating being that close but not solving it.


## Day 16:
This day was really enjoyable it got me even more interested in compilers and 
interpreters. I also thought the challenge this day was the perfect level. It was
difficult enough that I had to think of ways to solve it but not to difficult
that even if I knew how to do it I didn't want to code it in my free time
\*cough\* day 15 \*cough\*.