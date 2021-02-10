import sys

def part2():
    lines = sys.stdin.readlines()
    sum = 0
    answer = 0
    table = {
        0: 0
    }
    notFound = True
    while(notFound):
        for line in lines:
            num = int(line)
            sum += num
            print("Sum: ", sum, "\tNum: ", num)
            if not sum in table:
                table[sum] = 1
            else:
                table[sum] = table[sum] + 1
            if table[sum] >= 2:
                answer = sum
                notFound = False
                break

    print(answer)

def part1():
    sum = 0
    for line in sys.stdin:
        sum += int(line)

    print(sum)

def main():
    part2()

if __name__ == "__main__":
    main()