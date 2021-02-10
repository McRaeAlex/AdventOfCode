import sys

lines = sys.stdin.readlines()

# Parse the input
coords = []
infinite = {}
areas = {(-1,-1):0}

for line in lines:
    data = line.split(',')
    coord = (int(data[0]), int(data[1]))
    coords.append(coord)
    infinite[coord] = False
    areas[coord] = 0

regionCount = 0
for x in range(500):
    for y in range(500):
        totalDist = 0
        for coord in coords:
            totalDist += abs(coord[0] - x) + abs(coord[1] - y)
        if totalDist < 10000:
            regionCount += 1

print(regionCount)