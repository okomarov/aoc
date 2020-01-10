import itertools
import sys
from collections import defaultdict, deque
import random
from copy import deepcopy

class Program(object):
    def __init__(self, pid, program_file, input):
        self.P = defaultdict(int)
        for i,x in enumerate(open(program_file).read().split(',')):
            self.P[i] = int(x)
        self.input = input
        self.ip = 0
        self.pid = pid
        self.rel_base = 0
        self.halted = False
    def idx(self, i, I):
        mode = (0 if i>=len(I) else I[i])
        val = self.P[self.ip+1+i]
        if mode == 0:
            pass # no-op
        elif mode == 2:
            val = val+self.rel_base
        else:
            assert False, mode
        return val
    def val(self, i, I):
        mode = (0 if i>=len(I) else I[i])
        val = self.P[self.ip+1+i]
        if mode == 0:
            val = self.P[val]
        elif mode == 2:
            val = self.P[val+self.rel_base]
        return val
    def run_all(self):
        ans = []
        while True:
            val = self.run()
            if val == None:
                return ans
            ans.append(val)

    def run(self):
        """Return next output"""
        while True:
            cmd = str(self.P[self.ip])
            opcode = int(cmd[-2:])
            I = list(reversed([int(x) for x in cmd[:-2]]))
            if opcode == 1:
                i1,i2 = self.val(0,I),self.val(1,I)
                self.P[self.idx(2, I)] = self.val(0, I) + self.val(1, I)
                self.ip += 4
            elif opcode == 2:
                i1,i2 = self.val(0,I),self.val(1,I)
                self.P[self.idx(2, I)] = self.val(0, I) * self.val(1, I)
                self.ip += 4
            elif opcode == 3:
                inp = self.input()
                self.P[self.idx(0, I)] = inp #self.Q[0]
                #self.Q.pop(0)
                self.ip += 2
            elif opcode == 4:
                ans = self.val(0, I)
                self.ip += 2
                return ans
            elif opcode == 5:
                self.ip = self.val(1, I) if self.val(0, I)!=0 else self.ip+3
            elif opcode == 6:
                self.ip = self.val(1, I) if self.val(0, I)==0 else self.ip+3
            elif opcode == 7:
                self.P[self.idx(2, I)] = (1 if self.val(0,I) < self.val(1,I) else 0)
                self.ip += 4
            elif opcode == 8:
                self.P[self.idx(2, I)] = (1 if self.val(0,I) == self.val(1,I) else 0)
                self.ip += 4
            elif opcode == 9:
                self.rel_base += self.val(0, I)
                self.ip += 2
            else:
                assert opcode == 99, opcode
                self.halted = True
                return None


def get_input():
    return None

G = []
row = []
P = Program('0', 'data/day17.txt', get_input)
#P.P[0] = 2
while not P.halted:
    ch = P.run()
    if ch == None:
        break
    if ch == 10:
        G.append(row)
        row = []
    else:
        row.append(chr(ch))
DR = [-1,0,1,0]
DC = [0,1,0,-1]
ans = 0
G = G[:-1]
R = len(G)
C = len(G[0])
for r in range(R):
    for c in range(C):
        print(G[r][c],end='')
    print()

for r in range(len(G)):
    for c in range(len(G[r])):
        if G[r][c]=='#':
            ok = True
            for d in range(4):
                rr,cc = r+DR[d], c+DC[d]
                if 0<=rr<R and 0<=cc<C and G[rr][cc]!='#':
                    ok = False
            if ok:
                #for rr in [r-1,r,r+1]:
                #    for cc in [c-1,c,c+1]:
                #        print(G[rr][cc],end='')
                ans += r*c
                #print(r,c,ans)
print(ans)