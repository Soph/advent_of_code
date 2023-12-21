require "Set"

file = File.open(ARGV[0])
grid = file.readlines.map(&:chomp).map(&:chars)

starting = nil
grid.each_with_index do |line, y|
  line.each_with_index do |item, x|
    if item == 'S'
      starting = [x,y]
      break
    end
  end
end

positions = Set.new
positions << starting

directions = [[0,1], [1,0], [0,-1], [-1,0]]

64.times do |i|
  puts positions.inspect
  new_positions = Set.new
  positions.each do |pos|
    directions.each do |d|
      new_pos = [pos[0] + d[0], pos[1] + d[1]]
      next unless new_pos[0] >= 0 && new_pos[0] < grid[0].size
      next unless new_pos[1] >= 0 && new_pos[1] < grid.size
      next if grid[new_pos[1]][new_pos[0]] == '#'

      new_positions << new_pos
    end
  end
  positions = new_positions
end

puts "Part1: #{positions.size}"