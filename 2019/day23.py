from collections import deque
from collections import defaultdict
import sys
from utils import VM

with open('data/day23.txt') as f:
    data = list(map(int, f.read().split(',')))

vms = [VM(data) for _ in range(50)]
for i, vm in enumerate(vms):
    vm.add_input(i)

SEEN = set()
Q = defaultdict(list)
idle = 0
part1_done = False
while True:
    for i, vm in enumerate(vms):
        vm.run()
        while len(vm.outputs) > 0:
            address = vm.outputs.pop(0)
            x = vm.outputs.pop(0)
            y = vm.outputs.pop(0)
            if address == 255:
                NAT = (x, y)
                if not part1_done:
                    print('Part 1: ', y)
                    part1_done = True
            else:
                Q[address].append((x, y))

    for i, vm in enumerate(vms):
        if i in Q:
            while Q[i]:
                idle = 0
                vm.add_input(Q[i].pop(0))
        else:
            vm.add_input(-1)
    idle += 1
    if idle > 1:
        x, y = NAT
        if y in SEEN:
            print('Part 2: ', y)
            sys.exit(0)
        SEEN.add(y)
        vms[0].add_input(NAT)
