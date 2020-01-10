from collections import deque, namedtuple
from utils import VM
from intcodev1 import Intcode
import sys

with open('data/day15.txt') as f:
    data = list(map(int, f.read().split(',')))

#   1
# 3   4
#   2
# input north (1), south (2), west (3), and east (4).

# Output
# 0: The repair droid hit a wall. Its position has not changed.
# 1: The repair droid has moved one step in the requested direction.
# 2: The repair droid has moved one step in the requested direction; its new position is the location of the oxygen system

class Maze:
    REV = {1: 2,
           2: 1,
           3: 4,
           4: 3}

    D = {1: (0,1),
         2: (0,-1),
         3: (-1,0),
         4: (1,0)}

    def __init__(self, vm):
        self.vm = vm
        self.pos = (0,0)
        self.stack = [0]
        self.board = {self.pos: True}
        self.seen = set([self.pos])
        self.oxygen = None

    def step(self, pos, d):
        return (pos[0] + self.D[d][0], pos[1] + self.D[d][1])

    def move(self, d):
        self.vm.add_input(d)
        self.vm.run()
        new_pos = self.step(self.pos, d)
        return self.vm.outputs.pop(-1), new_pos

    def go_back(self, d):
        # print('back')
        _, self.pos = self.move(self.REV[d])

    def run(self):
        while self.stack:
            current = self.stack.pop()

            if current is None:
                if len(self.stack) == 0:
                    return 
                self.go_back(self.stack[-1])
                continue

            for d in range(current+1, 6):
                # print(d)
                # Dead end (exhausted directions)
                if d == 5:
                    self.stack.append(None)
                    break

                new_pos = self.step(self.pos, d)
                if new_pos in self.seen:
                    continue

                out, _ = self.move(d)
                self.seen.add(new_pos)

                # print('d', d, 'out', out, 'new_pos', new_pos)
                # Wall
                if out == 0:
                    self.board[new_pos] = False

                # Oxygen system
                elif out == 2:
                    self.pos = new_pos
                    self.board[new_pos] = True
                    self.stack.extend([d, 0])
                    self.oxygen = (len(self.stack)-1, new_pos)
                    # keep running to record the whole board
                    break

                # Valid path
                elif out == 1:
                    self.pos = new_pos
                    self.board[new_pos] = True
                    self.stack.extend([d, 0])
                    break

# Part 1
# =================================
# What is the fewest number of movement commands required to move the
# repair droid from its starting position to the location of the oxygen system?

# Can't simply do a BFS. We need to backtrack as the intcode program
# will have modified each time take a step, so we need to step back
vm = VM(data)
maze = Maze(vm)
maze.run()
print('Part 1: ', maze.oxygen[0])

# Part 2
# =================================
# How many minutes will it take to fill with oxygen?

# Here we can simply BFS as we don't need the VM anymore as we
# recorded the board
SEEN = set()
BOARD = maze.board
Q = deque()
Q.append((maze.oxygen[1],0))

while Q:
    P = Q.popleft()
    if P[0] in SEEN:
        continue
    SEEN.add(P[0])

    if BOARD[P[0]]:
        for d in [(0,1),(0,-1),(1,0),(-1,0)]:
            coord = (P[0][0]+d[0], P[0][1]+d[1])
            new_p = (coord, P[1]+1)
            Q.append(new_p)

print('Part 2: ', P[1]-1)
