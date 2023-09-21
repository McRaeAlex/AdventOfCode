
file = open('./input.txt')


def part1():
    current = 0
    max = 0
    for line in file.readlines():
        line = line.strip()
        if line == "":
            if max < current:
                max = current
            current = 0
        else:
            current += int(line)


    # the last line is may not be empty
    if max < current:
        max = current

    print("max load:", max)

def part2():
    current = 0
    maxes = [0,0,0]

    for line in file.readlines():
        line = line.strip()
        if line == "":
            maxes.sort(reverse=True)
            if maxes[2] < current:
                maxes[2] = current
            current = 0
        else:
            current += int(line)
    
    maxes.sort()
    if maxes[2] < current:
        maxes[2] = current
    
    print("top 3 load:", maxes[0] + maxes[1] + maxes[2])

part1()
file.seek(0)
part2()