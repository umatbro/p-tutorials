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