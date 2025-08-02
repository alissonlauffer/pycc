# Test logical operators
a = True
b = False
c = True
d = False

# Logical operations
and_result = a and b
or_result = a or b
not_result = not a

print("Logical Operations:")
print(f"{a} and {b} = {and_result}")
print(f"{a} or {b} = {or_result}")
print(f"not {a} = {not_result}")

# More complex logical expressions
complex_and = (a and c) and (b and d)
complex_or = (a or b) or (c or d)
complex_not = not (a and b)

print("\nComplex Logical Operations:")
print(f"({a} and {c}) and ({b} and {d}) = {complex_and}")
print(f"({a} or {b}) or ({c} or {d}) = {complex_or}")
print(f"not ({a} and {b}) = {complex_not}")

# Logical operations with non-boolean values
x = 10
y = 0
z = "hello"
w = ""

# In Python, empty values are considered False in boolean context
# Non-empty values are considered True in boolean context
value_and = x and y
value_or = x or y
value_not = not z

print("\nLogical Operations with Non-boolean Values:")
print(f"{x} and {y} = {value_and}")
print(f"{x} or {y} = {value_or}")
print(f"not '{z}' = {value_not}")

# Short-circuit evaluation
def true_func():
    print("true_func called")
    return True

def false_func():
    print("false_func called")
    return False

print("\nShort-circuit Evaluation:")
print("Testing 'False and true_func()':")
result1 = False and true_func()  # true_func won't be called

print("Testing 'True or false_func()':")
result2 = True or false_func()   # false_func won't be called

print(f"Result of 'False and true_func()': {result1}")
print(f"Result of 'True or false_func()': {result2}")