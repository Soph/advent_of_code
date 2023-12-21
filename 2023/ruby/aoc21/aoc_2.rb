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
#puts count_rocks

directions = [[0,1], [1,0], [0,-1], [-1,0]]

startings = [
  [grid.size/2,grid.size/2], [grid.size/2,0], [0,grid.size/2], [grid.size-1,grid.size/2], [grid.size/2,grid.size-1],
  [0,0], [0,grid.size-1], [grid.size-1,0], [grid.size-1, grid.size-1]
]
# startings.each do |starting|
#   transitions = Set.new
#   positions = Set.new
#   positions << starting
#   stable = nil
#   stable_count = 0
#   i = 0
#   while true
#     i += 1
#     new_positions = Set.new
#     positions.each do |pos|
#       directions.each do |d|
#         new_pos = [pos[0] + d[0], pos[1] + d[1]]
#         next unless new_pos[0] >= 0 && new_pos[0] < grid[0].size
#         next unless new_pos[1] >= 0 && new_pos[1] < grid.size
#         next if grid[new_pos[1]][new_pos[0]] == '#'

#         new_positions << new_pos
#       end
#     end
#     if transitions.include?([positions.size, new_positions.size])
#       stable = [positions.size, new_positions.size]
#       stable_count = i
#       break
#     else
#       transitions << [positions.size, new_positions.size]
#     end
#     positions = new_positions
#   end
#   puts "#{starting}: #{stable.inspect} - #{stable_count}"
# end

# after 66 steps it reaches n,s,w,e
# after 132 more steps it reaches the next n,s,w,e
# it takes 197 to become stable for any block after the first
# 
positions = Set.new
positions << starting

def calc_state(grid, start, steps, restrict, print = false)
  directions = [[0,1], [1,0], [0,-1], [-1,0]] 
  positions = Set.new
  positions << start
  steps.times do |i|
    #puts positions.inspect
    new_positions = Set.new
    positions.each do |pos|
      directions.each do |d|
        new_pos = [pos[0] + d[0], pos[1] + d[1]]
        next if grid[new_pos[1]%grid.size][new_pos[0]%grid[0].size] == '#'
  
        new_positions << new_pos
      end
    end
    positions = new_positions
  end
  if restrict
    positions = positions.select do |pos|
      pos[0] >= 0 && pos[0] < grid.size && pos[1] >= 0 && pos[1] < grid.size
    end
  end
  if print
    x_values = positions.map{|pos| pos[0]}.sort
    y_values = positions.map{|pos| pos[1]}.sort
    (y_values[0]..y_values[-1]).each do |y|
      (x_values[0]..x_values[-1]).each do |x|
        if positions.include?([x,y])
          putc "O"
          #count += 1
        else
          putc grid[y%grid.size][x%grid[0].size]
        end
      end
      puts
    end
  end
  positions
end

startings = [
  [grid.size/2,grid.size/2], [grid.size/2,0], [0,grid.size/2], [grid.size-1,grid.size/2], [grid.size/2,grid.size-1],
  [0,0], [0,grid.size-1], [grid.size-1,0], [grid.size-1, grid.size-1]
]


position = calc_state(grid, [grid.size/2, grid.size/2], grid.size * 2 + grid.size / 2, false, false)

pieces << position.select {|pos| pos[0] < 0 && pos[1] < -grid.size/2} #top top left
pieces << position.select {|pos| pos[0] < 0 && pos[0] >= -grid.size && pos[1] >= -grid.size/2 && pos[1] < 0} #top left
pieces << position.select {|pos| pos[0] < -grid.size && pos[1] > -grid.size/2 && pos[1] < 0} #top left left

pieces << position.select {|pos| pos[0] < -grid.size && pos[1] > grid.size/2} # bottom left left
pieces << position.select {|pos| pos[0] < 0 && pos[0] >= -grid.size && pos[1] > grid.size/2 && pos[1] < 2*grid.size} #bottom left
pieces << position.select {|pos| pos[0] < 0 && pos[1] > 2*grid.size} #top left left


corners << calc_state(grid, [grid.size/2, grid.size-1], 65, true, false).size # top
corners << calc_state(grid, [0, grid.size/2], 65, true).size # right
corners << calc_state(grid, [grid.size-1, grid.size/2], 65, true).size # left
corners << calc_state(grid, [grid.size/2, 0], 65, true).size # bottom


corners = []

corners << calc_state(grid, [0, grid.size-1], 65, true, false).size # top right
corners << calc_state(grid, [0, 0], 65, true).size # bottom right
corners << calc_state(grid, [grid.size-1, 0], 65, true).size # bottom left
corners << calc_state(grid, [grid.size-1, grid.size-1], 65, true).size # top left

corners << calc_state(grid, [grid.size/2, grid.size/2], 131, true).size

puts corners.inspect

puts corners[0] + corners[2] + corners[4] + corners[6] + corners[-1]
# puts corners.sum
puts calc_state(grid, [65,65], 131, false, false).size
#puts calc_state(grid, [65,65], 65+131, false, true)
exit

exit

#3762
#33547

last_count = 0
500.times do |i|
  #puts positions.inspect
  new_positions = Set.new
  positions.each do |pos|
    directions.each do |d|
      new_pos = [pos[0] + d[0], pos[1] + d[1]]
      next if grid[new_pos[1]%grid.size][new_pos[0]%grid[0].size] == '#'

      new_positions << new_pos
    end
  end
  if (i - 5) % 11 == 0
    puts "#{i}: #{new_positions.size - last_count}"
    last_count = new_positions.size
  end
  positions = new_positions
  count = 0
  # if positions.select{|pos| pos[0] >= 261}.size > 0
  #   (-grid.size*1...grid.size*2).each do |y|
  #     (-grid[0].size*1...grid[0].size*2).each do |x|
  #       if positions.include?([x,y])
  #         putc "O"
  #         count += 1
  #       else
  #         putc grid[y%grid.size][x%grid[0].size]
  #       end
  #     end
  #     puts
  #   end
  #   puts "Iteration Count: #{count}"
  #   exit
  # end
end

# (-grid.size*10...grid.size*11).each do |y|
#   (-grid[0].size*10...grid[0].size*11).each do |x|
#     if positions.include?([x,y])
#       putc "O"
#       #count += 1
#     else
#       putc grid[y%grid.size][x%grid[0].size]
#     end
#   end
#   puts
# end

puts "Part1: #{positions.size}"


#608152828731262
#608151312489000