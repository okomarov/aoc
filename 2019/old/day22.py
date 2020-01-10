import time

# with open('data/day22_test.txt') as f:
with open('data/day22.txt') as f:
    data = f.read().splitlines()

# print(data)

N = 10007
# N = 10
cards = list(range(N))

def shuffle(cards):
    N = len(cards)
    for l in data:
        if 'deal into new stack' in l:
            cards = cards[::-1]
        elif 'cut' in l:
            _, n = l.split(' ')
            n = int(n)
            cards = cards[n:] + cards[:n] 
        elif 'deal with increment' in l:
            _, n = l.split('increment ')
            n = int(n)
            new_cards = cards.copy()
            for i in range(N):
                new_cards[(i*n) % N] = cards[i]
            cards = new_cards
    return cards

print('Part 1: ', shuffle(cards).index(2019))

def shuffle2(N, pos):
    for l in data:
        if 'deal into new stack' in l:
            pos = N-pos-1
        elif 'cut' in l:
            n = int(l.split()[1])
            if pos >= n:
                pos -= n
            else:
                pos += (N-n)
        elif 'deal with increment' in l:
            n = int(l.split('increment ')[1])
            pos = (pos*n) % N
    return pos

N = 10007
pos = 2019
# print(shuffle2(N, pos))

# commands = []
# for l in data:
#     if 'deal into new stack' in l:
#         commands.append('pos = N-pos-1')
#     elif 'cut' in l:
#         n = int(l.split()[1])
#         if n < 0:
#             commands.append(f'pos += {-n}')
#         else:
#             commands.append(f'pos = pos-{n} if pos >={n} else pos + N-{n}')
#     elif 'deal with increment' in l:
#         n = int(l.split('increment ')[1])
#         commands.append(f'pos = (pos*{n}) % N')

# # [print(c) for c in commands]

def shuffle3(N, pos):
    pos = (pos*31) % N
    pos = N-pos-1
    pos += 7558
    pos = (pos*49) % N
    pos = pos-194 if pos >=194 else pos + N-194
    pos = (pos*23) % N
    pos += 4891
    pos = (pos*53) % N
    pos = pos-5938 if pos >=5938 else pos + N-5938
    pos = (pos*61) % N
    pos = pos-7454 if pos >=7454 else pos + N-7454
    pos = N-pos-1
    pos = (pos*31) % N
    pos = pos-3138 if pos >=3138 else pos + N-3138
    pos = (pos*53) % N
    pos = pos-3553 if pos >=3553 else pos + N-3553
    pos = (pos*61) % N
    pos += 5824
    pos = (pos*42) % N
    pos += 889
    pos = (pos*34) % N
    pos = pos-7128 if pos >=7128 else pos + N-7128
    pos = (pos*42) % N
    pos += 9003
    pos = (pos*75) % N
    pos = pos-13 if pos >=13 else pos + N-13
    pos = (pos*75) % N
    pos += 3065
    pos = (pos*74) % N
    pos += 8156
    pos = (pos*39) % N
    pos = pos-4242 if pos >=4242 else pos + N-4242
    pos = (pos*24) % N
    pos += 405
    pos = (pos*27) % N
    pos = pos-6273 if pos >=6273 else pos + N-6273
    pos = (pos*19) % N
    pos += 9826
    pos = (pos*58) % N
    pos = N-pos-1
    pos += 6927
    pos = (pos*65) % N
    pos += 9906
    pos = (pos*31) % N
    pos = N-pos-1
    pos = (pos*42) % N
    pos = N-pos-1
    pos = (pos*39) % N
    pos += 4271
    pos = N-pos-1
    pos = (pos*32) % N
    pos += 8799
    pos = (pos*69) % N
    pos = pos-2277 if pos >=2277 else pos + N-2277
    pos = (pos*55) % N
    pos = pos-2871 if pos >=2871 else pos + N-2871
    pos = (pos*54) % N
    pos += 2118
    pos = (pos*15) % N
    pos = pos-1529 if pos >=1529 else pos + N-1529
    pos = (pos*57) % N
    pos += 4745
    pos = (pos*23) % N
    pos += 5959
    pos = (pos*58) % N
    pos = N-pos-1
    pos = (pos*48) % N
    pos = N-pos-1
    pos = pos-2501 if pos >=2501 else pos + N-2501
    pos = N-pos-1
    pos = (pos*42) % N
    pos = N-pos-1
    pos = pos-831 if pos >=831 else pos + N-831
    pos = (pos*74) % N
    pos += 3119
    pos = (pos*33) % N
    pos = pos-967 if pos >=967 else pos + N-967
    pos = (pos*69) % N
    pos = pos-9191 if pos >=9191 else pos + N-9191
    pos = (pos*9) % N
    pos = pos-5489 if pos >=5489 else pos + N-5489
    pos = (pos*62) % N
    pos += 9107
    pos = (pos*14) % N
    pos += 7717
    pos = (pos*56) % N
    pos = pos-7900 if pos >=7900 else pos + N-7900
    pos = (pos*49) % N
    pos = pos-631 if pos >=631 else pos + N-631
    pos = (pos*14) % N
    pos = N-pos-1
    pos = (pos*58) % N
    pos += 9978
    pos = (pos*48) % N
    pos = N-pos-1
    pos = (pos*66) % N
    pos += 1554
    pos = N-pos-1
    pos = pos-897 if pos >=897 else pos + N-897
    pos = (pos*36) % N
    return pos

print(shuffle3(N,pos))

pos = 2020
initial = 2020
N = 119315717514047
r = 101741582076661
t = time.time()
for _ in range(r):
    if _ % 1e7 == 0:
        print(_, time.time()-t)
    pos = shuffle3(N, pos)
    if pos == initial:
        print('Cycle', _)
        break
# print(pos)
