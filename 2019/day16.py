import itertools

# with open('data/day16_test.txt') as f:
with open('data/day16.txt') as f:
    lines = f.read().strip()
    signal = [int(d) for d in lines]

or_signal = list(signal)

def get_pattern_iterable(i, n):
    match = [itertools.repeat(b, i+1) for b in base]
    match = itertools.cycle(itertools.chain(*match))
    next(match)
    return match

base = [0, 1, 0, -1]
n = len(signal)
lenb = len(base)

# Part 1
# =================================
# After 100 phases of FFT, what are the first eight digits in the
# final output list?
for step in range(100):
    new_signal = []
    for i, d in enumerate(signal):
        match = get_pattern_iterable(i, n)
        out = 0
        # print(signal)
        # print(match)
        for s in signal:
            out += s * next(match)
        new_signal.append(out)

    signal = [abs(d) % 10 for d in new_signal]

print('Part 1: ', ''.join(map(str, signal[:8])))

# Part 2
# =================================
# After repeating your input signal 10000 times and running 100 phases
# of FFT, what is the eight-digit message embedded in the final output list?
signal = 10000 * or_signal
offset = int(''.join([str(d) for d in signal[:7]]))
signal = signal[offset:]

# Zeros until offset and ones until the end, since we are past the 
# middle with the offset, so use partial sums
for step in range(100):
    new_signal = signal[::-1]
    for i in range(1, len(new_signal)):
        new_signal[i] += new_signal[i-1]
    signal = [s % 10 for s in new_signal[::-1]]

print('Part 2: ', ''.join(map(str, signal[:8])))
