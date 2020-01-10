from utils import VM

# with open('data/day19_test.txt') as f:
with open('data/day19.txt') as f:
    data = list(map(int, f.read().strip().split(',')))

# Part 1
# =================================
# How many points are affected by the tractor beam in the 50x50 area
# closest to the emitter?
count = 0
for r in range(50):
    for c in range(50):
        vm = VM(data)   
        vm.add_input(r)
        vm.add_input(c)
        vm.run()
        ans = vm.outputs.pop(0)
        count += ans
        print(ans if ans else ' ', end='')
    print()
print('Part 1: ', count)

# Part 2
# =================================
# Find the 100x100 square closest to the emitter that fits entirely
# within the tractor beam; within that square, find the point closest
# to the emitter. What value do you get if you take that point's X
# coordinate, multiply it by 10000, then add the point's Y coordinate?

def run(c, r):
    # Re-start everytime...uff
    vm = VM(data)
    vm.add_input(r)
    vm.add_input(c)
    vm.run()
    return vm.outputs.pop(0)

# starting offsets
r = 100
c = 30
while True:
    out = run(r, c)
    if out == 1:
        if run(r, c+99) == 1:
            if run(r+99, c) == 1:
                print('Part 2: ', c*10000+r)
                break
            else:
                # print('Col')
                c += 1
        else:
            # print('Row')
            r += 1
    else:
        # print('outer')
        r += 1
        while run(r, c) == 0:
            c += 1
