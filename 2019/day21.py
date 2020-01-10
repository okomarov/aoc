import itertools
from utils import VM

with open('data/day21.txt') as f:
    data = list(map(int, f.read().split(',')))

# will move forward automatically
# Springscript programs only use Boolean values
# T, the temporary value register, 
# J, the jump register. 
# Both of these registers start with the value false.

# False if hole:
# one tile away (A),
# two tiles away (B),
# three tiles away (C),
# four tiles away (D)

# 3 instructions
# AND X Y, sets Y to true if both X and Y are true; otherwise, it sets Y to false.
# OR X Y, sets Y to true if at least one of X or Y is true; otherwise, it sets Y to false.
# NOT X Y, sets Y to true if X is false; otherwise, it sets Y to false.

# When you have finished entering your program (at most 15 springscript),
# provide the command WALK followed by a newline

def string_to_ascii(s):
    return list(map(ord, s))

def NOT(x, y):
    s = f'NOT {x} {y}\n'
    return string_to_ascii(s)

def AND(x, y):
    s = f'AND {x} {y}\n'
    return string_to_ascii(s)

def OR(x, y):
    s = f'OR {x} {y}\n'
    return string_to_ascii(s)

def WALK():
    return string_to_ascii('WALK\n')

def RUN():
    return string_to_ascii('RUN\n')

# Part 1
# =================================
#  What amount of hull damage does it report?

# if not A and D:
#     jump
# elif not B and D:
#     jump
# elif not C and D:
#     jump

instructions = [
    NOT('A','T'),
    AND('D', 'T'),
    OR('T', 'J'),

    NOT('B','T'),
    AND('D', 'T'),
    OR('T', 'J'),

    NOT('C','T'),
    AND('D', 'T'),
    OR('T', 'J'),

    WALK()]

vm = VM(data)
vm.add_input(list(itertools.chain(*instructions)))
vm.run()
if vm.outputs[-1] > 0x110000:
    print('Part 1: ', vm.outputs[-1])
else:
    print(''.join(list(map(chr, vm.outputs))))

# Part 1
# =================================
# Successfully survey the rest of the hull by ending your program with RUN.
# What amount of hull damage does it report?

instructions = [
    NOT('A','T'),
    AND('D', 'T'),
    OR('T', 'J'),

    NOT('B','T'),
    AND('D', 'T'),
    AND('H', 'T'),
    OR('T', 'J'),

    NOT('C','T'),
    AND('D', 'T'),
    AND('H', 'T'),
    OR('T', 'J'),

    RUN()]

vm = VM(data)
vm.add_input(list(itertools.chain(*instructions)))
vm.run()
if vm.outputs[-1] > 0x110000:
    print('Part 2: ', vm.outputs[-1])
else:
    print(''.join(list(map(chr, vm.outputs))))
