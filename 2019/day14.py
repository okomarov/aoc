import math

with open('data/day14.txt') as f:
    lines = f.read().splitlines()


def parse(x):
    n, ingredient = x.split(' ')
    return int(n), ingredient

F = {}
for l in lines:
    inputs, output = l.split(' => ')
    
    out_n, out = parse(output)
    F[out] = (out_n, [parse(x.strip()) for x in inputs.split(', ')])

S = {}
def reaction(element, n):
    if element == 'ORE':
        return n

    ores = 0
    if element not in S:
        S[element] = 0

    # Reuse and store excess elements
    n -= S[element]
    produced = F[element][0]
    k = math.ceil(n/produced)
    S[element] = k * produced - n

    for ning, ingredient in F[element][1]:
        ores += reaction(ingredient, ning * k)
    return ores

# Part 1
# =================================
# What is the minimum amount of ORE required to produce exactly 1 FUEL?
print('Part 1: ', reaction('FUEL', 1))


# Part 2
# =================================
# Given 1 trillion ORE, what is the maximum amount of FUEL you can produce?
hi = int(1e12)
S = {}
lo = hi//reaction('FUEL', 1)
while hi > lo:
    mid = lo+(hi-lo)//2 + 1
    surplus = {}
    cost = reaction('FUEL', mid)
    # print(cost)
    if cost < int(1e12):
        lo = mid
    else:
        hi = mid-1

print('Part 2: ', lo)
