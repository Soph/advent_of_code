require "set"

file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)
@width = lines[0].length
@height = lines.count

directions = [[1,0], [0,1], [-1,0], [0,-1]]

walls = Set.new
steps = Set.new
start = nil
finish = nil

lines.each_with_index do |line, y|
  line.split("").each_with_index do |pos, x|
    if pos == "S"
      start = [x,y]
    elsif pos == "E"
      finish = [x,y]
    elsif pos == "#"
      walls << [x,y]
    elsif pos == "."
      steps << [x,y]
    end
  end
end

def out_of_bounds?(position)
  return true if position[0] < 0
  return true if position[1] < 0
  return true if position[0] >= @width
  return true if position[1] >= @height

  false
end

def print_maze(walls, pos)
  puts "pos: #{pos}"
  @height.times do |y|
    @width.times do |x|
      if walls.include?([x,y])
        putc "#"
      elsif pos == [x,y]
        putc "@"
      else
        putc "."
      end
    end
    puts
  end
  sleep 1
end

def find_path(start, exit, walls)
  directions = [[1,0], [0,1], [-1,0], [0,-1]]
  visited = {start => 0}
  paths = {start => []}
  to_check = [start]
  while !to_check.empty?
    pos = to_check.sort_by{|pos| visited[pos] }.first
    to_check.delete(pos)
    cost = visited[pos]
    path = paths[pos]
    directions.each do |dir|
      new_pos = [pos[0]+dir[0], pos[1]+dir[1]]
      next if out_of_bounds?(new_pos)
      next if walls.include?(new_pos)

      if new_pos == exit
        if visited[new_pos].nil?
          visited[new_pos] = cost + 1
          paths[new_pos] = path + [new_pos]
        elsif visited[new_pos] > cost + 1
          visited[new_pos] = cost + 1
          paths[new_pos] = path + [new_pos]
        end
      end

      if visited[new_pos].nil? || visited[new_pos] > cost + 1
        visited[new_pos] = cost + 1
        paths[new_pos] = path + [new_pos]
        to_check << new_pos
      end
    end
  end
  paths[exit]
end


directions = [[1,0], [0,1], [-1,0], [0,-1]]
distances = {}
to_check = [finish]
i = 1
while !to_check.empty?
  new_to_check = Set.new
  to_check.each do |pos|
    directions.each do |dir|
      new_pos = [pos[0]+dir[0], pos[1]+dir[1]]
      next if out_of_bounds?(new_pos)
      next if walls.include?(new_pos)
      next if distances.include?(new_pos)
      
      distances[new_pos] = i if distances[new_pos].nil?
      new_to_check << new_pos
    end
  end
  i += 1
  to_check = new_to_check
end

puts distances.keys.size

path = find_path(start,finish,walls)
length = path.size
path.insert(0, start)
   #
  # #   
 # s #   
  # #
   #    
cache = {}
checks = [[-2,0], [2,0], [0,-2], [0, 2], [1, 1], [1, -1], [-1, 1], [-1, -1]]

10.times do |i|
  puts "#{path[i]}: #{distances[path[i]]}"
end

def distance(a, b)
  [(a[0] - b[0]).abs,(a[1] - b[1]).abs].max
end

cheat_positions = Set.new
safes = {}
path.each_with_index do |pos, i|
  next if pos == finish
  checks.each do |check|
    new_pos = [pos[0]+check[0],pos[1]+check[1]]
    next if walls.include?(new_pos) # must be back on a track
    next if out_of_bounds?(new_pos)
    next if distances[new_pos]+2 >= distances[pos] # is farer away then next step
    #puts "#{pos} -> #{new_pos} vs #{path[i+1]}"
    #require "pry"; binding.pry
    safe = distances[pos] - (distances[new_pos] + 2)
    #puts "#{distances[pos]} - #{distances[new_pos]+2} -> #{safe}"
    safes[safe] ||= 0
    safes[safe] += 1
    cheat_positions << [pos, new_pos] if safe >= 100
  end
end

puts cheat_positions.size
puts safes.inspect
puts safes.select{|key,value| key >= 100}.values.sum