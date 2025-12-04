file = File.open(ARGV[0])
grid = file.readlines.map(&:chomp).map { |l| l.split('') }

candidates = []

width_y = grid.size
width_x = grid[0].size

checks = [[-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1]]

(0...width_y).each do |y|
  (0...width_x).each do |x|
    next if grid[y][x] == '.'

    count = 0
    count_pos = []
    checks.each do |dir|
      new_pos = [x + dir[0], y + dir[1]]
      next if new_pos[0].negative? || new_pos[1].negative?
      next if new_pos[0] >= width_x || new_pos[1] >= width_y

      if grid[new_pos[1]][new_pos[0]] == '@'
        count_pos << new_pos
        count += 1
      end
    end
    # puts "#{[x, y]}: #{count} - #{count_pos}"
    candidates << [x, y] if count < 4
  end
end

puts "Part 1: #{candidates.size}"

positions = Set.new
grid.each_with_index do |row, y|
  row.each_with_index do |roll, x|
    positions << [x, y] if roll == '@'
  end
end
initial_size = positions.size
loop do
  new_positions = Set.new
  positions.each do |pos|
    count = 0
    checks.each do |dir|
      new_pos = [pos[0] + dir[0], pos[1] + dir[1]]

      count += 1 if positions.include?(new_pos)
    end
    new_positions << pos if count >= 4
  end

  break if new_positions.size == positions.size

  positions = new_positions
  # puts positions.size
end

puts "Part 2: #{initial_size - positions.size}"
