const Operation = require('./Operation');

class Account {
    constructor(initValue) {
        this.balance = initValue || 0;
        this.__operations = [];
    }

    addOperation(operation) {
        this.balance += operation.amount;
        this.__operations.push(operation);
    }

    getOperations() {
        return this.__operations;
    }
}

module.exports = Account;