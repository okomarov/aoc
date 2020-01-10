from utils import VM

# with open('data/day11_test.txt') as f:
with open('data/day11.txt') as f:
    data = f.read().split(',')
    data = list(map(int, data))

# 0 - black
# 1 - white

# 0 left 90 degrees
# 1 right 90 degrees.

# After the robot turns, move forward 1.
# The robot starts facing up.

DR = [1, 0, -1, 0]
DC = [0, 1, 0, -1]

def run_painting_bot(initial_input):
    vm = VM(data)
    current_pos = (0, 0)
    BOARD = {current_pos: initial_input}
    direction = 0
    while not vm.halted:
        if current_pos in BOARD:
            vm.add_input(BOARD[current_pos])
        else:
            vm.add_input(0)

        vm.run()
        color = vm.outputs.pop(0)
        BOARD[current_pos] = color

        rotate_right = vm.outputs.pop(0)
        if rotate_right:
            direction = (direction + 1) % len(DR)
        else:
            direction = (direction - 1) % len(DR)
        current_pos = (current_pos[0]+DR[direction], current_pos[1]+DC[direction])

    return BOARD

# Part 1
# =================================
# How many panels does it paint at least once?
print('Part 1: ', len(run_painting_bot(0)))

# Part 2
# =================================
# After starting the robot on a single white panel instead,
# what registration identifier does it paint on your hull?

import matplotlib.pyplot as plt
board = run_painting_bot(1)
x = []
y = []
for k, v in board.items():
    if v == 1:
        x.append(k[1])
        y.append(k[0])
plt.scatter(x, y)
plt.axes().set_aspect('equal')
plt.show()
