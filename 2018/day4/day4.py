import sys

# read in the lines
lines = sys.stdin.read().splitlines()
#lines = open('/home/alex/Coding/AdventOfCode2018/day4/test.txt', 'r').readlines()
lines.sort()

# go through each line and parse the line
guards = {}
iD = -1
start = 0
end = 0
for line in lines:
    if 'Guard' in line:
        j = 26
        tmp = ''
        while(line[j] != ' '):
            tmp = tmp + line[j]
            j += 1
        iD = int(tmp)

    if 'falls' in line:
        start = int(line[15:17])

    if 'wakes' in line:
        end = int(line[15:17])
        # clear the current guard 
        if not iD in guards:
            guards[iD] = {}
        for i in range(start, end):
            if not i in guards[iD]:
                guards[iD][i] = 0
            guards[iD][i] += 1

iD = 0
maxTotalSlept = 0
for key, value in guards.items():
    totalSlept = 0
    for key2, val in value.items():
        totalSlept += 1
    print(key, totalSlept)
    if totalSlept >= maxTotalSlept:
        iD = key
        maxTotalSlept = totalSlept

print ("Guard that slept the most:", iD)

minute = 0
maxSlept = 0
for key, value in guards[iD].items():
    if value >= maxSlept:
        minute = key
        maxSlept = value

print('minute slept the most:', minute)

print('part1', minute * iD)

# part 2
maxSlept = 0
minute = 0
iD = 0
for key, value in guards.items():
    for key2, val in value.items():
        if val > maxSlept:
            iD = key
            minute = key2
            maxSlept = val

print(iD,'slept the most on', minute, 'sleeping', maxSlept, 'times', 'Result:', iD * minute)