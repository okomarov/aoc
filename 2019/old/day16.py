import numpy as np
import math

with open('data/day16_test.txt') as f:
with open('data/day16.txt') as f:
    lines = f.read().strip()
    signal = [int(d) for d in lines]


def get_replicated_base(base, i):
    if i == 0:
        return base
    else:
        return np.tile(base, (i+1,1)).flatten(order='F')
        
# signal = 10000*signal

base = [0, 1, 0, -1]
n = len(signal)
lenb = len(base)

for step in range(100):
    print(step)
    new_signal = []
    for i, d in enumerate(signal):
        match = []
        p = 0
        c = 0 
        while len(match) < n:
            if i == 0 and c == 0:
                p += 1
            to_append = [base[p]] * (i + 1)
            if c == 0 and i > 0:
                match += to_append[1:]
            else:
                match += to_append
            p = (p + 1) % lenb
            c += 1
        match = match[:n]
        out = 0
        # print(signal)
        # print(match)
        for s, m in zip(signal, match):
            out += s*m
        new_signal.append(out)


    # print(new_signal)
    signal = [abs(d) % 10 for d in new_signal]

print(step, signal)

print(signal[:8])
offset = int(''.join([str(d) for d in signal[:8]]))
print(signal[offset:offset+9])


with open('data/day16.txt') as f:
    lines = f.read().strip()
    signal = [int(d) for d in lines]

signal = 10000*signal
offset = int(''.join([str(d) for d in signal[:7]]))
signal = signal[offset:]
for step in range(100):
    new_signal = np.cumsum(signal[::-1]) % 10
    signal = new_signal[::-1]

print(signal[:8])
