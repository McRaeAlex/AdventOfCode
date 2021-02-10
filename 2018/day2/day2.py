import sys

def part1():
    lines = sys.stdin.readlines()
    twos = 0
    threes = 0
    for line in lines:
        table = {}
        foundTwo = False
        foundThree = False
        for char in line:
            if char in table:
                continue
            res = 0
            table[char] = True
            for char2 in line:
                if char == char2:
                    res = res + 1
            if res == 2:
                foundTwo = True
            elif res == 3:
                foundThree = True
        if foundTwo:
            twos = twos + 1
        if foundThree:
            threes = threes + 1
    print(twos * threes)

def compare(line, line2):
    diference = 0
    for char1, char2 in zip(line, line2):
        if char1 != char2:
            diference += 1
        if diference > 1:
            return False
           
    if diference == 1:
        return True
    else:
        return False

    
def part2():
    lines = sys.stdin.readlines()
    string1 = ""
    string2 = ""
    for line in lines:
        for line2 in lines:
            if compare(line, line2):
                print(line, line2)
            
def main():
    part2()

if __name__ == "__main__":
    main()
