from utils import VM

# with open('data/day13_test.txt') as f:
with open('data/day13.txt') as f:
    data = f.read().split(',')
    data = list(map(int, data))

# Part 1
# =================================
# How many block tiles are on the screen when the game exits?

vm = VM(data)
vm.run()
output = vm.outputs

T = {}
for st in range(3, len(output), 3):
    T[(output[st-3], output[st-2])] = output[st-1]

count = 0
for k, v in T.items():
    if v == 2:
        count += 1

print('Part 1: ', count)

# Part 2
# =================================
# What is your score after the last block is broken?

EMPTY = 0
WALL = 1
BLOCK = 2
PADDLE = 3
BALL = 4

LEFT = -1
RIGHT = 1
NEUTRAL = 0

def get_ball(output):
    for st in range(len(output)-1, 0, -3):
        if output[st] == BALL:
            return (output[st-2], output[st-1])

def get_paddle(output):
    for st in range(len(output)-1, 0, -3):
        if output[st] == PADDLE:
            return (output[st-2], output[st-1])

def get_score(output):
    for st in range(len(output)-1, 0, -3):
        if output[st-1] == 0 and output[st-2] == -1:
            return output[st]

def move_joystick(vm):
    ball = get_ball(vm.outputs)
    paddle = get_paddle(vm.outputs)
    if ball[0] < paddle[0]:
        joystick = LEFT
    elif ball[0] > paddle[0]:
        joystick = RIGHT
    else:
        joystick = NEUTRAL
    vm.add_input(joystick)

data[0] = 2
vm = VM(data)
while not vm.halted:
    vm.run()
    if len(vm.inputs) == 0:
        move_joystick(vm)

print('Part 2: ', get_score(vm.outputs))
