from utils import VM
import itertools
import sys
from collections import namedtuple

with open('data/day25.txt') as f:
    data = list(map(int, f.read().split(',')))

def to_ascii(s):
    return [ord(c) for c in s] + [10]

def to_str(ascii):
    return ''.join(map(chr, ascii))

def command(vm, command):
    vm.add_input(to_ascii(command))
    vm.run()
    out = to_str(vm.outputs)
    vm.outputs = []
    return out

def parse_inventory(out):
    out = out.splitlines()
    items = []
    for l in out:
        if l and l[0] == '-':
            items.append(l[2:])
    return items

vm = VM(data)
vm.run()

# Manually assembled this list of commands
# There were too many edge cases to handle with the items and the map
# is small, so don't need a programmatic approach. 
# Walk around, collect items, then once in the room with pressure sensor
# try all combinations of items
commands = ['south', 'south', 'south','south', 'take festive hat',
    'north','north','north', 'take whirled peas', 'north',
    'west', 'take pointer', 'east', 'north', 'take coin',
    'north','take astronaut ice cream','north','west','take dark matter',
    'south', 'take klein bottle','west', 'take mutex','west','south',
    'drop dark matter', 'drop astronaut ice cream',
    'drop klein bottle', 'drop pointer', 'east']

items = ['mutex', 'dark matter', 'astronaut ice cream',
    'festive hat', 'whirled peas', 'coin', 'klein bottle', 'pointer']

while True:
    while commands:
        cmd = commands.pop(0)
        out = command(vm, cmd)
    if '16410' in out:
        print(out)
        sys.exit(0)
    # Try all combinations of items to pass the sensor
    for k in range(1, len(items)+1):
        combs = itertools.combinations(items, k)
        for c in combs:
            print(c)
            for item in c:
                command(vm, f'take {item}')
            out = command(vm, 'east')
            if '16410' in out:
                print(out)
                sys.exit(0)
            for item in c:
                command(vm, f'drop {item}')
