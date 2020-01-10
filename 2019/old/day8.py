from aocd.models import Puzzle
puzzle = Puzzle(year=2019, day=8)
data = puzzle.input_data
data  = [int(d) for d in data]

# from collections import Counter

shape = [6, 25]
n = shape[0]*shape[1]
# runmin = n
# for pos in range(1, int(len(data)/n + 1)):
#     l = Counter(data[(pos-1)*n:pos*n])
#     if l[0] < runmin:
#         selected_layer = l
#         runmin = l[0]

# print(selected_layer[1]*selected_layer[2])


import numpy as np

layers = np.array(data).reshape((25, 6, -1), order='F').swapaxes(0,1)


runmin = n
for i in range(100):
    layer = layers[:,:, i]
    count = np.count_nonzero(layer== 0)
    if count < runmin:
        selected_layer = layer
        runmin = count

print(np.count_nonzero(selected_layer==1) * np.count_nonzero(selected_layer==2))


final_layer = np.zeros(shape)

for row in range(6):
    print('\n')
    for col in range(25):
        for dep in range(100):
            pixel = layers[row, col, dep]
            if pixel == 0 or pixel == 1:
                print(f'{pixel},', end = '')
                break 