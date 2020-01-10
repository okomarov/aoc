with open('data/day24.txt') as f:
# with open('data/day24_test.txt') as f:
    data = f.read().splitlines()

# A bug dies (becoming an empty space) unless there is exactly one bug adjacent to it.
# An empty space becomes infested with a bug if exactly one or two bugs are adjacent to it.

DR = [1,0,-1,0]
DC = [0,1,0,-1]
R = len(data)
C = len(data[0])

def in_bounds(r, c):
    return 0 <= r < R and 0 <= c < C

# Part 1
# =================================
# What is the biodiversity rating for the first layout that appears twice?

def get_score(layer):
    ans = 0    
    for r in range(R):
        for c in range(C):
            idx = (r * C + c)
            if layer[r][c] == '#':
                ans += 2**idx
    return ans


SEEN = set()
SEEN.add(tuple(data))
while True:
    new_layer = []
    for r in range(R):
        row = ''
        for c in range(C):
            b = 0
            for i in range(4):
                rr, cc = r+DR[i], c+DC[i]
                if not in_bounds(rr, cc):
                    continue
                b += data[rr][cc] == '#'
            # print(data[r][c], e, b)
            if data[r][c] == '#' and b != 1:
                row += '.'
            elif data[r][c] == '.' and (b == 1 or b == 2):
                row += '#'
            else:
                row += data[r][c]
        new_layer.append(row)

    score = get_score(new_layer)
    # print(new_layer, score)
    if score in SEEN:
        break
    SEEN.add(score)
    data = new_layer

print('Part 1: ', score)


# Part 2
# =================================
# How many bugs are present after 200 minutes?

with open('data/day24.txt') as f:
# with open('data/day24_test.txt') as f:
    data = f.read().splitlines()
data[2] = '#.?.#'

def count_bugs(layer, row=None, column=None):
    if column is None:
        return sum(layer[row][i] == '#' for i in range(C))

    if row is None:
        return sum(layer[i][column] == '#' for i in range(R))

    return int(layer[row][column] == '#')

data = {0: data}
SIDE = {
    (1,2): ('N', lambda layer: count_bugs(layer, row=0)),
    (2,3): ('E', lambda layer: count_bugs(layer, column=C-1)),
    (3,2): ('S', lambda layer: count_bugs(layer, row=R-1)),
    (2,1): ('W', lambda layer: count_bugs(layer, column=0))
}

def make_levels(data):
    levels = sorted(list(data.keys()))
    minl = min(levels)
    if (count_bugs(data[minl], column=0) > 0 or
        count_bugs(data[minl], column=C-1) > 0 or
        count_bugs(data[minl], row=0) > 0 or
        count_bugs(data[minl], row=R-1) > 0):
        data[minl-1] = ['.....' for _ in range(R)]

    maxl = max(levels)
    if (data[maxl][1][2] == '#' or
        data[maxl][3][2] == '#' or
        data[maxl][2][1] == '#' or
        data[maxl][2][3] == '#'):
        data[maxl+1] = ['.....' for _ in range(R)]

    return data

def print_layers(data):
    for l, layer in data.items():
        print(l)
        for r in range(R):
            print(layer[r])

def count_bugs_four(data, l, r, c):
    b = 0
    # print('rc', r, c)
    for i in range(4):
        rr, cc = r+DR[i], c+DC[i]

        num_bugs = 0
        if rr < 0:
            # print('outer')
            if l-1 in data:
                num_bugs = count_bugs(data[l-1], row=1, column=2)
        elif rr >= R:
            # print('outer')
            if l-1 in data:
                num_bugs = count_bugs(data[l-1], row=3, column=2)
        elif cc < 0:
            # print('outer')
            if l-1 in data:
                num_bugs = count_bugs(data[l-1], row=2, column=1)
        elif cc >= C:
            # print('outer')
            if l-1 in data:
                num_bugs = count_bugs(data[l-1], row=2, column=3)
        elif rr == 2 and cc == 2:
            # print('inner')
            if l+1 in data:
                count_inner_bugs = SIDE[(r, c)][1]
                num_bugs = count_inner_bugs(data[l+1])
        else:
            # print('normal')
            num_bugs = count_bugs(layer, row=rr, column=cc)

        b += num_bugs
        # print(b)
    return b

# print_layers(data)
for iteration in range(200):
    # print(f'ITERATION {iteration}\n')
    data = make_levels(data)
    new_data = data.copy()
    for l, layer in data.items():
        new_layer = []
        for r in range(R):
            row = ''
            for c in range(C):
                if r == 2 and c == 2:
                    row += '?'
                else:
                    bugs = count_bugs_four(data, l, r, c)
                    if layer[r][c] == '#' and bugs != 1:
                        row += '.'
                    elif layer[r][c] == '.' and 1<= bugs <= 2:
                        row += '#'
                    else:
                        row += layer[r][c]
            new_layer.append(row)
        new_data[l] = new_layer
    data = new_data
    # print_layers(data)

def count_all_bugs(data):
    count = 0
    for l in data:
        for r in range(R):
            for c in range(C):
                count += data[l][r][c] == '#'
    return count

print('Part 2: ', count_all_bugs(data))
