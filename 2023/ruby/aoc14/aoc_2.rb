file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)
grid = lines.map(&:chars)

def move(grid, x, y, direction)
  target = [y + direction[0], x + direction[1]]
  return false if target[0] < 0 || target[0] >= grid.size # y out of bounds
  return false if target[1] < 0 || target[1] >= grid[0].size # x out of bounds

  if grid[y][x] == 'O'
    if grid[target[0]][target[1]] == '.'
      grid[target[0]][target[1]] = 'O'
      grid[y][x] = '.'
      return true
    end
  end
  false
end

def calc_weight(grid)
  sum = 0
  grid.each_with_index do |line, y|
    line.each do |rock|
      sum += (grid.size - y) if rock == 'O'
    end
  end
  sum
end

cache = {}
directions = [[-1,0],[0,-1],[1,0],[0,1]]
count = 1000000000
last_weight = 0
last_cache_size = 0
done = false
n = 0
while n < count do
  directions.each_with_index do |direction, dn|
    while true do
      moved = false
      case direction
      when [-1, 0]
        puts "Hallo1"
        (0...grid.size).each do |y|
          (0...grid[0].size).each do |x|
            moved ||= move(grid, x, y, direction)
          end
        end
      when [1, 0]
        puts "Hallo2"
        (0...grid.size).to_a.reverse.each do |y|
          (0...grid[0].size).each do |x|
            moved ||= move(grid, x, y, direction)
          end
        end
      when [0, -1]
        puts "Hallo3"
        (0...grid[0].size).each do |x|
          (0...grid.size).each do |y|
            moved ||= move(grid, x, y, direction)
          end
        end
      when [0, 1]
        puts "Hallo4"
        (0...grid[0].size).to_a.reverse.each do |x|
          (0...grid.size).each do |y|
            moved ||= move(grid, x, y, direction)
          end
        end
      end      
      break unless moved
    end
    puts "next"
  end
  weight = calc_weight(grid)
  if last_weight != weight
    last_weight = weight
  end
  puts weight
  n += 1
end

grid.each do |line|
  puts line.join
end
puts

sum = 0
grid.each_with_index do |line, y|
  line.each do |rock|
    sum += (grid.size - y) if rock == 'O'
  end
end

puts sum