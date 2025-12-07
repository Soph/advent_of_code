file = File.open(ARGV[0])
grid = file.readlines.map(&:chomp).map { |l| l.split('') }

max_y = grid.size
max_x = grid[0].size
start = [0, 0]
splitter = Set.new
grid.each_with_index do |row, y|
  row.each_with_index do |pos, x|
    start = [x, y] if pos == 'S'
    splitter << [x, y] if pos == '^'
  end
end

next_rays = []
next_rays << [start[0], start[1] + 1]

done = Set.new
split_count = 0
loop do
  ray = next_rays.pop
  break if ray.nil?
  break if done.include?(ray)

  done << ray
  next if ray[1] + 1 >= max_y

  next_pos = [ray[0], ray[1] + 1]
  if splitter.include?(next_pos)
    next_rays << [next_pos[0] - 1, next_pos[1]] if next_pos[0] - 1 >= 0 && !done.include?([next_pos[0] - 1,
                                                                                           next_pos[1]])
    next_rays << [next_pos[0] + 1, next_pos[1]] if next_pos[0] + 1 < max_x && !done.include?([next_pos[0] + 1,
                                                                                              next_pos[1]])
    split_count += 1
  else
    next_rays << next_pos unless done.include?(next_pos)
  end
end
puts "Part 1: #{split_count}"

positions = {}
positions[[start[0], start[1]]] = 1

tocheck = Set.new
tocheck << [start[0], start[1]]

loop do
  pos = tocheck.take(1).first
  break if pos.nil?

  tocheck.subtract([pos])
  # puts tocheck.inspect
  next if pos[1] + 1 >= max_y

  next_pos = [pos[0], pos[1] + 1]
  next_rays = []
  if splitter.include?(next_pos)
    next_rays << [next_pos[0] - 1, next_pos[1]] if next_pos[0] - 1 >= 0
    next_rays << [next_pos[0] + 1, next_pos[1]] if next_pos[0] + 1 < max_x
    next_rays.each do |new_pos|
      positions[new_pos] ||= 0
      positions[new_pos] += positions[pos]
      tocheck << new_pos
    end
  else
    tocheck << next_pos
    positions[next_pos] ||= 0
    positions[next_pos] += positions[pos]
  end
  # puts positions.inspect
  # sleep 5
  break if tocheck.empty?
end

puts "Part 2: #{positions.keys.select { |key| key[1] == (max_y - 1) }.map { |key| positions[key] }.sum}"
