inp = [map(int, line.split("   ")) for line in open("input/day1.txt").read().strip().split("\n")]
a = sorted([a for a, b in inp])
b = sorted([b for a, b in inp])
print(sum(abs(a-b) for a, b in zip(a, b)))
print(sum(a*sum(1 for b in b if a == b) for a in a))
