import sys

# Hardcoded input
or_seq = [1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,13,1,19,1,6,19,23,2,23,6,27,1,5,27,31,1,10,31,35,2,6,35,39,1,39,13,43,1,43,9,47,2,47,10,51,1,5,51,55,1,55,10,59,2,59,6,63,2,6,63,67,1,5,67,71,2,9,71,75,1,75,6,79,1,6,79,83,2,83,9,87,2,87,13,91,1,10,91,95,1,95,13,99,2,13,99,103,1,103,10,107,2,107,10,111,1,111,9,115,1,115,2,119,1,9,119,0,99,2,0,14,0]

def get_values(seq, st, en):
    subs = seq[st:en]
    opcode = subs[0]
    v1 = seq[subs[1]]
    v2 = seq[subs[2]]
    dest = subs[3]
    return opcode, v1, v2, dest 

def process_seq(seq):
    st = 0
    en = 4
    while True:
        opcode, v1, v2, dest = get_values(seq, st, en)
        if opcode == 99:
            return seq[0]
        elif opcode == 1:
            seq[dest] = v1+v2
        elif opcode == 2:
            seq[dest] = v1*v2

        st, en = st+4, en+4

# Part 1
# =================================
seq = or_seq.copy()
seq[1:3] = [12, 2]
print('Part 1: ', process_seq(seq))

# Part 2
# =================================
for noun in range(0, 100):
    for verb in range(0, 100):
        seq = or_seq.copy()
        seq[1:3] = [noun, verb]
        if process_seq(seq) == 19690720:
            print('Part 2: ', 100 * noun + verb)
            sys.exit(0)
