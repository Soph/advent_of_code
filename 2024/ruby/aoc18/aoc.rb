require "set"

file = File.open(ARGV[0])
corrupted = file.readlines.map(&:chomp).map{|line| line.split(",").map(&:to_i)}

@width = 71
@height = 71
start = [0,0]
max = 1024
if ARGV[0] =~ /test/
  @width = 7
  @height = 7
  max = 12
end
exit = [@width-1, @height-1]

def out_of_bounds?(position)
  return true if position[0] < 0
  return true if position[1] < 0
  return true if position[0] >= @width
  return true if position[1] >= @height

  false
end

def print_grid(corrupted, path, special)
  @height.times do |y|
    @width.times do |x|
      if special == [x,y]
        putc "X"
      elsif corrupted.include?([x,y]) && path.include?([x,y])
        putc "@"
      elsif corrupted.include?([x,y])
        putc "#"
      elsif path.include?([x,y])
        putc "O"
      else
        putc "."
      end
    end
    puts
  end
end

walls = corrupted[0...max]

def find_path(start, exit, walls)
  directions = [[1,0], [0,1], [-1,0], [0,-1]]
  visited = {}
  to_check = [start]
  while !to_check.empty?
    pos = to_check.sort_by{|pos| visited[pos]}.first
    to_check.delete(pos)
    cost, path = visited[pos]
    cost ||= 0
    path ||= []

    directions.each do |dir|
      new_pos = [pos[0]+dir[0], pos[1]+dir[1]]
      next if out_of_bounds?(new_pos)
      next if walls.include?(new_pos)
      next if path.include?(new_pos)

      if new_pos == exit
        if visited[new_pos].nil?
          visited[new_pos] = [cost + 1, path + [new_pos]]
        elsif visited[new_pos][0] > cost + 1
          visited[new_pos] = [cost + 1, path + [new_pos]]
        end
      end

      if visited[new_pos].nil? || visited[new_pos][0] > cost + 1
        visited[new_pos] = [cost + 1, path + [new_pos]]
        to_check << new_pos
        #print_grid(walls, new_pos)
        #puts
      end
    end
  end
  visited[exit]
end

result_1 = find_path(start, exit, walls)
puts "Part1: #{result_1[0]}"

path = result_1[1]
corrupted[max..].each_with_index do |cor, i|
  index = path.index(cor)
  if !index.nil?
    puts "#{cor} - #{corrupted[i+max-5..i+max]}"
    new_path = find_path(path[index-1], path[index+1], corrupted[0..i+max])
    if new_path.nil?
      print_grid(corrupted[0..i+max], path, cor)
      puts "Part2: #{cor}"
      break
    end
  end
end
