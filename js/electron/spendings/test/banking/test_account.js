const assert = require('assert');
const Account = require('../../banking/Account');
const Operation = require('../../banking/Operation');

describe('Account', () => {
  it('empty constructor should set balance properly', () => {
    const acc = new Account();
    assert.equal(acc.balance, 0);

    const acc2 = new Account(40);
    assert.equal(acc2.balance, 40);
  });

  describe('addOperation', () => {
    it('should change balance properly', () => {
      const acc = new Account(30);
      const positiveOp = new Operation({ amount: 42 });
      acc.addOperation(positiveOp);
      assert.equal(acc.balance, 72);

      const negativeOp = new Operation({ amount: -10 });
      acc.addOperation(negativeOp);
      assert.equal(acc.balance, 62);
    });

    it('should store operation history', () => {
      const acc = new Account(30);
      acc.addOperation(new Operation({ description: 'first' }));
      acc.addOperation(new Operation({ description: 'second' }));

      assert.deepEqual(
        ['first', 'second'],
        acc.getOperations().map(operation => operation.description),
      );
    });
  });

  describe('history', () => {
    let self;
    beforeEach(() => {
      self = {};
      self.acc = new Account();
      self.operations = [30, 40, 50].map(amount => new Operation({ amount }));
      self.operations.forEach(operation => self.acc.addOperation(operation));
    });

    it('should have last account balances in order', done => done('todo'));
    it('should have last balance changes in order', done => done('todo'));
  });
});
