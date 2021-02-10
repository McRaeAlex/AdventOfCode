import sys

global line 
line = [int(x) for x in sys.stdin.readline().split()]
#line = [int(x) for x in open('/home/alex/Coding/AdventOfCode2018/day8/test.txt', 'r').readline().split()]

def parse(i):
    numberOfChildern = line[i]
    numberOfMeta = line[i + 1]
    children = []
    i += 2
    val = 0
    for _ in range(numberOfChildern):
        tmp, tmp2 = parse(i)
        i = tmp
        children.append(tmp2)
    for _ in range(numberOfMeta):
        if numberOfChildern == 0:
            val += line[i]
        elif len(children) > (line[i]-1) and (line[1] - 1) >= 0:
            val += children[line[i]-1]
        i += 1
    
    return (i, val)

_, val = parse(0)
print(val)