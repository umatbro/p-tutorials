import random

def crc(data, n, crc_generator=None):
    """
    Calculate checksum for given message

    :param data: message to calculate checksum from
    :param n: number of bits of checksum
    :param crc_generator: generator, should have n+1 digits (in binary)
    :return: 2 element tuple containing calculated checksum and generator used to calculate it
    """
    if crc_generator is None:
        crc_generator = random.getrandbits(n + 1)
    if type(crc_generator) is str:
        crc_generator = int(crc_generator, 2)
        
    # print_b(crc_generator, n+1)

    data = int(data, 2)
    # print_b(data)
    # input(f'add {n}x0 in the end')
    result_data = data << n
    # print_b(result_data)
    # print_pair(result_data, crc_generator)

    # while my_data >= int('1' * n, 2):
    # condition in a while loop chechs whether last n digits are 0s
    while result_data & int('1' * n, 2) == 0:
        # move generator max to the left
        # (there will be trailing 0s in the number - it's OK since 0 does not change
        # initial value in XOR operation) 
        temp_gen = crc_generator << (len(bin(result_data)) - len(bin(crc_generator)))
        # input(f'calculate temporary generator {bin(temp_gen)}')
        # print_pair(result_data, temp_gen)
        result_data = result_data ^ temp_gen

        # input('after applying generator')
        # print_b(result_data)

    return result_data, crc_generator

def print_b(number: int, ndigits: int=4):
    formatter = '{{0:0{b}b}}'.format(b=ndigits)
    print(formatter.format(number))

def print_pair(top, bottom, bottom_spaces=0):
    print('------------')
    print_b(top)
    print(' ' * bottom_spaces, end='')
    print_b(bottom)
    print('------------')

if __name__ == '__main__':
    # crc('11010011101110', 3, '1011')
    print(tuple(map(bin, crc('0110111', 4, '10011'))))

    # wiki angielska
    # crc('11010011101100', 3, '1011')
