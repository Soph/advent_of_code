require "set"
file = File.open(ARGV[0])
instructions = file.readlines.map(&:chomp)

HOLES = {}
HOLES[[0,0]] = ''

movements = {
  "R" => [1,0],
  "U" => [0,-1],
  "L" => [-1,0],
  "D" => [0,1],
}

pos = 0,0

instructions.each do |instruction|
  parts = instruction.split(" ")
  direction = movements[parts[0]]
  (0...parts[1].to_i).each do |move|
    pos = [pos[0]+direction[0], pos[1]+direction[1]]
    HOLES[pos] = parts[2].gsub(/[\(\)]/,'')
  end
end

sorted_x = HOLES.keys.sort_by{|hole| hole[0]}
MIN_X = sorted_x.first[0] - 1
MAX_X = sorted_x.last[0] + 1
sorted_y = HOLES.keys.sort_by{|hole| hole[1]}
MIN_Y = sorted_y.first[1] - 1
MAX_Y = sorted_y.last[1] + 1

EMPTY = Set.new
to_check = Set.new
to_check << [MIN_X,MIN_Y]
while !to_check.empty?
  pos = to_check.to_a.pop
  to_check.delete(pos)
  next if pos[0] < MIN_X || pos[0] > MAX_X
  next if pos[1] < MIN_Y || pos[1] > MAX_Y
  next if EMPTY.include?(pos)

  if HOLES[pos].nil?
    EMPTY << pos
    to_check << [pos[0], pos[1] + 1]
    to_check << [pos[0], pos[1] - 1]
    to_check << [pos[0] + 1, pos[1]]
    to_check << [pos[0] - 1, pos[1]]
  end
end

puts EMPTY.size
grid_size =  ((MAX_X-MIN_X)+1) * ((MAX_Y-MIN_Y)+1)
puts "Part1: #{grid_size-EMPTY.size}"


# sum = 0
# (MIN_Y...MAX_Y).each do |y|
#   (MIN_X...MAX_X).each do |x|
#     if HOLES[[x,y]]
#       putc "#"
#     else
#       putc "."
#     end
#   end
#   sum += (MIN_X...MAX_X).size - EMPTY.select{|pos| pos[1] == y}.size + 1
#   puts " - #{y}: #{((MIN_X...MAX_X).size - EMPTY.select{|pos| pos[1] == y}.size) + 1} - #{sum}"
# end
