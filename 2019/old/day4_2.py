import numpy as np

pwd_range = [236491, 713787]

def start_end(tf):
    """Find start and end indices of running streaks of True values"""
    n = len(tf)

    tf = np.insert(tf, [0, len(tf)], [False, False])    

    # 01 and 10 masks
    start_mask = (tf[:-1] == 0) & (tf[1:] == 1)
    end_mask = (tf[:-1] == 1) & (tf[1:] == 0)

    # Locations
    start_loc = np.where(start_mask)[0]
    end_loc = np.minimum(np.where(end_mask)[0] - 1, n-1)

    return start_loc, end_loc

# Part 1
# =================================
# How many different passwords within the range given in your 
# puzzle input meet these criteria?
pwds = set()
for n in range(236491, 713787):
    digits = np.array([int(d) for d in str(n)])
    diff = np.diff(digits)
    increasing = np.all(diff >= 0)
    has_adjacent_dgts = any(diff == 0)
    if increasing and has_adjacent_dgts:
        for st, en in zip(*start_end(diff == 0)):
            if en >= st:
                pwds.add(n)
print('Part 1: ', len(pwds))

# Part 2
# =================================
# The two adjacent matching digits should not be part of a larger 
# group of matching digits.

pwds = set()
for n in range(236491, 713787):
    digits = np.array([int(d) for d in str(n)])
    diff = np.diff(digits)
    increasing = np.all(diff >= 0)
    has_adjacent_dgts = any(diff == 0)
    if increasing and has_adjacent_dgts:
        for st, en in zip(*start_end(diff == 0)):
            if en == st:
                pwds.add(n)
print('Part 2: ', len(pwds))
