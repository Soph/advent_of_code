require "pry"
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
finished = Set.new
i = 0
sub_paths = Set.new
while true
  i += 1
  break if paths.empty?
  path = paths.shift
  puts paths.inspect if path.nil?
  pos = path[-1]
  found = 0
  new_paths = []
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
    new_path = path.map(&:clone)
    new_path += path_add
    new_paths << new_path
  end
  if new_paths.size > 1
    puts "crossing #{path.inspect}"
    # crossing
    unless sub_paths.include?(path)
      sub_paths << path 
      new_paths.each do |new_path|
        puts "New Start: #{new_path[-2..-1]}"
        paths << new_path[-2..-1]
      end
    end
  elsif new_paths.size > 0
    paths << new_paths[0]
  end
  puts sub_paths.size
  puts paths.size if i % 1000 == 0
end

sub_paths.each do |path|
  puts "#{path[0]} - #{path[1]} : #{path.size}"
end
#binding.pry
options = [finished.first]
solutions = []
i = 0
while true
  i += 1
  break if options.empty?
  option = options.shift
  if option[0] == start
    #puts "Found full path"
    solutions << option
  else
    next_paths = sub_paths.select {|path| path[-1] == option[0]}
    #puts "Next Options: #{next_paths.size}"
    #binding.pry if next_paths.size == 4
    next_paths.each do |path|
      next_path = path + option[1..]
      if next_path.uniq.size == next_path.size
        options << next_path
      else
        #puts "skipping, not uniq"
      end
    end
  end
  #options.uniq!
  puts options.size if i % 100 == 0
  #puts "#{options.sort_by{|option| option.size}.min} - #{options.sort_by{|option| option.size}.max}"
end

puts solutions.inspect
binding.pry
#puts finished.inspect
longest = finished.sort_by(&:size).last
puts longest.inspect
puts longest.size - 1

puts "Part 2: #{longest.size - 1}"
print_grid(grid, longest)