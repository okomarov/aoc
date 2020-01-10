from utils import VM
from collections import deque, Counter, defaultdict
import re

# with open('data/day17_test.txt') as f:
with open('data/day17.txt') as f:
    data = list(map(int, f.read().strip().split(',')))

# 35 means #
# 46 means .
# 10 starts a new line

vm = VM(data)
vm.run()

G = []
row = []
while vm.outputs:
    c = vm.outputs.pop(0)
    if c == 10:
        if len(row) > 0:
            G.append(row)
        row = []
    else:
        row.append(chr(c))

R = len(G)
C = len(G[3])

for r in range(R):
    print(''.join(G[r]))

# Part 1
# =================================
# What is the sum of the alignment parameters for the scaffold intersections?

I = []
robot_pos = None
for r in range(1, R-1):
    for c in range(1, C-1):
        if G[r][c] == '^':
            robot_pos = (r,c)
        if (G[r][c] == '#' and
            G[r-1][c] == '#' and 
            G[r+1][c] == '#' and
            G[r][c-1] == '#' and
            G[r][c+1] == '#'):
            I.append((r,c))

total = 0
for (r,c) in I:
    total += (r*c)

print('Part 1: ', total)
print()

# Part 2
# =================================

# Build the command sequence as L8, R4, etc...
DR = [-1,0,1,0]
DC = [0,1,0,-1]

def get_value(pos, d):
    r,c = pos[0]+DR[d], pos[1]+DC[d]
    if 0 <= r < R and 0 <= c < C:
        return G[r][c]

def build_lr_sequence():
    seq = []
    pos = robot_pos
    d = 0
    count = 0

    while True:
        if get_value(pos, d % 4) == '#':
            count += 1
        elif get_value(pos, (d-1) % 4) == '#':
            d = (d-1) % 4
            if count > 0:
                seq[-1] += str(count)
            seq.append('L')
            count = 1
        elif get_value(pos, (d+1) % 4) == '#':
            d = (d+1) % 4
            if count > 0:
                seq[-1] += str(count)
            seq.append('R')
            count = 1
        else:
            seq[-1] += str(count)
            break
        pos = (pos[0] + DR[d], pos[1] + DC[d])
    return seq

seq = build_lr_sequence()
print(seq)

# Get all possible compression subsequences fitting into 20 chars
patterns = set()
for window in range(5, 0, -1):
    for i in range(window, len(seq)):
        patterns.add(''.join(seq[i-window:i]))

def get_compression_sequences(seqstr):
    occurences = defaultdict(lambda: 0)
    seqlen = len(seqstr)

    for pattern in patterns:
        occurences[pattern] += 1
        w = len(pattern)
        st = seqstr.find(pattern)
        while st + w < seqlen:
            st += w
            st = seqstr.find(pattern, st)
            if st > 0:
                occurences[pattern] += 1
            else:
                break

    # Saved space: (len(subsequence)-1) * 2 * freq_subsequence
    counter = Counter({k: (len(k)-1) * 2 * v for k, v in occurences.items()})
    return sorted(counter.items(), key=lambda kv: kv[1], reverse=True)


seqstr = ''.join(seq)
selected = {}
available = ['A','B','C']

def find_sequences(seqstr, selected, available):
    '''
    Finds optimal compression sequence
    '''
    # print(seqstr, selected)
    if len(available) == 0:
        if any(c in seqstr for c in 'LR'):
            return None
        else:
            return seqstr, selected

    f = available.pop(0)
    for cseq, _ in get_compression_sequences(seqstr):
        selected[f] = cseq
        newstr = seqstr.replace(cseq, f)

        ans = find_sequences(newstr, selected.copy(), available.copy())
        if ans is None:
            continue
        else:
            # Uncomment to see other possible sequences
            # print(ans)
            # continue
            return ans

main_seq, funcs = find_sequences(seqstr, {}, available)
print(main_seq, funcs)

# Convert to ASCII
comma = lambda seq: ','.join(seq) + '\n'
inputs = [comma(main_seq)] + \
         [comma(re.findall(r'L|R|\d{1,2}', f)) for f in funcs.values()] + \
         ['n\n']
inputs = list(map(ord,''.join(inputs)))

data[0] = 2
vm = VM(data)
vm.inputs = inputs
vm.run()
print('Part 2 :', vm.outputs[-1])
