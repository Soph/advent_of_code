require "set"

file = File.open(ARGV[0])
data = file.readlines.map(&:chomp).join("\n").split("\n\n")

locks = []
keys = []

def parse(item)
  parsed = Set.new
  item.split("\n").each_with_index do |line,y|
    line.split("").each_with_index do |pos,x|
      if pos == '#'
        parsed << [x,y]
      end
    end
  end
  parsed
end

data.each do |item|
  parsed = parse(item)
  if parsed.include?([0,0])
    locks << parsed
  else
    keys << parsed
  end
end

valid = 0
locks.each do |lock|
  keys.each do |key|
    if lock.to_a & key.to_a == []
      valid += 1
    end
  end
end

puts "Part1: #{valid}"