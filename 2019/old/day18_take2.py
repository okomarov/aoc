from collections import deque
from collections import namedtuple
import sys
# with open('data/day18_test.txt') as f:
# # with open('data/day18.txt') as f:
#     data = f.read().splitlines()

# C = len(data[0])
# R = len(data)

DR = [-1, 0, 1, 0]
DC = [0, 1, 0, -1]
# Pos = namedtuple('Pos', ['r', 'c', 'mykeys', 'd'])
# all_keys = set()

# Q = deque()
# for c in range(C):
#     for r in range(R):
#         val = data[r][c] 
#         if val == '@':
#             p = Pos(r,c, set(), 0)
#             Q.append(p)
#         if 'a' <= val <= 'z':
#             all_keys.add(val)

# SEEN = set()

# while Q:
#     P = Q.popleft()
#     k = (P.r, P.c, tuple(sorted(P.mykeys)))
#     if k in SEEN:
#         continue
#     SEEN.add(k)

#     # Out of range or wall
#     if not (0 <= P.r <= R and 0 <= P.c <= C and data[P.r][P.c] != '#'):
#         continue

#     # Door without key
#     val = data[P.r][P.c]
#     if 'A' <= val <= 'Z' and val.lower() not in P.mykeys:
#         continue

#     new_keys = P.mykeys.copy()
#     if 'a' <= val <= 'z':
#         new_keys.add(val)
#         if new_keys == all_keys:
#             print(P.d)
#             sys.exit(0)

#         # Make sure we don't start from here and come back to it
#         # just because we did not add the key
#         k = (P.r, P.c, tuple(sorted(new_keys)))
#         SEEN.add(k)
        
#     for i in range(4):
#         Q.append(Pos(P.r+DR[i], P.c+DC[i], new_keys, P.d+1))


# Part 2
# ------------------------------------

with open('data/day18.txt') as f:
# with open('data/day18_test.txt') as f:
# with open('data/day18_2.txt') as f:
    data = f.read().splitlines()

C = len(data[0])
R = len(data)

Pos = namedtuple('Pos', ['r', 'c', 'mykeys', 'd', 'id', 'others'])
id = 0
robots = []
Q = deque()

all_keys = set()
for r in range(R):
    for c in range(C):
        val = data[r][c] 
        if val == '@':
            robots.append((id, r, c))
            id += 1
        if 'a' <= val <= 'z':
            all_keys.add(val)

for robot in robots:
    Q.append(Pos(robot[1], robot[2], set(), 0, robot[0], robots))

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
            print('Part 2: ', P.d)
            sys.exit(0)
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
