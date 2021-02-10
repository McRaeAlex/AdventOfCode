import sys


class layout():
    def __init__ (self, id, x, y, width, height):
        self.id = id
        self.x = x
        self.y = y
        self.width = width
        self.height = height
    
    def __repr__ (self):
        return "id: %d, xy: (%d, %d), width: %d, height: %d" % (self.id, self.x, self.y, self.width, self.height)

def getClaims():
    # read the file into a list of structs
    #lines = open('/home/alex/Coding/AdventOfCode2018/day3/test1.txt', 'r')
    lines = sys.stdin.readlines()
    claims = []
    for line in lines:
        tmparr1 = line.split(' ')
        id = int(tmparr1[0][1:])
        xy = tmparr1[2].split(',') # get the xy
        x = int(xy[0])
        y = int(xy[1][:-1]) # remove the last character
        wh = tmparr1[3].split('x')
        w = int(wh[0]) # get w
        h = int(wh[1][:-1])
        claims.append(layout(id, x, y, w, h))
    return claims

def checkOverLap(item1, item2):
    area1 = (item1.width) * (item1.height)
    area2 = (item2.width) * (item2.height)
    areaI = (min(item1.x + item1.width, item2.x + item2.width)\
            -  max(item1.x, item2.x)) *\
            (min(item1.y + item1.height, item2.y + item2.height)\
            - max(item1.y, item2.y))
    return (areaI)

def checkIfOverLap(item1, item2):
    return (item1.x < (item2.x + item2.width)\
    and (item1.x + item1.width) > item2.x\
    and item1.y > (item2.y + item2.height)\
    and (item1.y + item1.height) < item2.y)

def part1():
    claims = getClaims()
    # Completely misunderstood the question
    overlapping = 0
    fabric = [[0 for i in range(1000)] for i in range(1000)] # create the fabric
    for claim in claims:
        for x in range(claim.x, claim.x + claim.width):
            for y in range(claim.y, claim.y + claim.height):
                fabric[y][x] += 1
    for i in range(1000):
        for j in range(1000):
            if fabric[i][j] > 1:
                overlapping += 1
    print("Overlapping: ", overlapping)
    return (overlapping, fabric, claims)

def part2():
    (overlap, fabric, claims) = part1()
    for claim in claims:
        collision = False
        for x in range(claim.x, claim.x + claim.width):
            for y in range(claim.y, claim.y + claim.height):
                if fabric[y][x] > 1:
                    collision = True
                    break
            if collision:
                break
        if not collision:
            print("ID:", claim.id)
        
                

if __name__ == "__main__":
    part2()