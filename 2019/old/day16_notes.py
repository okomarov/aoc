import itertools

signal = list(map(int, '12345678'))
base = [0, 1, 0, -1]
n = len(signal)

def get_pattern_monster(i, n):
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
        p = (p + 1) % len(base)
        c += 1
    return match[:n]

def get_pattern_iterable(i, n):
    match = [itertools.repeat(b, i+1) for b in base]
    match = itertools.cycle(itertools.chain(*match))
    next(match)
    return [next(match) for _ in range(n)]

print(get_pattern_monster(2, n))
print(get_pattern_iterable(2, n))