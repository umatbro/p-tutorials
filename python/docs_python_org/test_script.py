# test script to check different things

# check what happens with variable after passing it to a function
def change_var(x):
    x += 10

n = 5
print "n var before function call: {}".format(n)
change_var(n)
print "n after function call: {}".format(n)

# function to next segment
def print_2_vars(a,b):
    print "var 1: {}\nvar 2: {}".format(a,b)

# checking syntax
a, b = 1, 10
print_2_vars(a,b)
a, b = b, a+b
print_2_vars(a,b)

# functions with default values
def ask_ok(prompt, retries=4, complaint="Yes or no, please"):
    while True:
        ok = raw_input(prompt)
        if ok in ('y', 'ye', 'yes'):
            return True
        if ok in ('n', 'no', 'nop', 'nope'):
            return False

        retries -= 1
        if retries < 0:
            raise IOError('refusenik user')
        print complaint

try:
    ask_ok("Do you really wanna quit? ", 2)
except IOError as e:
    print type(e)
