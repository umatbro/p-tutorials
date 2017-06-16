var str = "Please, locate where 'locate' occurs";
var pos = str.search("locate")
var poz = str.indexOf("locate")

console.log("pos: " + pos + ", poz: " + poz);

var htmlOutput = "";
var string = "Apple, Banana, Kiwi";

// slice obcina string od indeksu z parametru 1 do indeksu do parametru 2
// indeksy mogą być ujemne
// wtedy liczy się to od końca stringu
var sliceStr = string.slice(-12, -6);

// substring() to samo co slice tylko nie można podawać ujemnych argumentów
var substringStr = string.substring(7,13);

// zamienia pierwszy substring na drugi we wskazanym stringu
var replaced = string.replace("Apple", "Pizza");
// replace nie zmienia stringa wejściowego, tylko tworzy kopię

// zapisywanie stringów do arrayów

var txt = "1,2,3,4,5";
arr = txt.split(",");
console.log(arr);
document.getElementById("string").innerHTML = arr;
