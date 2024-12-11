require "set"

file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

data = lines[0].split(" ").map(&:to_i)
stones = {}
data.each do |stone|
  stones[stone] = 1
end

75.times do |i|
  new_stones = Hash.new(0)
  stones.keys.each do |stone|
    if stone == 0
      new_stones[1] += (1 * stones[stone])
    elsif stone.to_s.size % 2 == 0
      new_stones[stone.to_s[0...stone.to_s.size/2].to_i] += stones[stone]
      new_stones[stone.to_s[stone.to_s.size/2..].to_i] += stones[stone]
    else
      new_stones[2024*stone] += stones[stone]
    end
  end
  stones = new_stones
  puts "Part1: #{stones.values.sum}" if i == 24
end

puts "Part2: #{stones.values.sum}"
