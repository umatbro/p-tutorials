Tutorials based on youtube series by [thenewboston](https://www.youtube.com/channel/UCJbPGzawDH1njbqV-D5HqKw)<br/>
Youtube [video playlist](https://www.youtube.com/watch?v=tvC1WCdV1XU&index=1&list=PLAE85DE8440AA6B83)

## Tutorial 21 - Assignment and Increment Operators 
There is a difference between `x++` and `++x`
```c++
int x = 20;
cout << x++; // this line first prints 20 and then increments x to 21
cout << x;   // this will print 21
```
```c++
int x = 20;
cout << ++x; // x will be incremented before printing - so the value printed will be 21
cout << x;   // this will still print 21
```

## Tutorial 40 - sizeof
Cool trick - get number get number of elements in an array
```c++
double table[30];
int sizeOfTable = sizeof(table) / sizeof(table[0]);
```
Size of `table` is being computed by dividing size of whole array by size of only one element.

## Tutorial 45 - member initializers

Members of a such class: 
```c++
class Sally {
private:
    int regVar; // regular variable
    const int constVar; // constant variable
public:
    Sally(int a, int b);
    void print();
};
```
can be initialized this way:
```c++
Sally::Sally(int a, int b)
: regVar(a), constVar(b)
{}
```
Syntax to assign values to class members is following
Constructor -> arguments in brackets -> colon -> member(argument) (...) -> constructor body