from itertools import combinations
import math

with open('data/day12.txt', 'r') as f:
# with open('data/day12_test.txt', 'r') as f:
    data = f.read().splitlines()
    data = [''.join(c for c in line if c not in set('<>xyz= ')) for line in data]
    data = [[int(c) for c in l.split(',')] for l in data]

all_comb = [comb for comb in combinations(range(4), r=2)]

def step(moons, velocity):
    for comb in all_comb:
        first, second = comb
        for coord in range(3):       
            if moons[first][coord] < moons[second][coord]:
                velocity[first][coord] += 1
                velocity[second][coord] -= 1
            elif moons[first][coord] > moons[second][coord]:
                velocity[first][coord] -= 1
                velocity[second][coord] += 1

    for moon in range(4):
        for c in range(3):
            moons[moon][c] += velocity[moon][c]
    return moons, velocity

# Part 1
# =================================
# What is the total energy in the system after simulating the moons 
# given in your scan for 1000 steps?

velocity = [[0,0,0] for _ in range(4)]
for i in range(1000):
    data, velocity = step(data, velocity)

total = 0
for moon in range(4):
    potential = 0
    kinetic = 0
    for c in range(3):
        potential += abs(data[moon][c])
        kinetic += abs(velocity[moon][c])
    total += (potential * kinetic)

print('Part 1: ', total)
print()

# Part 2
# =================================
# How many steps does it take to reach the first state that exactly \
# matches a previous state?
with open('data/day12.txt', 'r') as f:
    data = f.read().splitlines()
    data = [''.join(c for c in line if c not in set('<>xyz= ')) for line in data]
    data = [[int(c) for c in l.split(',')] for l in data]

allx = set()
ally = set()
allz = set()

seenx = False
seeny = False
seenz = False

velocity = [[0,0,0] for _ in range(4)]

i = 0
while True:
    if not seenx:
        xk = str([[m[0], v[0]] for m,v in zip(data,velocity)])
        if xk in allx:
            seenx = i
        else:
            allx.add(xk)
    if not seeny:
        xy = str([[m[1], v[1]] for m,v in zip(data,velocity)])
        if xy in ally:
            seeny = i
        else:
            ally.add(xy)
    if not seenz:
        xz = str([[m[2], v[2]] for m,v in zip(data,velocity)])
        if xz in allz:
            seenz = i
        else:
            allz.add(xz)

    if seenx and seeny and seenz:
        print('X cycle', seenx, '\nY cycle', seeny, '\nZ cycle', seenz)
        break

    data, velocity = step(data, velocity)
    i+=1

def lcm(x, y):
    return x // math.gcd(x, y) * y

print('Part 2: ', lcm(lcm(seenx, seeny), seenz))
