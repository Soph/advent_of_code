require "set"

file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

@width = lines[0].length
@height = lines.count

def out_of_bounds?(guard_position)
  return true if guard_position[0] < 0
  return true if guard_position[1] < 0
  return true if guard_position[0] >= @width
  return true if guard_position[1] >= @height

  false
end

antennas = {}
antenna_positions = Set.new

lines.each_with_index do |line, y|
  line.split("").each_with_index do |position, x|
    next if position == '.'

    antennas[position] ||= Set.new
    antennas[position] << [x,y]
    antenna_positions << [x,y]
  end
end

antinodes = Set.new

antennas.each do |antenna, positions|
  positions.each do |position|
    checks = positions.clone.delete(position)
    checks.each do |check_pos|
      dist = [position[0]-check_pos[0], position[1]-check_pos[1]]
      antinode = [position[0]-dist[0]*-1,position[1]-dist[1]*-1]
      antinodes << antinode unless out_of_bounds?(antinode)
    end
  end
end

puts "Part1: #{antinodes.size}"

antinodes = antenna_positions.clone

antennas.each do |antenna, positions|
  positions.each do |position|
    checks = positions.clone.delete(position)
    checks.each do |check_pos|
      dist = [position[0]-check_pos[0], position[1]-check_pos[1]]
      step = 0
      while true do
        step += 1
        antinode = [position[0]-dist[0]*-step,position[1]-dist[1]*-step]
        break if out_of_bounds?(antinode)
        antinodes << antinode   
      end
    end
  end
end

puts "Part2: #{antinodes.size}"

def print_grid(antennas, antinodes)
  (0...@height).each do |y|
    (0...@width).each do |x|
      if antinodes.include?([x,y])
        putc "#"
      else
        putc "."
      end
    end
    puts
  end
end

print_grid(antennas, antinodes)
