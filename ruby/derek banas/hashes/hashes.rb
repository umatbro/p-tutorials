# hashes are collections of key-value pairs

number_hash = {
  "PI" => 3.14,
  "Golden" => 1.618,
  "e" => 2.781}

puts number_hash["PI"]
superheroes = Hash[
  "Clark Kent",
  "Superman",
  "Bruce Wayne",
  "Batman"]

puts superheroes["Clark Kent"]
superheroes["Barry Allen"] = "Flash"

samp_hash = Hash.new("No such key")

superheroines = Hash[
  "Lisa Morel" => "Aquagirl",
  "Betty Kane" => "Batgirl"
]

# also look up .merge method
superheroes.update(superheroines)

superheroes.each do |keyy, value|
  puts keyy.to_s + ": " + value
end

puts "Has Key Lisa Morel: " + superheroes.has_key?("Lisa Morel").to_s
puts "Has value Batman: " + superheroes.has_value?("Batman").to_s
puts "Is hash empty: " + superheroes.empty?.to_s
puts "Size of hash: " + superheroes.size.to_s

puts "Deleting Barry Allen"
superheroes.delete("Barry Allen")

puts "Size of hash: " + superheroes.size.to_s
