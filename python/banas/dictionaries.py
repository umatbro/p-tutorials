import random
import sys
import os

super_villans = {
'Fiddler' : 'Isaac Bowin',
'Captain Cold' : 'Leonard Snart',
'Weather Wizard' : 'Mark Mardon',
'Mirror Master' : 'Sam Scudder'}

print(super_villans['Captain Cold'])

del super_villans['Fiddler']

# changing value under certain key
super_villans['Weather Wizard'] = 'Hartley Rathaway'

print(len(super_villans))
print(super_villans.get("Pied Piper"))
print(super_villans.get('Weather Wizard'))

print(super_villans.keys())
print(super_villans.values())
