# Test bitwise operators
a = 12  # 1100 in binary
b = 10  # 1010 in binary

# Bitwise operations
bitwise_and = a & b
bitwise_or = a | b
bitwise_xor = a ^ b
bitwise_not = ~a
left_shift = a << 2
right_shift = a >> 2

print("Bitwise Operations:")
print(f"{a} & {b} = {bitwise_and} (binary: {bin(bitwise_and)})")
print(f"{a} | {b} = {bitwise_or} (binary: {bin(bitwise_or)})")
print(f"{a} ^ {b} = {bitwise_xor} (binary: {bin(bitwise_xor)})")
print(f"~{a} = {bitwise_not} (binary: {bin(bitwise_not)})")
print(f"{a} << 2 = {left_shift} (binary: {bin(left_shift)})")
print(f"{a} >> 2 = {right_shift} (binary: {bin(right_shift)})")

# Additional bitwise operations with different values
x = 15  # 1111 in binary
y = 7   # 0111 in binary

print("\nAdditional Bitwise Operations:")
print(f"{x} & {y} = {x & y} (binary: {bin(x & y)})")
print(f"{x} | {y} = {x | y} (binary: {bin(x | y)})")
print(f"{x} ^ {y} = {x ^ y} (binary: {bin(x ^ y)})")
print(f"~{y} = {~y} (binary: {bin(~y & 0xFF)})")  # Masking to show only 8 bits

# Bitwise operations with negative numbers
negative_a = -12
negative_b = -10

print("\nBitwise Operations with Negative Numbers:")
print(f"{negative_a} & {negative_b} = {negative_a & negative_b}")
print(f"{negative_a} | {negative_b} = {negative_a | negative_b}")
print(f"{negative_a} ^ {negative_b} = {negative_a ^ negative_b}")
print(f"~{negative_a} = {~negative_a}")

# Bit shifting with negative numbers
print("\nBit Shifting with Negative Numbers:")
print(f"{negative_a} << 2 = {negative_a << 2}")
print(f"{negative_a} >> 2 = {negative_a >> 2}")