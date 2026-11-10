"""Hidden check: lcm is the least common multiple, including zero and negatives."""

from numtheory import gcd, lcm

for a in range(-12, 13):
    for b in range(-12, 13):
        g = gcd(a, b)
        m = lcm(a, b)
        assert m >= 0, (a, b, m)
        assert g * m == abs(a * b), (a, b, g, m)
        if a == 0 or b == 0:
            assert m == 0, (a, b, m)
print("ok")
