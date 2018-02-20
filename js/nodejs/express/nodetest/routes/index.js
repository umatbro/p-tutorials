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

router.get('/newuser', function(req, res) {
    res.render('newuser', { title: 'Add new user'} );
});

router.post('/adduser', function (req, res) {
    // set internal db variable
    let db = req.db;

    // get form values. They rely on "name" attributes
    let userName = req.body.username;
    let userEmail = req.body.useremail;

    // set our collection
    let collection = db.get('usercollection');

    // submit to DB
    collection.insert({
        "username": userName,
        "email" : userEmail
    }, function(err, doc) {
        if (err) {
            // if fail return error
            res.send("There was a problem adding information to the database");
        } else {
            // forward to success page
            res.redirect("userlist");
        }
    });
});

module.exports = router;
