import sys

#line = sys.stdin.read()
line = open('/home/alex/Coding/AdventOfCode2018/day5/input.txt', 'r').readline()
line = line[:-1]

alphabet = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z']

results = {}

for char in alphabet:
    breakHappened = True
    result = [a for a in line if a.lower() != char]
    while(breakHappened):
        breakHappened = False
        tmp = ''
        i = 0
        skip = False
        while(i<len(result)-1):
            firstLetter = result[i]
            i += 1
            secondLetter = result[i]
            if not (firstLetter != secondLetter and firstLetter.lower() == secondLetter.lower()):
                if not skip:
                    tmp += firstLetter
                skip = False
            elif skip == False:
                breakHappened = True
                skip = True
            else:
                skip = False
        if not skip:
            tmp += result[i]
        result = tmp

    #print(result)
    print(len(result))
    results[char] = len(result)

smallest = 100000000000000000000000000000
for key, value in results.items():
    if value < smallest:
        smallest = value
        print(key)

print(smallest)