### Dictionaries syntax

```python
dictionary_in_curly_braces = {'Key' : 'Value'}
```

### Conditionals

#### Statements:
* `if`
* `else`
* `elif`

Syntax:
```python
if age >= 21 :
  print('You are old enough to drive a motorcycle')
elif age >= 16 :
  print('You are old enough to drive a car')
else :
  print('You are not old enough')
# following code (note whitespace)
```

#### Logical operators:
* `and`
* `or`
* `not`

```python
if ( age >= 1 and age <= 18) :
  print('You are pretty young')
```

### Looping
```python
# regular loop, executed 10 times
for x in range(0,10) :
  print(x, ' ', end="")
```
Equivalent of C/C++'s
```c++
for(int i = 0; i < 10; i++){
  printf("%d ", i);
}
```
#### Printing all elements from a list
```python
for your_item in your_list :
  print(your_item) # will cycle through your_list
```

#### Define numbers for loop to cycle through
```python
for x in [2,4,6,8,10] :
  print(x)
```

#### while loop
```python
while(condition) :
  print(value)
```

### Reading input from user
```python
name = sys.stdin.readline()
```

### Objects
```python
class Animal:
  __name = ""
  __weight = 0
```
Preceding variable names with two underscores makes them *private*.

**`self`** = Python's `this` equivalent.

#### Constructor
`def __init__(self)`

#### inheritance
```python
class Dog(Animal):
  # class Dog now inherits from class Animal
```
