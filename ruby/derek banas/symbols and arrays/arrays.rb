array_1 = Array.new
array_2 = Array.new(5)
array_3 = Array.new(5, "empty")
array_4 = [1,"two", 3, 5.5]


puts array_4.values_at(0,1,3).join(", ");
array_4.unshift(0)    # add item to the begenning of the array
array_4.shift()       # remove the first element of the array

array_4.push(100,200) # will add 100 and 200 to the end of the array
array_4.pop()         # will remove last item in the array

array_4.concat([10,20,30])

puts "Array size: " + array_4.size.to_s
puts "Array contains 100: " + array_4.include?(100).to_s
puts "How may 100s?: " + array_4.count(100).to_s
puts "Array empty: " + array_4.empty?.to_s

puts array_4.join(", ")

p array_4
array_4.each do |value|
  puts value
end
