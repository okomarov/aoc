# with open('data/day14.txt', 'r') as f:
with open('data/day14_test.txt', 'r') as f:
    data = f.read().splitlines()

I = {}
for l in data:
    inputs, outputs = l.split('=>')

    count = 0
    for c in outputs.strip().split(' '):
        count+=1
        if count % 2 == 1:
            num = int(c)
        else:
            material = c
            I[material] = {}
            I[material]['output'] = num

    count = 0
    for c in inputs.strip().split(' '):
        count+=1
        if count % 2 == 1:
            num = int(c)
        else:
            element = c.replace(',','')
            I[material][element] = num

def get_recipe(needed, element):
    return {
        'needed': needed,
        'output': {'element': element, 'qty': I[element]['output']},
        'inputs': {subk: subv for subk, subv in I[element].items() if subk != 'output'}
        }

def reaction(recipe, surplus):
    needed = recipe['needed']
    out_el = recipe['output']['element']
    out_qt = recipe['output']['qty']
    inputs = recipe['inputs']

    if out_el in surplus:
        if needed <= surplus[out_el]:
            surplus[out_el] -= needed
            return 0
        else:
            needed -= surplus[out_el]
            surplus[out_el] = 0
    else:
        surplus[out_el] = 0

    mult = np.ceil(needed/out_qt)
    new_needed = mult * out_qt
    surplus[out_el] += new_needed - needed

    ores = 0
    for k, v in inputs.items():
        if k == 'ORE':
            return mult*v
        else:
            new_recipe = get_recipe(mult*v, k)
            ores += reaction(new_recipe, surplus)

    return ores

surplus = {}
# part 1
min_reaction = reaction(get_recipe(1, 'FUEL'), surplus)
print(min_reaction)

# part 2
hi = int(1e12)
lo = 0
while hi > lo:
    mid = (hi+lo+1)//2
    surplus = {}
    cost = reaction(get_recipe(mid, 'FUEL'), surplus)
    print(cost)
    if cost < int(1e12):
        lo = mid
    else:
        hi = mid-1

print(lo)

