from aocd.models import Puzzle
puzzle = Puzzle(year=2019, day=7)
data = puzzle.input_data
seq = [int(x) for x in data.split(',')]

ADD = 1
MUL = 2
IN = 3
OUT = 4
JUMP_TRUE = 5
JUMP_FALSE = 6
LESS_THAN = 7
EQUALS = 8
HALT = 99

STEPS = {
    ADD: 4,
    MUL: 4,
    IN: 2,
    OUT: 2,
    JUMP_TRUE: 3,
    JUMP_FALSE: 3,
    LESS_THAN: 4,
    EQUALS: 4,
    HALT: 1,
}

# With inspiration from https://github.com/benediktwerner/AdventOfCode/blob/master/2019/day07/sol.py
class VM:
    def __init__(self, program):
        self.program = program.copy()
        self.p = 0
        self.inputs = []
        self.outputs = []
        self.halted = False

    def add_input(self, value):
        self.inputs.append(value)

    def get_mode(self, first):
        opcode = first % 100
        modes = [int(d) for d in str(int(first/100))]
        modes = [0] * (STEPS[opcode]-len(modes)-1) + modes
        return opcode, modes[::-1]

    def get_value(self, pos, immediate):
        if immediate:
            return self.program[pos]
        else:
            return self.program[self.program[pos]]

    def parse(self):
        opcode, modes = self.get_mode(self.program[self.p])
        values = []
        positions = []
        # print('state', self.p, self.program)
        for i in range(1, STEPS[opcode]):
            pos = self.p+i
            values.append(self.get_value(pos, modes[i-1]))
            positions.append(self.program[pos])

        write_to = None
        if opcode in [ADD, MUL, IN, LESS_THAN, EQUALS]:
            write_to = positions[-1]

        return opcode, modes, values, write_to


    def run(self):
        while not self.halted:
            opcode, modes, values, write_to = self.parse()
            # print(self.program, opcode, modes, values, write_to)
            if opcode == HALT:
                self.halted = True
                return
            elif opcode == ADD:
                self.program[write_to] = values[0] + values[1]
                self.p += STEPS[opcode]
            elif opcode == MUL:
                self.program[write_to] = values[0] * values[1]
                self.p += STEPS[opcode]
            elif opcode == IN:
                if len(self.inputs) == 0:
                    return
                self.program[write_to] = self.inputs.pop(0)
                self.p += STEPS[opcode]
            elif opcode == OUT:
                self.outputs.append(values[0])
                self.p += STEPS[opcode]
            elif opcode == JUMP_TRUE:
                if values[0] != 0:
                    self.p = values[1]
                else:
                    self.p += STEPS[opcode]
            elif opcode == JUMP_FALSE:
                if values[0] == 0:
                    self.p = values[1]
                else:
                    self.p += STEPS[opcode]
            elif opcode == LESS_THAN:
                self.program[write_to] = int(values[0] < values[1])
                self.p += STEPS[opcode]
            elif opcode == EQUALS:
                self.program[write_to] = int(values[0] == values[1])
                self.p += STEPS[opcode]
            else:
                BaseException(f'Unrecognized opcode: {opcode}.')

def perms(A):
    if len(A) == 1:
        return [A]

    ans = []
    for i, first in enumerate(A):
        remainder = A[:i] + A[1+i:]
        B = perms(remainder)
        for b in B:
            ans.append([first] + b)
    return ans

def run_amplifiers(program, phases):
    vms = [VM(program.copy()) for _ in range(5)]
    for phase, vm in zip(phases, vms):
        vm.add_input(phase)
    vms[0].add_input(0)

    while not vms[4].halted:
        for i, vm in enumerate(vms):
            vm.run()
            vms[(i+1) % 5].add_input(vm.outputs[-1])

    return vms[4].outputs[-1]

# Part 1
# =================================
print('Part 1: ',
    max(run_amplifiers(seq, phases) for phases in perms([0,1,2,3,4])))

# Part 2
# =================================
print('Part 2: ',
    max(run_amplifiers(seq, phases) for phases in perms([5,6,7,8,9])))
