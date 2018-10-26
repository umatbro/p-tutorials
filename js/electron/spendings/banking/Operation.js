const _ = require('underscore');

class Operation {
    constructor(options) {
        options = _.defaults(options, {
            amount: 0,
            currency: 'PLN',
            supposedBalance: 0,
            description: '',
        });

        Object.assign(this, options);
    }
}

module.exports = Operation