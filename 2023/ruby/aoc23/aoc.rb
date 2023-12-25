require "Set"

file = File.open(ARGV[0])
grid = file.readlines.map(&:chomp).map(&:chars)

start = [1,0]
end_pos = [grid[0].size-2, grid.size-1]
puts "End Pos: #{end_pos.inspect}"
directions = [[0,1], [1,0], [0,-1], [-1,0]]
arrows = ["v", ">", "^", "<"]

def print_grid(grid, path)
  grid.each_with_index do |line,y|
    line.each_with_index do |pos,x|
      if path.include?([x,y])
        putc "O"
      else
        putc pos
      end
    end
    puts
  end
end

paths = [[[1,0]]]
finished = []
i = 0
while true
  i += 1
  break if paths.empty?
  path = paths.shift
  #puts path.inspect
  pos = path[-1]
  directions.each do |dx,dy|
    new_pos = [pos[0]+dx, pos[1]+dy]
    next if new_pos[0] < 0 || new_pos[0] >= grid[0].size
    next if new_pos[1] < 0 || new_pos[1] >= grid.size
    next if grid[new_pos[1]][new_pos[0]] == '#'
    next if path[-2] == new_pos # no backward
    next if path.include?(new_pos)

    if new_pos == end_pos
      path << new_pos
      finished << path
    end
    
    path_add = [new_pos]
    # slippery slope, add next step
    if grid[new_pos[1]][new_pos[0]] != "."
      dx2,dy2 = directions[arrows.index(grid[new_pos[1]][new_pos[0]])]
      #puts "Move: #{[dx,dy]} #{[dx2,dy2]}"
      if [dx+dx2, dy+dy2] == [0,0] # no walking up a slope
        #puts "Skipping, can't walk up a slope"
        next
      end
      path_add << [new_pos[0]+dx2,new_pos[1]+dy2]
    end
    new_path = path.map(&:clone)
    new_path += path_add
    # if path_add.include?([12, 13])
    #   print_grid(grid, new_path)
    #   sleep 10
    # end    
    paths << new_path
  end
  puts paths.size if i % 1000 == 0
end

#puts finished.inspect
longest = finished.sort_by(&:size).last
puts longest.inspect
puts "Part 1: #{longest.size - 1}"

#print_grid(grid, longest)