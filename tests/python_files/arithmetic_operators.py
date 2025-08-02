# Test arithmetic operators
a = 10
b = 3

# Basic arithmetic operations
addition = a + b
subtraction = a - b
multiplication = a * b
division = a / b
floor_division = a // b
modulo = a % b
exponentiation = a ** b

print("Arithmetic Operations:")
print(f"{a} + {b} = {addition}")
print(f"{a} - {b} = {subtraction}")
print(f"{a} * {b} = {multiplication}")
print(f"{a} / {b} = {division}")
print(f"{a} // {b} = {floor_division}")
print(f"{a} % {b} = {modulo}")
print(f"{a} ** {b} = {exponentiation}")

# Unary operators
positive_a = +a
negative_a = -a

print("\nUnary Operations:")
print(f"+{a} = {positive_a}")
print(f"-{a} = {negative_a}")