import numpy as np
import math
import itertools as it

with open('data/day18_test.txt') as f:
# with open('data/day18.txt') as f:
    data = f.read().splitlines()

data = np.char.array([list(l) for l in data])

keys = {
    (p[0], p[1]): key for p, key in 
    zip(np.argwhere(data.islower()), data[data.islower()])}
score_placeholder = 9999999

def get_point(letter):
    p = np.argwhere(data==letter)
    if len(p) == 0:
        return
    return tuple(p[0])

def get_walkable(data):
    walkable = np.argwhere(data != '#')
    return set((p[0], p[1]) for p in walkable)

def get_doors(data):
    doors = np.argwhere(data.isupper())
    return set((p[0], p[1]) for p in doors)

def get_four(xy):
    return set([(xy[0], xy[1]+1),
            (xy[0]+1, xy[1]),
            (xy[0], xy[1]-1),
            (xy[0]-1, xy[1])])

class Board:
    def __init__(self, board):
        self.board = board 
        self.walkable = get_walkable(data)
        self._visited = {p: False for p in self.walkable}
        self.unobstructed = (self.walkable - get_doors(data))

    def goto(self, s, target, unlocked):
        p = get_point(target)
        s = get_point(s)
        self.visited = self._visited.copy()
        ans = self.reach(s, p, unlocked, 0)
        
        if ans is None:
            ans = (score_placeholder, set())
        return ans + (target,)

    def reach(self, s, target, unlocked, count):
        self.visited[s] = True
    
        adjacent = get_four(s) & (self.unobstructed | unlocked)
        for a in adjacent:
            if self.visited[a]:
                continue

            count += 1
            if a == target:
                door = get_point(keys[a].upper())
                if door:
                    unlocked.add(door)

                return count, unlocked

            res = self.reach(a, target, unlocked.copy(), count)
            if res is None:
                count -= 1
                continue
            return res
        return

def sort_scores(scores):
    n = len(scores)
    for i in range(n):
        for j in range(0, n-i-1):
            if scores[j][0] > scores[j+1][0]: 
                scores[j], scores[j+1] = scores[j+1], scores[j]
    return scores


def get_path(root, start, remaining, unlocked):
    if len(remaining) == 0:
        p = ''
        total = 0
        for score in root:
            p += score[2]
            total += score[0]
        print(p, total)
        results.append(total)

    scores = [b.goto(start, target, unlocked) for target in remaining]

    for score in scores:
        if score[0] == score_placeholder:
            continue
        else:
            new_ = root.copy()
            new_.append(score)
            remaining = [s[2] for s in scores if s != score]
            get_path(new_, score[2], remaining, score[1])


b = Board(data)
remaining = set(keys.values())
results = []
# get_path([], '@', remaining, set())
# print(min(results))

# from matplotlib import pyplot as plt
# x = [t[1] for t in b.unobstructed]
# y = [t[0] for t in b.unobstructed]
# plt.scatter(x, y, marker='s')
# plt.show()
print(remaining)