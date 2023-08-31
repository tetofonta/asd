def calc(w):
    o = set()
    for i in range(0, w-1):
        o.add((1, i))
        o.add((2+i, 1+i))
        o.add((1+i, 2+i))
        o.add((1+i, 3+i))
        o.add((3+i, 1+i))

    for i in range(1, w-3):
        o.add((i, w-2))

    o.add((w-1, w-1))
    o.remove((w-3, w-1))
    print([list(e) for e in o])
    print(len(o))

calc(1000)