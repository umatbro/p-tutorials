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

router.delete('/deleteuser', (req, res) => {
  let db = req.db;
  let usersCollection = db.get('userlist');
  let userToDelete = req.body.id;
  usersCollection.remove({_id: userToDelete}, err => {
    res.send({
        msg: (err === null) ? '' : `Error: ${err}`
    });
  });
});

module.exports = router;
