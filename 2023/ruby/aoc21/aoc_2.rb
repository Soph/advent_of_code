require "Set"

file = File.open(ARGV[0])
grid = file.readlines.map(&:chomp).map(&:chars)

starting = nil
count_rocks = 0
grid.each_with_index do |line, y|
  line.each_with_index do |item, x|
    if item == 'S'
      starting = [x,y]
    end
    if item == '#'
      count_rocks += 1
    end
  end
end

positions = Set.new
positions << starting

directions = [[0,1], [1,0], [0,-1], [-1,0]]

100.times do |i|
  #puts positions.inspect
  new_positions = Set.new
  positions.each do |pos|
    directions.each do |d|
      new_pos = [pos[0] + d[0], pos[1] + d[1]]
      next if grid[new_pos[1]%grid.size][new_pos[0]%grid[0].size] == '#'

      new_positions << new_pos
    end
  end
  puts "#{i}: #{new_positions.size}" if (i - 65) % 131 == 0
  #puts "#{i}: #{new_positions.size - positions.size}"
  positions = new_positions
  count = 0
  # (-grid.size*1...grid.size*2).each do |y|
  #   (-grid[0].size*1...grid[0].size*2).each do |x|
  #     if positions.include?([x,y])
  #       #putc "O"
  #       count += 1
  #     else
  #       #putc grid[y%grid.size][x%grid[0].size]
  #     end
  #   end
  #   #puts
  # end
  #puts "Iteration Count: #{count}"
end

(-grid.size*5...grid.size*6).each do |y|
  (-grid[0].size*10...grid[0].size*10).each do |x|
    if positions.include?([x,y])
      putc "O"
      #count += 1
    else
      putc grid[y%grid.size][x%grid[0].size]
    end
  end
  puts
end

puts "Part1: #{positions.size}"