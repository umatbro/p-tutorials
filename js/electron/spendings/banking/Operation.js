const _ = require('underscore');

class Operation {
  constructor(options) {
    const opts = _.defaults(options, {
      amount: 0,
      currency: 'PLN',
      supposedBalance: 0,
      description: '',
    });

    Object.assign(this, opts);
  }
}

module.exports = Operation;
