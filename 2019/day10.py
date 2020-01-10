from collections import defaultdict
import math

# X,Y coordinates where X is the distance from the left edge and Y is the distance from the top edge

with open('data/day10.txt') as f:
    data = f.read().splitlines()

class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y
    def __repr__(self):
        return f'{self.x, self.y}' 

R = len(data)
C = len(data[0])

asteroids = set()
for r in range(R):
    for c in range(C):
        if data[r][c] == '#':
            asteroids.add(Point(r, c))

def is_on(a, b, c):
    "Return true iff point c intersects the line segment from a to b."
    # (or the degenerate case that all 3 points are coincident)
    return (collinear(a, b, c)
            and (within(a.x, c.x, b.x) if a.x != b.x else 
                 within(a.y, c.y, b.y)))

def collinear(a, b, c):
    "Return true iff a, b, and c all lie on the same line."
    return (b.x - a.x) * (c.y - a.y) == (c.x - a.x) * (b.y - a.y)

def within(p, q, r):
    "Return true iff q is between p and r (inclusive)."
    return p <= q <= r or r <= q <= p

# Part 1
# =================================
# Find the best location for a new monitoring station. 
# How many other asteroids can be detected from that location?

# Count only first asteroids on the line of sight
in_sight = defaultdict(list)
for A in asteroids:
    for B in asteroids - set([A]):
        discard_b = False
        for C in asteroids - set([A]) - set([B]):
            if is_on(A, B, C):
                discard_b = True
                break
        if discard_b:
            continue    
        else:
            in_sight[A].append(B)

best_pos, sighted = max(in_sight.items(), key=lambda kv: len(kv[1]))

print('Part 1: ', best_pos, len(sighted))


# Part 2
# =================================
# Which will be the 200th asteroid to be vaporized.
# What do you get if you multiply its X coordinate by 100 
# and then add its Y coordinate?

# Vaporize in clockwise order. Order asteroids by angle.
def get_delta(center, point):
    delta_x = point.y - center.y
    delta_y = point.x - center.x
    return math.atan2(delta_y, delta_x)

def angle_to(p1, p2, rotation=270, clockwise=True):
    angle = math.degrees(get_delta(p1,p2)) - rotation
    return angle % 360

rot = defaultdict(set)
for a in asteroids:
    if a == best_pos:
        continue
    angle = angle_to(best_pos, a)
    rot[angle].add(a)

count = 0 
while count <= 200:
    for angle in sorted(rot.keys()):
        ast = rot[angle]
        count += 1
        if len(ast) == 1:
            rot.pop(angle)
            if count == 200:
                ast = ast.pop()
                print('Part 2: ', count, ast.y*100 + ast.x)
        else:
            for B in ast:
                discard_b = False
                for C in ast - set([B]):
                    if is_on(best_pos, B, C):
                        discard_b = True
                        break
                if discard_b:
                    continue
                else:
                    if count == 200:
                        print('Part 2: ', count, B.y*100 + B.x)
                    rot[angle] = ast - set([B])
                    break
