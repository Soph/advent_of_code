file = File.open(ARGV[0])
coordinates = file.readlines.map(&:chomp).map { |l| l.split(',').map(&:to_i) }

max = 0
coordinates.combination(2).each do |pair|
  size = ((pair[0][0] - pair[1][0]).abs + 1) * ((pair[0][1] - pair[1][1]).abs + 1)
  max = size if size > max
end

puts "Part 1: #{max}"

edges = Hash.new(Set.new)
path = Set.new
min_x = 1_000_000
max_x = 0
min_y = 1_000_000
max_y = 0
(-1..coordinates.size - 2).each do |i|
  start = coordinates[i]
  finish = coordinates[i + 1]
  if start[0] == finish[0]
    ends = [start[1], finish[1]].sort
    min_y = ends[0] if min_y > ends[0]
    max_y = ends[1] if max_y < ends[1]
    (ends[0]..ends[1]).each do |y|
      path << [start[0], y]
      edges[y] += [[start[0], start[0]]]
    end
  else
    ends = [start[0], finish[0]].sort
    min_x = ends[0] if min_x > ends[0]
    max_x = ends[1] if max_x < ends[1]
    (ends[0]..ends[1]).each do |x|
      path << [x, start[1]]
    end
    edges[start[1]] += [[ends[0], ends[1]]]
  end
end
# raycast
puts "#{[min_x, min_y]} -> #{[max_x, max_y]}"
# grid = path.dup
# (min_x..max_x).each do |x|
#  (min_y..max_y).each do |y|
#    next if grid.include?([x, y])
#
#    same_y = path.select { |pos| pos[1] == y }
#    to_the_right = same_y.select { |pos| pos[0] > x }
#    next if same_y.size == to_the_right.size # before grid
#
#    grid << [x, y] if to_the_right.size.odd?
#  end
# #end

# puts edges.inspect

# compact edges
edges.size.times do |i|
  loop do
    reduced = Set.new
    edges[i].each do |range|
      next if edges[i].any? { |r| r != range && r[0] <= range[0] && r[1] >= range[1] }

      reduced << range
    end
    break if reduced.size == edges[i].size

    edges[i] = reduced
  end
end

# puts edges.inspect

def inside?(edges, position)
  #  same_y = path.select { |pos| pos[1] == position[1] }
  to_the_right = edges[position[1]].select { |x| x[1] > position[0] }
  # puts "#{position}: #{edges[position[1]]} #{to_the_right}"
  return false if edges[position[1]].size == to_the_right.size # before grid

  to_the_right.size.odd?
end

def border(pair)
  path = Set.new
  coordinates = [pair[0], [pair[0][0], pair[1][1]], pair[1], [pair[1][0], pair[0][1]]]
  (-1..2).each do |i|
    start = coordinates[i]
    finish = coordinates[i + 1]
    if start[0] == finish[0]
      ends = [start[1], finish[1]].sort
      (ends[0]..ends[1]).each do |y|
        path << [start[0], y]
      end
    else
      ends = [start[0], finish[0]].sort
      (ends[0]..ends[1]).each do |x|
        path << [x, start[1]]
      end
    end
  end
  path
end

max = 0
max_coordinates = nil
candidates = {}
i = 0
coordinates.combination(2).each do |pair|
  # next check if there are no edges inside the rectangle either
  next unless border(pair).all? { |pos| path.include?(pos) || inside?(edges, pos) }

  size = ((pair[0][0] - pair[1][0]).abs + 1) * ((pair[0][1] - pair[1][1]).abs + 1)
  candidates[size] ||= []
  candidates[size] << pair
  if size > max
    max = size
    max_coordinates = pair
  end
  i += 1
  puts i
end
puts max_coordinates.inspect
puts "Part 2: #{max}"
exit
candidates.keys.sort.reverse.each do |size|
  candidates[size].each do |pair|
    valid = true
    (pair[0][1]..pair[1][1]).each do |y|
      valid = false if edges[y].any? do |edge|
        (edge[0] >= pair[0][0] && edge[0] <= pair[1][0]) || (edge[1] >= pair[0][0] && edge[1] <= pair[1][0])
      end
    end
    (pair[0][]..pair[1][1]).each do |y|
      valid = false if edges[y].any? do |edge|
        (edge[0] >= pair[0][0] && edge[0] <= pair[1][0]) || (edge[1] >= pair[0][0] && edge[1] <= pair[1][0])
      end
    end

    puts "Part2: #{size}" if valid
  end
end

exit
# [15822, 84037], [83178, 15240]
(15_240..84_037).each do |y|
  (15_822..83_178).each do |x|
    if path.include?([x, y])
      putc '#'
    # elsif grid.include?([x, y])
    #  putc 'o'
    else
      putc '.'
    end
  end
  puts
end
