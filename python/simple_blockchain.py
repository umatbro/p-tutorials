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
        self.hash = self.calculate_hash()

    def calculate_hash(self):
        sha = sha256()
        to_hash = str(self.index) + self.timestamp + str(self.data) + self.previous_hash
        sha.update(bytes(to_hash, encoding='utf-8'))
        return sha.digest().hex()

    def __str__(self):
        return json.dumps(self.__dict__, indent=4)


class BlockChain:
    def __init__(self):
        self.chain = [self.create_genesis_block()]

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
        new_block.hash = new_block.calculate_hash()
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


if __name__ == '__main__':
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
