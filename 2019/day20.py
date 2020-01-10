import sys
from collections import deque
from collections import namedtuple

with open('data/day20.txt') as f:
    data = f.read().splitlines()

DR = [-1,0,1,0]
DC = [0,1,0,-1]

R = len(data)
C = len(data[0])

def in_bounds(r, c):
    return 0 <= r < R and 0 <= c < C

def get_portals():
    portals = {}
    for r in range(R):
        for c in range(C):
            if data[r][c].isalpha():
                if in_bounds(r-1, c) and data[r-1][c] == '.':
                    portals[(r-1, c)] = (data[r][c] + data[r+1][c],)
                elif in_bounds(r, c+1) and data[r][c+1] == '.':
                    portals[(r, c+1)] = (data[r][c-1] + data[r][c],)
                elif in_bounds(r, c-1) and data[r][c-1] == '.':
                    portals[(r, c-1)] = (data[r][c] + data[r][c+1],)
                elif in_bounds(r+1, c) and data[r+1][c] == '.':
                    portals[(r+1, c)] = (data[r-1][c] + data[r][c],)

    # Get enter position to (portal name, exit) for both sides
    for pos, pt in portals.items():
        for other_pos, other_pt in portals.items():
            if pos == other_pos:
                continue
            if pt[0] == other_pt[0]:
                portals[pos] = (pt[0], other_pos)

    return portals

portals = get_portals()

for pos, pt in portals.items():
    if pt[0] == 'AA':
        start = pos

def is_inner(r,c):
    return 32 <= r <= 92 and 32 <= c <= 90

Pos = namedtuple('Pos', ['r', 'c', 'portals', 'd', 'out', 'level'])

def walk_donut_maze(data, Q):
    SEEN =  set()
    while Q:
        P = Q.popleft()
        if P.level is None:
            p = (P.r, P.c, tuple(sorted(P.portals)))
        else:
            p = (P.r, P.c, tuple(sorted(P.portals)), P.level)

        if p in SEEN:
            # print(p)
            continue
        SEEN.add(p)

        for i in range(4):
            r, c = P.r + DR[i], P.c + DC[i]
            if not in_bounds(r, c):
                continue

            # Entering a portal
            if (r, c) in portals and not P.out and not portals[(r,c)][0] == 'AA':
                port = portals[(r,c)]
                if port[0] == 'ZZ':
                    if P.level == 0 or P.level is None:
                        return P.d+1
                    else:
                        continue

                new_portals = P.portals.copy()
                new_portals.add(port[0])

                if P.level is None:
                    new_level = None
                else:
                    new_level = P.level + (is_inner(r,c)*2 - 1)
                    if new_level < 0:
                        continue

                Q.append(Pos(*port[1], new_portals, P.d+2, True, new_level))
            elif data[r][c] == '.':
                Q.append(Pos(r, c, P.portals, P.d+1, False, P.level))

# Part 1
# =================================
Q = deque()
Q.append(Pos(*start, set(), 0, False, None))
steps = walk_donut_maze(data, Q)
print('Part 1: ', steps)

# Part 2
# =================================

Q = deque()
Q.append(Pos(*start, set(), 0, False, 0))
steps = walk_donut_maze(data, Q)
print('Part 2: ', steps)
