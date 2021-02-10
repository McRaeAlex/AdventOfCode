import sys

global line 
line = [int(x) for x in sys.stdin.readline().split()]

def parse(i):
    numberOfChildern = line[i]
    numberOfMeta = line[i + 1]
    i += 2
    val = 0
    for _ in range(numberOfChildern):
        tmp, tmp2 = parse(i)
        i = tmp
        val += tmp2
    for _ in range(numberOfMeta):
        val += line[i]
        i += 1
    
    return (i, val)

_, val = parse(0)
print(val)