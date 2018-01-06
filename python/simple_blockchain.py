"""
This is a simple blockchain implementation based on video tutorial: https://www.youtube.com/watch?v=zVqczFZr124
"""
import json
from hashlib import sha256


class Block:
    def __init__(self, timestamp, data, previous_hash='', index=None):
        self.index = index
        self.timestamp = timestamp
        self.data = data
        self.hash = None
        self.previous_hash = previous_hash
        self.nonce = 0

        self.hash = self.calculate_hash()

    def calculate_hash(self):
        sha = sha256()
        to_hash = str(self.index) + self.timestamp + str(self.data) + self.previous_hash + str(self.nonce)
        sha.update(bytes(to_hash, encoding='utf-8'))
        return sha.digest().hex()

    def mine_block(self, difficulty):
        while not self.hash[:difficulty] == '0' * difficulty:  # while not first <difficulty> numbers are 0
            self.hash = self.calculate_hash()
            self.nonce += 1

        print('Block mined: {}'.format(self.hash))
        return self

    def __str__(self):
        return json.dumps(self.__dict__, indent=2)


class BlockChain:
    def __init__(self, difficulty=2):
        self.chain = [self.create_genesis_block()]
        self.difficulty = difficulty

    @staticmethod
    def create_genesis_block():
        return Block('01/11/2017', 'Genesis block', '0', index=0)

    @property
    def last_block(self):
        return self.chain[-1]

    def add_block(self, new_block: Block):
        if new_block.index is None:
            new_block.index = self.last_block.index + 1
        new_block.previous_hash = self.last_block.hash
        # new_block.hash = new_block.calculate_hash()
        new_block.mine_block(self.difficulty)
        self.chain.append(new_block)

        return self

    @property
    def is_valid(self):
        for i, block in enumerate(self.chain[1:]):  # we skip 1st block because it is genesis block
            prev_block = self.chain[i]
            if block.hash != block.calculate_hash():
                return False
            if block.previous_hash != prev_block.hash:
                return False

        return True

    def __str__(self):
        return '\n'.join([str(block) for block in self.chain])

    def __getitem__(self, item):
        return self.chain[item]


if not __name__ == '__main__':
    mat_chain = BlockChain()
    mat_chain.add_block(Block('11/12/2017', {'dupsko': 1}))
    mat_chain.add_block(Block('12/12/2017', {'dupsko': 2}))

    # print(mat_chain)
    print('Validation before changes:', mat_chain.is_valid)
    print('CHanging block 1 data...')
    mat_chain.chain[1].data = {'dupa': 100}
    # print(mat_chain)
    print('Validation after changes:', mat_chain.is_valid)
    print('Updating block hash...')
    mat_chain.chain[1].hash = mat_chain.chain[1].calculate_hash()
    print(mat_chain)
    print('Validation after hash recalculation: ', mat_chain.is_valid)


if __name__ == '__main__':
    mat_chain = BlockChain(6)
    print('Mining block 1...')
    mat_chain.add_block(Block('20/11/2017', {'amount': 100}))
    print(mat_chain[1].nonce)

    print('Mining block 2...')
    mat_chain.add_block(Block('20/11/2017', {'amount': 300}))
    print('Nonce:', mat_chain[2].nonce)
