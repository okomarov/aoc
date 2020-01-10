

with open('data/day3_or.txt', 'r') as f:
    A, B  = f.read().splitlines()

A = A.split(',')
B = B.split(',')

# After checking Reddit for hints implement moves as a
# map of (x, y) coordinates to the step count
def get_line_points(moves):
    line = {}
    current = (0,0)
    total_steps = 0
    for move in moves:
        d = move[0]
        steps = int(move[1:])

        for _ in range(steps):
            if d == 'L':
                current = (current[0], current[1]-1)
            elif d == 'R':
                current = (current[0], current[1]+1)
            elif d == 'U':
                current = (current[0]+1, current[1])
            elif d == 'D':
                current = (current[0]-1, current[1])
            total_steps += 1
            line[current] = total_steps
    return line

line1 = get_line_points(A)
line2 = get_line_points(B)

# Intersection of coordinates
intersections = set(line1.keys()) & set(line2.keys())

# Part 1
# =================================
# What is the Manhattan distance from the central port to the closest intersection?
print('Part 1: ', min(map(lambda x: abs(x[0]) + abs(x[1]), intersections)))

# Part 2
# =================================
# What is the fewest combined steps the wires must take to reach an intersection?
print('Part 2: ', min(map(lambda x: line1[x] + line2[x], intersections)))
