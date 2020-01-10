from collections import Counter

pwds = set()
pwds_strict = set()

# Hardcoded input
for n in range(236491, 713787):
    n_str = str(n)
    increasing = all(n_str[i] >= n_str[i-1] for i in range(1, 6))
    if increasing:
        # Streaks cannot be interrupted as digits are increasing
        # so simply count by digit
        c = Counter(n_str)
        if 2 in c.values():
            pwds_strict.add(n)
        if any(d >= 2 for d in c.values()):
            pwds.add(n)

# Part 1
# =================================
print('Part 1: ', len(pwds))

# Part 2
# =================================
print('Part 2: ', len(pwds_strict))
