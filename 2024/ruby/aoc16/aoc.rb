require "set"

file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)
@width = lines[0].length
@height = lines.count

directions = [[1,0], [0,1], [-1,0], [0,-1]]

walls = Set.new
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
    end
  end
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

# [cost, steps]
path = [0, [[]]]
visited = {}
visited[[start, 0]] = path
to_check = Set.new
to_check << [start, 0]
lowest_finish = (2**(0.size * 8 -2) -1)
lowest_finisher = []
while !to_check.empty? do
  pos = to_check.sort_by{ |pos| visited[pos][0] }.first
  to_check.delete(pos)
  path = visited[pos]

  dir = directions[pos[1]]
  # first left / right
  left_right = [(pos[1] + 1) % directions.size, (pos[1] - 1) % directions.size]
  left_right.each do |new_dir_index|
    if visited[[pos[0], new_dir_index]].nil? || visited[[pos[0], new_dir_index]][0] > (visited[pos][0] + 1000)
      visited[[pos[0], new_dir_index]] = [visited[pos][0] + 1000, visited[pos][1]]
      to_check << [pos[0], new_dir_index]
    elsif visited[[pos[0], new_dir_index]][0] == (visited[pos][0] + 1000)
      visited[[pos[0], new_dir_index]][1] += visited[pos][1]
      to_check << [pos[0], new_dir_index]
    end
  end

  # next step
  new_pos = [pos[0][0] + dir[0], pos[0][1] + dir[1]]
  next if walls.include?(new_pos)
  if new_pos == finish
    if path[0] + 1 < lowest_finish
      lowest_finish = path[0] + 1
      lowest_finisher = path[1]
    elsif path[0] + 1 == lowest_finish
      lowest_finisher += path[1]
    end
  else
    if visited[[new_pos, pos[1]]].nil? || visited[[new_pos, pos[1]]][0] > (visited[pos][0] + 1)
      visited[[new_pos, pos[1]]] = [visited[pos][0] + 1, visited[pos][1].map{|raw_path| raw_path + [new_pos]}]
      to_check << [new_pos, pos[1]]
    elsif visited[[new_pos, pos[1]]][0] == (visited[pos][0] + 1)
      visited[[new_pos, pos[1]]][1] += visited[pos][1].map{|raw_path| raw_path + [new_pos]}
      to_check << [new_pos, pos[1]]
    end
  end
end

puts "Part1: #{lowest_finish}"
puts "Part2: #{lowest_finisher.reduce(:concat).uniq.size + 2}" # + start + end
