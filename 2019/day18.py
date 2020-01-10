from collections import deque
from collections import namedtuple

Pos = namedtuple('Pos', ['r', 'c', 'mykeys', 'd', 'id', 'others'])

def find_keys_robots(data):
    id = 0
    all_keys = set()
    robots = []
    for r in range(R):
        for c in range(C):
            val = data[r][c] 
            if val == '@':
                robots.append((id, r, c))
                id += 1
            if 'a' <= val <= 'z':
                all_keys.add(val)
    return all_keys, robots

DR = [-1, 0, 1, 0]
DC = [0, 1, 0, -1]

def walk_maze(data, Q, all_keys):
    SEEN = set()
    while Q:
        P = Q.popleft()
        k = (P.r, P.c, tuple(sorted(P.mykeys)), tuple(P.others))
        if k in SEEN:
            # print('SEEN', k)
            continue
        SEEN.add(k)

        # Out of range or wall
        if not (0 <= P.r <= R and 0 <= P.c <= C and data[P.r][P.c] != '#'):
            continue

        # print(P)
        # Door without key
        val = data[P.r][P.c]
        if 'A' <= val <= 'Z' and val.lower() not in P.mykeys:
            continue

        new_keys = P.mykeys.copy()
        others = P.others.copy()
        if 'a' <= val <= 'z':
            new_keys.add(val)
            # print(val)
            if new_keys == all_keys:
                return P.d
            others[P.id] = (P.id, P.r, P.c)

        for robot in others:
            for i in range(4):
                if robot[0] == P.id:
                    Q.append(
                    Pos(P.r+DR[i], 
                        P.c+DC[i],
                        new_keys,
                        P.d+1,
                        P.id, others))
                else:
                    Q.append(
                        Pos(robot[1]+DR[i], 
                            robot[2]+DC[i],
                            new_keys,
                            P.d+1,
                            robot[0], others))

# Part 1
# =================================
# How many steps is the shortest path that collects all of the keys?

# with open('data/day18_test.txt') as f:
with open('data/day18.txt') as f:
    data = f.read().splitlines()

C = len(data[0])
R = len(data)

all_keys, robots = find_keys_robots(data)

Q = deque()
for robot in robots:
    Q.append(Pos(robot[1], robot[2], set(), 0, robot[0], robots))

steps = walk_maze(data, Q, all_keys)
print('Part 1: ', steps)

# Part 2
# =================================
# After updating your map and using the remote-controlled robots,
# what is the fewest steps necessary to collect all of the keys?

with open('data/day18_2.txt') as f:
    data = f.read().splitlines()

C = len(data[0])
R = len(data)

all_keys, robots = find_keys_robots(data)

Q = deque()
for robot in robots:
    Q.append(Pos(robot[1], robot[2], set(), 0, robot[0], robots))

steps = walk_maze(data, Q, all_keys)
print('Part 2: ', steps)
