# with open('data/day22_test.txt') as f:
with open('data/day22.txt') as f:
    data = f.read().splitlines()

# Part 1
# =================================
# After shuffling your factory order deck of 10007 cards,
# what is the position of card 2019?
N = 10007
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

# Part 2
# =================================
# After shuffling your new, giant, factory order deck that
# many times, what number is on the card that ends up in position 2020?

N = 119315717514047
repeat = 101741582076661

# After trying several speedups, as e.g. tracking position only instead of
# actually shuffling cards, I gave up as modular arithmetic is beyond 
# my understanding at the moment.

def shuffle_pos(N, pos):
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
