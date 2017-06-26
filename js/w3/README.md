[W3 JS Tutorial](https://www.w3schools.com/js/default.asp) HOME page

## Booleans
[Link](https://www.w3schools.com/Js/js_booleans.asp) <br/>
*Everything without a "Real" is False"*
```js
// The Boolean value of 0 (zero) is false:
var x = 0;
Boolean(x);       // returns false
// The Boolean value of -0 (minus zero) is false:
var x = -0;
Boolean(x);       // returns false
// The Boolean value of "" (empty string) is false:
var x = "";
Boolean(x);       // returns false
// The Boolean value of undefined is false:
var x;
Boolean(x);       // returns false
// The Boolean value of null is false:
var x = null;
Boolean(x);       // returns false
// The Boolean value of false is false:
var x = false;
Boolean(x);       // returns false
// The Boolean value of NaN is false:
var x = 10 / "H";
Boolean(x);       // returns false
```
### Booleans can be objects
```js
var x = false; // typeof x returns boolean
var y = new Boolean(false); // typeof x returns boolean
```

## Comparison and logical operators

```js
// given:
let x = 5;
x == "5"; // true
x === 5 // true
x === "5" // false
// !== not equal value or not equal type
x !== "5" // true

```

### Ternary operator
`variable = test == "is true?" true : false`

### Break and continue
`break` - breaks loop and continues executing
`continue` - breaks only one iteration and continues with the next iteration of the loop
