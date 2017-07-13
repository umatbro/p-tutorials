[Source](https://docs.python.org/3.6/tutorial/)

### for loops
**`range()`** function
```python
for i in range(5):
  print(i) # will print 0,1,2,3,4
```

Other `range()` function possibilities:
```python
range(5,10)
  5 through 9
range(0,10,3)
  0,3,6,9
range(-10,-100,-30)
  -10, -40, -70 (! no -100)
```

## Classes
#### Tip - static variables
Variables declared inside class definition but not inside a method are class (static) variables
```python
class ExampleClass:
  staticVar = 0
  def __init__(self):
    self.nonStaticVariable = True
```
