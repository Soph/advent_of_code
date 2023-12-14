file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

grid = lines.map(&:chars)

while true do
  moved = false
  (1...grid.size).each do |y|
    (0...grid[0].size).each do |x|
      if grid[y][x] == 'O'
        if grid[y-1][x] == '.'
          moved = true
          grid[y-1][x] = 'O'
          grid[y][x] = '.'
        end
      end
    end
  end
  break unless moved
end

grid.each do |line|
  puts line.join
end

sum = 0
grid.each_with_index do |line, y|
  line.each do |rock|
    sum += (grid.size - y) if rock == 'O'
  end
end

puts sum