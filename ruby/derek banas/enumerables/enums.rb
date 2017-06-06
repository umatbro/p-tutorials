class Menu
  include Enumerable
  def each
    yield "pizza"
    yield "spaghetti"
    yield "salad"
    yield "water"
    yield "bread"
  end
end

menu_options = Menu.new

menu_options.each do |item|
  puts "Would you like: #{item}"
end

#check if item is in
p menu_options.find{|item| item = "pizza"}
# get all items that are 5 or less characters long
p menu_options.select{|item| item.size <= 5}
# reject above ones
p menu_options.reject{|item| item.size <= 5}
# first item
p menu_options.first
# first two
p menu_options.take(2)
# will show all but 2 first
p menu_options.drop(2)
# min
p menu_options.min
# max
p menu_options.max

p menu_options.sort
# reverse order
menu_options.reverse_each {|item| puts item}
