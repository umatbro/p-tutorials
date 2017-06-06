# modules are like classes but they can't be instantiaded
# = they're kinda static classes
# you can inherit ONLY ONE class, but
# you can inherit MANY modules.

require_relative "human"
require_relative "smart"

module Animal
  def make_sound
    puts "Grr"
  end
end

class Dog
  include Animal
end

rover = Dog.new
rover.make_sound

class Scientist
  include Human
  prepend Smart

  def act_smart
    return "E = mc4444"
  end
end

einstein = Scientist.new
einstein.name = "Albert"

puts einstein.name
einstein.run

puts einstein.name + " says " + einstein.act_smart
