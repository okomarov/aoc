from collections import Counter

with open('data/day8.txt') as f:
    data = list(map(int, f.read().strip()))

C = 25
R = 6

# Part 1
# =================================
# Find the layer that contains the fewest 0 digits. On that layer,
# what is the number of 1 digits multiplied by the number of 2 digits?
L = []
N = len(data)
for i, st in enumerate(range(0, N, C*R)):
    L.append(Counter(data[st:st+C*R]))

zeroes = [l[0] for l in L]
idx = zeroes.index(min(zeroes))
print('Part 1: ', L[idx][1]*L[idx][2])

# Part 2
# =================================
# What message is produced after decoding your image?

out = [[2]*C for _ in range(R)]

for l in range(len(L)):
    for r in range(R):
        for c in range(C):
            val = data.pop(0)
            if out[r][c] == 2:
                if val == 1:
                    out[r][c] = '#'
                elif val == 0:
                    out[r][c] = ' '

print('Part 2:')
for r in range(R):
    print(''.join(out[r]))
