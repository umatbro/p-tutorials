let fs = require('fs');
let http = require('http');

fs.appendFile('new_file.txt', 'Hello', function(err) {
  if (err) throw err;
  console.log("Saved");
});

http.createServer(function(req, res) {
  fs.readFile('first.js', function(err, data) {
    res.writeHead(200, {'Content-Type': 'text/html'});
    res.write(data);
    res.end();
  });
}).listen(8000);
