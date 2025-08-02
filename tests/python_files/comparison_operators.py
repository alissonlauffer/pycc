# Test comparison operators
a = 10
b = 5
c = 10

# Comparison operations
equal = a == c
not_equal = a != b
less_than = b < a
greater_than = a > b
less_equal = b <= a
greater_equal = a >= c

print("Comparison Operations:")
print(f"{a} == {c} = {equal}")
print(f"{a} != {b} = {not_equal}")
print(f"{b} < {a} = {less_than}")
print(f"{a} > {b} = {greater_than}")
print(f"{b} <= {a} = {less_equal}")
print(f"{a} >= {c} = {greater_equal}")

# Additional comparisons with different types
d = "hello"
e = "world"
f = "hello"

string_equal = d == f
string_not_equal = d != e

print("\nString Comparisons:")
print(f"'{d}' == '{f}' = {string_equal}")
print(f"'{d}' != '{e}' = {string_not_equal}")

# Boolean comparisons
x = True
y = False

bool_and = x and y
bool_or = x or y
bool_not = not x

print("\nBoolean Operations:")
print(f"{x} and {y} = {bool_and}")
print(f"{x} or {y} = {bool_or}")
print(f"not {x} = {bool_not}")