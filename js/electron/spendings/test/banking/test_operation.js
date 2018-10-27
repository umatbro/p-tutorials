const assert = require('assert');
const Operation = require('../../banking/Operation');

describe('Operation', () => {
  it('should work with empty constructor', () => {
    const op = new Operation();
    assert.equal(op.amount, 0);
    assert.equal(op.supposedBalance, 0);
    assert.equal(op.currency, 'PLN');
    assert.equal(op.description, '');
  });

  it('should work with filled options', () => {
    const op = new Operation({
      amount: 30,
      supposedBalance: 120,
      description: 'Desc',
    });
    assert.equal(op.amount, 30);
    assert.equal(op.supposedBalance, 120);
    assert.equal(op.currency, 'PLN');
    assert.equal(op.description, 'Desc');
  });
});
