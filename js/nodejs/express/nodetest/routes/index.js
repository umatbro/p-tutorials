var express = require('express');
var router = express.Router();

/* GET home page. */
router.get('/', function(req, res, next) {
  res.render('index', { title: 'Express' });
});

router.get('/helloworld', function(req, res) {
  res.render('helloworld', {title: 'Hello world'});
});

/* GET userlist page */
router.get('/userlist', function(req, res) {
  let db = req.db;
  let collection = db.get('usercollection');
  collection.find({}, {}, function (e, docs) {
      res.render('userlist', {
        'userlist': docs
      });
      /* Code for plain JSON response */
      // for (let entry of docs) {
      //   delete entry._id;
      // }
      // res.send(docs);
  });
});

module.exports = router;
