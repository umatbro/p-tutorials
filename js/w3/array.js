var cars = ["Saab", "Volvo", "BMW"];
cars.push("Volksvagen"); // push method - adds element to array

array = document.getElementById("array");
array.innerHTML = "cars: " + cars;

points = new Array(40); // creates table with 40 undefined elements
console.log(points);
array.innerHTML += "<br/> typeof(cars): " + typeof(cars) +
"<br/>Array.isArray(cars): " + Array.isArray(cars) +
"<br/>cars <style=\"color: blue;\">instanceof</style> Array: " + (cars instanceof Array) +
"<br/>cars.toString(): " + cars.toString() +
"<br/>cars.join(\" | \"): " + cars.join(" | ");

cars.pop(); // removes last element from an array
pushReturnValue = cars.push("Renault");
array.innerHTML += "<br/>cars.push return value: " + pushReturnValue;
cars.shift(); // removes first element
cars.unshift("Kia"); // adds element to the begenning of the array
delete cars[2]; // deletes index 2 from array. May leave undefined holes
console.log("Tabela po użyciu delete[2]");
console.log(cars);

var fruits = ["Banana", "Orange", "Apple", "Mango"];
array.innerHTML += "<br/>fruits: " + fruits.toString();

// The first parameter (2) defines the position where new elements should be added (spliced in).
// The second parameter (0) defines how many elements should be removed.
// The rest of the parameters ("Lemon" , "Kiwi") define the new elements to be added.
fruits.splice(2, 0, "Lemon", "Kiwi");
array.innerHTML += "<br/>po fruit.splice(2,0,\"Lemon\", \"Kiwi\"): " + fruits;

fruits.sort();
array.innerHTML += "<br/>fruits.sort(): " +  fruits;

// COMPARE FUNCTION
function compareNums(a,b){
  return a-b;
}
var points = [40, 100, 1, 5, 25, 10];
/**
compare function should return negative/zero or positive value depending on the arguments
if function returns negative => a is lower than b
returns zero: a is equal to b
returns positive a is greater than b
*/
points.sort(compareNums);
array.innerHTML += "<br/>points sorted: " + points;
