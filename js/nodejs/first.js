let http = require('http');
let dt = require('./first_module')

http.createServer(function(req, res) {
  res.writeHead(200, {'Content-type': 'text/html'});
  let end = "The time and date currently is: " + dt.myDateTime();
  end += "<br/>You came from " + req.url;
  res.end(end);
}).listen(8000);
