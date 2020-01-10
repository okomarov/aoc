with open('data/day22.txt') as f:
    data = f.read().splitlines()

def qemod(a, b, m):  # quick exponent a**b mod m
    if b==0: return 1
    if b%2:
        return (a*qemod((a**2)%m, b//2, m)) % m
    return qemod((a**2)%m, b//2, m)

def exeuclid(a,b):
    # d == gcd(a,b) == ax+by
    if b==0:
        return a,1,0  # a == gcd(a,0) == 1*a + 0*0
    d,x,y = exeuclid(b, a%b)
    return d, y, x - (a//b)*y

def modinv(a,b):
    # a**-1 mod b
    d,x,y = exeuclid(a,b)
    if d==1:
        return x%b
    return 0

def num_at_pos(sz, reps, shuf, p):
    # return (position of p, number at position p)
    # after shuffling the deck of 'sz' cards 'reps' times (with shuf)
    # We use that both are a linear function of p (second inverse of first):
    # if p1-p0=x then position of p after 1 rep is p0 + x*p  (mod sz)
    # and number at position p is (pos-p0) * (x**-1)  (mod sz)
    # rep 2: p0*(1 + x + x**2) + (x**2)(p-p0)
    p0 = 0
    p1 = 1
    for line in shuf:
        if line.endswith("stack"):
            # reverse: 0 => sz-1
            p0=~p0
            p1=~p1
            continue
        n = int(line.split()[-1])
        if line.startswith('cut'):
            p0-=n
            p1-=n
        else:
            p0*=n
            p1*=n
        p0%=sz
        p1%=sz
    x = (p1-p0)
    # Use the identity x^n+...+x^2+x+1 = (x^(n+1)-1)//(x-1)
    x_reps = qemod(x, reps, sz)
    #   p0 * (x^(n+1) -1) * ((x-1)**-1)
    dx = p0*((x_reps*x-1) * modinv(x-1, sz)) % sz
    pos_of_p = (dx + x_reps*(p-p0)) % sz
    # invert the permutation
    num_at_p = ((p-dx) * modinv(x_reps, sz) + p0) % sz
    return pos_of_p, num_at_p

print("Part 2:", num_at_pos(119315717514047, 101741582076661, data, 2020)[1])