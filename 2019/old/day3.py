import pandas as pd
import numpy as np
from dateutil.parser import parse

# data = pd.read_csv('data/day3_test.txt',
#     sep='\t',
#     names=['d1', 'v1','d2', 'v2'])

# def walk_board(directions, values):
#     # board = np.zeros((20000, 20000))
#     # current = (10000,10000)

#     board = np.zeros((500, 500))
#     current = (250,250)

#     def go_right(current, v):
#         new_y = current[1]+v+1
#         board[current[0], current[1]+1:new_y] += 1
#         return (current[0], new_y-1)

#     def go_left(current, v):
#         new_y = current[1]-v
#         board[current[0], new_y:current[1]] += 1
#         return (current[0], new_y)

#     def go_up(current, v):
#         new_x = current[0]+v+1
#         board[current[0]+1:new_x, current[1]] += 1
#         return (new_x-1, current[1])

#     def go_down(current, v):
#         new_x = current[0]-v
#         board[new_x:current[0], current[1]] += 1
#         return (new_x, current[1])

#     for d, v in zip(directions, values):
#         if d == 'L':
#             current = go_left(current, v)
#         elif d == 'R':
#             current = go_right(current, v)
#         elif d == 'U':
#             current = go_up(current, v)
#         elif d == 'D':
#             current = go_down(current, v)

#     return board

# board1 = walk_board(data.d1, data.v1)
# board2 = walk_board(data.d2, data.v2)

# y, x = np.nonzero((board1>0).astype(int) + (board2>0).astype(int)>1)
# print(min(abs(y-10000) + abs(x-10000)))

# # part2



# def walk_board_with_sum(directions, values, target):
#     current = (10000, 10000)

#     def go_right(current, v):
#         new_y = current[1]+v+1
#         return (current[0], new_y-1)

#     def go_left(current, v):
#         new_y = current[1]-v
#         return (current[0], new_y)

#     def go_up(current, v):
#         new_x = current[0]+v+1
#         return (new_x-1, current[1])

#     def go_down(current, v):
#         new_x = current[0]-v
#         return (new_x, current[1])

#     steps = 0
#     for d, v in zip(directions, values):
#         if d == 'L':
#             current = go_left(current, v)
#         elif d == 'R':
#             current = go_right(current, v)
#         elif d == 'U':
#             current = go_up(current, v)
#         elif d == 'D':
#             current = go_down(current, v)

#         steps += v
#         print(current)
#         if current == target:
#             return steps

#     return steps

# for target in zip(y, x):
#     print(target)
#     walk_board_with_sum(data.d1, data.v1, target)
#     break


# After checking reddit
with open('data/day3_or.txt', 'r') as f:
    A, B  = f.read().splitlines()

A = A.split(',')
B = B.split(',')


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
intersections = set(line1.keys()) & set(line2.keys())


print('Part 1: ', min(map(lambda x: abs(x[0]) + abs(x[1]), intersections)))
print('Part 2: ', min(map(lambda x: line1[x] + line2[x], intersections)))