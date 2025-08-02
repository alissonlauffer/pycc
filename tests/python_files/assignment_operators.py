# Test assignment operators
a = 20
b = 10

print("Initial values:")
print(f"a = {a}")
print(f"b = {b}")

# Addition assignment
a += b
print(f"\nAfter a += {b}: a = {a}")

# Subtraction assignment
a -= 5
print(f"After a -= 5: a = {a}")

# Multiplication assignment
a *= 2
print(f"After a *= 2: a = {a}")

# Division assignment
a /= 3
print(f"After a /= 3: a = {a}")

# Floor division assignment
a //= 2
print(f"After a //= 2: a = {a}")

# Modulo assignment
a %= 3
print(f"After a %= 3: a = {a}")

# Exponentiation assignment
a **= 2
print(f"After a **= 2: a = {a}")

# Reset a for bitwise operations
a = 12  # 1100 in binary
print(f"\nReset a to {a} (binary: {bin(a)})")

# Bitwise AND assignment
a &= 10  # 1010 in binary
print(f"After a &= 10: a = {a} (binary: {bin(a)})")

# Bitwise OR assignment
a |= 4   # 0100 in binary
print(f"After a |= 4: a = {a} (binary: {bin(a)})")

# Bitwise XOR assignment
a ^= 2   # 0010 in binary
print(f"After a ^= 2: a = {a} (binary: {bin(a)})")

# Left shift assignment
a <<= 1
print(f"After a <<= 1: a = {a} (binary: {bin(a)})")

# Right shift assignment
a >>= 1
print(f"After a >>= 1: a = {a} (binary: {bin(a)})")

# Multiple assignments in one line
x = y = z = 100
print(f"\nMultiple assignment: x = y = z = {x}")

# Chained assignment
p = q = r = 50
print(f"Chained assignment: p = q = r = {p}")

# Augmented assignment with different types
s = "Hello"
s += " World"
print(f"\nString augmented assignment: s += ' World' => s = '{s}'")

t = [1, 2, 3]
t += [4, 5]
print(f"List augmented assignment: t += [4, 5] => t = {t}")