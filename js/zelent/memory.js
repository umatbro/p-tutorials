let images = [
  "ciri.png",     // 1
  "geralt.png",
  "jaskier.png",
  "jaskier.png",
  "iorweth.png",
  "triss.png",
  "geralt.png",
  "yen.png",
  "ciri.png",
  "triss.png",
  "yen.png",
  "iorweth.png",
];

// alert(cards);

cards = [];
for(let i = 0; i < 12; i++){
  // find card on page
  let card = document.getElementById("c" + i);
  // add listener
  card.addEventListener("click", function(){
    revealCard(i);
  });

  // store in array
  cards.push(card);
}

console.log(cards);

let oneVisible = false;
let turnCounter = 0;
let visibleNumber;
let lock = false;
let pairsLeft = 6;

function revealCard(nr){
  let opacityValue = $('#c' + nr).css('opacity');

  // alert('Opacity: ' + opacityValue);
  if(opacityValue != 0 && !lock)
  {
    lock = true;
    let image = 'url(img/' + images[nr] + ")";
    $('#c' + nr).css('background-image', image);
    $('#c' + nr).addClass('card-active');
    $('#c' + nr).removeClass('card');

    if(!oneVisible){
      oneVisible = true;
      visibleNumber = nr;
      lock = false;
    }
    else {
      if (images[visibleNumber] == images[nr]){ // pair found
        hide2Cards(visibleNumber, nr);
      } else { // fail
        restore2Cards(visibleNumber, nr);
      }

      turnCounter++;
      $('.score').html('Turn counter: ' + turnCounter);
      oneVisible = false;
    }
  }
}


function hide2Cards(x, y){
  setTimeout(function(){
    $('#c' + x).css('opacity', 0);
    $('#c' + y).css('opacity', 0);
  }, 400);

  pairsLeft--;
  if(pairsLeft == 0){
    $('.board').html('<h1>You win</br>Done in ' + turnCounter + ' turns</h1>');
  }

  lock = false;
}

function restore2Cards(x, y){
  setTimeout(function(){
    $('#c' + x).css('background-image', 'url(img/karta.png)');
    $('#c' + x).addClass('card');
    $('#c' + x).removeClass('card-active');

    $('#c' + y).css('background-image', 'url(img/karta.png)');
    $('#c' + y).addClass('card');
    $('#c' + y).removeClass('card-active');
  }, 1000);

  lock = false;
}
