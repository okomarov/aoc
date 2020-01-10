from utils import VM

# with open('data/day9_test.txt') as f:
with open('data/day9.txt') as f:
    data = f.read().split(',')
    data = list(map(int, data))

ADD = 1
MUL = 2
IN = 3
OUT = 4
JUMP_TRUE = 5
JUMP_FALSE = 6
LESS_THAN = 7
EQUALS = 8
ADJUSTBASE = 9
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
    ADJUSTBASE: 2,
    HALT: 1,
}

class VM:
    def __init__(self, program):
        self.program = program.copy()
        self.p = 0
        self.inputs = []
        self.outputs = []
        self.halted = False
        self.rel_base = 0

    def add_input(self, value):
        self.inputs.append(value)

    def get_mode(self, first):
        opcode = first % 100
        modes = [int(d) for d in str(int(first/100))]
        modes = [0] * (STEPS[opcode]-len(modes)-1) + modes
        return opcode, modes[::-1]

    def get_value(self, pos, mode):
        if mode == 1:
            return self.read(pos)
        elif mode == 0:
            return self.read(self.program[pos])
        elif mode == 2:
            return self.read(self.program[pos] + self.rel_base)

    def parse(self):
        opcode, modes = self.get_mode(self.read(self.p))
        values = []
        positions = []
        # print('state', self.p, self.program)
        for i in range(1, STEPS[opcode]):
            pos = self.p+i
            values.append(self.get_value(pos, modes[i-1]))
            positions.append(self.read(pos))

        write_to = None
        if opcode in [ADD, MUL, IN, LESS_THAN, EQUALS]:
            write_to = positions[-1]
            if modes[-1] == 2:
                write_to += self.rel_base

        return opcode, modes, values, write_to

    def write(self, dest, value):
        try:
            self.program[dest] = value
        except IndexError as e:
            if dest > 0:
                self.program += [0] * (dest - len(self.program)+1)
                self.program[dest] = value
            else:
                raise e

    def read(self, pos):
        try:
            return self.program[pos]
        except IndexError as e:
            if pos > 0:
                self.program += [0] * (pos - len(self.program)+1)
                return self.program[pos]
            else:
                raise e

    def run(self):
        while not self.halted:
            opcode, modes, values, write_to = self.parse()
            # print(self.program, opcode, modes, values, write_to)
            if opcode == HALT:
                self.halted = True
                return
            elif opcode == ADD:
                self.write(write_to, values[0] + values[1])
                self.p += STEPS[opcode]
            elif opcode == MUL:
                self.write(write_to, values[0] * values[1])
                self.p += STEPS[opcode]
            elif opcode == IN:
                if len(self.inputs) == 0:
                    return
                self.write(write_to, self.inputs.pop(0))
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
                self.write(write_to, int(values[0] < values[1]))
                self.p += STEPS[opcode]
            elif opcode == EQUALS:
                self.write(write_to, int(values[0] == values[1]))
                self.p += STEPS[opcode]
            elif opcode == ADJUSTBASE:
                self.rel_base += values[0]
                # print(self.program[self.p:self.p+2], self.rel_base)
                self.p += STEPS[opcode]
            else:
                BaseException(f'Unrecognized opcode: {opcode}.')


# Part 1
# =================================
vm = VM(data)
vm.add_input(1)
vm.run()
print('Part 1: ', vm.outputs.pop())

# Part 1
# =================================
vm = VM(data)
vm.add_input(2)
vm.run()
print('Part 2: ', vm.outputs.pop())
