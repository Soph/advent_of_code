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
  visited = Set.new
  to_check = [start]
  i = 0
  while !to_check.empty?
    new_to_check = Set.new
    to_check.each do |pos|
      directions.each do |dir|
        new_pos = [pos[0]+dir[0], pos[1]+dir[1]]
        next if out_of_bounds?(new_pos)
        next if walls.include?(new_pos)
        next if visited.include?(new_pos)
        
        visited << new_pos
        new_to_check << new_pos
      end
    end
    i += 1
    to_check = new_to_check
    return i if to_check.include?(exit)
  end
end

result_1 = find_path(start, exit, walls.to_set)
puts "Part1: #{result_1}"

corrupted[max..].each_with_index do |cor, i|
  result = find_path(start, exit, corrupted[0..i+max].to_set)
  if result.nil?
    puts "Part2: #{cor[0]},#{cor[1]}"
    break
  end
end
