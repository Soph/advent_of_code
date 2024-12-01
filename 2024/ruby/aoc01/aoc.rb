file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

list_left = []
list_right = []

lines.each do |line|
  numbers = line.split(" ")
  list_left << numbers[0].to_i
  list_right << numbers[1].to_i
end

list_left.sort!
list_right.sort!

distance = list_left.map.with_index do |value, index|
  (value - list_right[index]).abs
end

puts "Part1: #{distance.sum}"

times = list_left.map do |value|
  value * list_right.count(value)
end

puts "Part2: #{times.sum}"