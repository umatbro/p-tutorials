let url = require('url');
let http = require('http');

http.createServer(function(req, res) {
  res.writeHead(200, {'Content-type': 'text/html'});
  let q = url.parse(req.url, true).query;
  let txt = q.month + " " + q.year;
  res.end(txt);
}).listen(8000)
