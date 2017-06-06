# OBJECTS PART 29:52

class Animal
  #initialize = constructor
  def initialize
    puts "Creating new animal"
  end

  def set_name(new_name)
    @name = new_name
  end

  def get_name
    @name
  end

   def name
     @name
   end

  def name=(new_name)
    if new_name.is_a?(Numeric)
      puts "Name can't be a number"
    else
      @name = new_name
    end
  end
end

cat = Animal.new
cat.set_name("Peekaboo")

puts cat.get_name
puts cat.name

cat.name = "Sophie"

puts cat.name

class Dog
  # getters and setters
  attr_reader :name, :height, :weight
  attr_writer :name, :height, :weight
  # getters  and setters together
  attr_accessor :name, :height, :weight

  def bark
    return "Generic bark"
  end
end

rover = Dog.new
rover.name = "Rover"
puts rover.name

class GermanShepard < Dog
  def bark
    return "Loud bark"
  end
end

max = GermanShepard.new
#max.name = "Max"
printf "%s goes %s\n", max.name, max.bark
