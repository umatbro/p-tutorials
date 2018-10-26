const assert = require('assert');
const Account = require('../../banking/Account');
const Operation = require('../../banking/Operation');

describe('Account', () =>{
    it('empty constructor should set balance properly', () => {
        let acc = new Account();
        assert.equal(acc.balance, 0);

        let acc2 = new Account(40);
        assert.equal(acc2.balance, 40);
    });

    describe('addOperation', () => {
        it('should change balance properly', () => {
            let acc = new Account(30);
            let positiveOp = new Operation({amount: 42});
            acc.addOperation(positiveOp);
            assert.equal(acc.balance, 72);

            let negativeOp = new Operation({amount: -10});
            acc.addOperation(negativeOp);
            assert.equal(acc.balance, 62);
        });

        it('should store operation history', () => {
            let acc = new Account(30);
            acc.addOperation(new Operation({description: 'first'}));
            acc.addOperation(new Operation({description: 'second'}));

            assert.deepEqual(
                ['first', 'second'],
                acc.getOperations().map((operation) => operation.description)
            );
        });
    });
});
