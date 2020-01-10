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

Room = namedtuple('Room', ['name','description','doors','items'])

def parse_output(out):
    out = out.splitlines()

    detect_doors = False
    doors = []
    detect_items = False
    items = []
    for l in out:
        if not l:
            if detect_doors:
                detect_doors = False
            if detect_items:
                detect_items = False
            continue

        if '==' in l:
            name = l[l.index('== ')+3: l.index(' ==')]

        elif l[0].isalpha():
            if 'Doors' in l:
                detect_doors = True
            elif 'Items' in l:
                detect_items = True
            elif 'Command' in l:
                continue
            else:
                description = l

        elif l[0] == '-':
            if detect_doors:
                doors.append(l[2:])
            if detect_items:
                items.append(l[2:])

    return Room(name, description, doors, items)


pos = (0,0)
SHIP = {}
OPP = {'north': 'south', 'south': 'north', 'east':'west', 'west':'east'}
DY = {'north': 1, 'south': -1, 'east':0, 'west':0}
DX = {'north': 0, 'south': 0, 'east':1, 'west':-1}
SEEN = set()

def command(vm, command):
    vm.add_input(to_ascii(command))
    vm.run()
    out = to_str(vm.outputs)
    vm.outputs = []
    return out

def navigate(came_from, room, pos):
    if len(room.doors) == 1 and came_from == OPP[room.doors[0]]:
        # print('returning')
        return None

    if came_from == '':
        doors = room.doors
    else:
        doors = set(room.doors)-set([OPP[came_from]])
    # print(doors)
    for door in doors:
        # print(room, door)
        new_room = parse_output(command(vm, door))
        if new_room is None or new_room.name in SEEN:
            continue
        SEEN.add(new_room.name)
        new_pos = (pos[0] + DX[door], pos[1] + DY[door])
        SHIP[new_pos] = new_room
        if navigate(door, new_room, new_pos) is None:
            command(vm, OPP[door])
            continue

    return None

# Parse all rooms in the ship
vm = VM(data)
vm.run()
print(vm.outputs)
room = parse_output(to_str(vm.outputs))
SHIP[pos] = room
navigate('', room, pos)
print(SHIP)


vm = VM(data)
vm.run()
commands = ['south', 'south', 'south','south', 'take festive hat',
    'north','north','north', 'take whirled peas', 'north',
    'west', 'take pointer', 'east', 'north', 'take coin',
    'north','take astronaut ice cream','north','west','take dark matter',
    'south', 'take klein bottle','west', 'take mutex','west','south',
    'drop dark matter', 'drop astronaut ice cream',
    'drop klein bottle', 'drop pointer', 'east']


def parse_inventory(out):
    out = out.splitlines()
    items = []
    for l in out:
        if l and l[0] == '-':
            items.append(l[2:])
    return items

items = ['mutex', 'dark matter', 'astronaut ice cream',
    'festive hat', 'whirled peas', 'coin', 'klein bottle', 'pointer']

while True:
    while commands:
        cmd = commands.pop(0)
        out = command(vm, cmd)
    if '16410' in out:
        print(out)
        sys.exit(0)
    # for k in range(1, len(items)+1):
    #     combs = itertools.combinations(items, k)
    #     for c in combs:
    #         print(c)
    #         for item in c:
    #             command(vm, f'take {item}')
    #         out = command(vm, 'east')
    #         if '16410' in out:
    #             print(out)
    #             sys.exit(0)
    #         for item in c:
    #             command(vm, f'drop {item}')

    # print(command(vm, input()))



