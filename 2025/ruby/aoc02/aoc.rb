require "set"
file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

ranges = lines[0].split(",")
invalid = Set.new
ranges.each do |range|
  parsed = range.split("-")
  (parsed[0].to_i..parsed[1].to_i).each do |number|
    invalid << number.to_i if number.to_s =~ /^([0-9]+)\1$/
  end
end

puts invalid.inspect
puts invalid.sum

ranges = lines[0].split(",")
invalid = Set.new
ranges.each do |range|
  parsed = range.split("-")
  (parsed[0].to_i..parsed[1].to_i).each do |number|
    invalid << number.to_i if number.to_s =~ /^([0-9]+)\1+$/
  end
end

puts invalid.inspect
puts invalid.sum
