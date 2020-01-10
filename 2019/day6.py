# with open('data/day6_test.txt', 'r') as f:
with open('data/day6_or.txt', 'r') as f:
    data  = [line.split(')') for line in f.read().splitlines()]

# Map child to parent
allnodes = {}
for parent, child in data:
    allnodes[child] = parent

def get_parents(node):
    path = set()
    while node in allnodes:
        parent = allnodes[node]
        path.add(parent)
        node = parent
    return path

# Part 1
# =================================
# What is the total number of direct and indirect orbits in your map data?        

count = 0
for l in allnodes:
    count += len(get_parents(l))

print('Part 1: ', count)


# Part 1
# =================================
# What is the minimum number of orbital transfers required to move 
# from the object YOU are orbiting to the object SAN is orbiting?

YOU = get_parents('YOU')
SAN = get_parents('SAN')

print('Part 2: ', len(YOU-SAN) + len(SAN-YOU))
