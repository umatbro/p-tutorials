let paragraph =  document.getElementById("bc");
paragraph.innerHTML += "<b>break:<b>";
for (let i = 0; i < 10; i++){
  if(i == 3){
    paragraph.innerHTML += "<br/><b><code>break</code> is here!</b>";
    break;
  }
  paragraph.innerHTML += "<br/>Iteration number " + i;
}

paragraph.innerHTML += "<br/><br/><b>continue:<b>";

for (let i = 0; i < 10; i++){
  if(i == 3){
    paragraph.innerHTML += "<br/><b><code>continue</code> is here!</b>";
    continue;
  }
  paragraph.innerHTML += "<br/>Iteration number " + i;
}
