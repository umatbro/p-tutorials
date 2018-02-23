let express = require('express');
let router = express.Router();

// GET userlist
router.get('/userlist', function(req, res) {
  let db = req.db;
  let collection = db.get('userlist');
  collection.find({}, {}, function(err, docs) {
    res.json(docs);
  });
});

router.post('/adduser', function(req, res) {
  let db = req.db;
  let collection = db.get('userlist');
  collection.insert(req.body, (err, result) => {
    res.send((err === null) ? {msg: ''} : {msg: err});
  });
});

module.exports = router;
