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
puts "#{[min_x, min_y]} -> #{[max_x, max_y]}"

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

max = 0
max_coordinates = nil
candidates = {}
i = 0
coordinates.combination(2).each do |pair|
  corners = [pair[0], pair[1], [pair[0][0], pair[1][1]], [pair[1][0], pair[0][1]]]
  next unless corners.all? { |c| path.include?(c) || inside?(edges, c) }

  # next check if there are no edges inside the rectangle either
  valid = true
  y_range = [pair[0][1], pair[1][1]].sort
  x_range = [pair[0][0], pair[1][0]].sort
  (y_range[0]..y_range[1]).each do |y|
    valid = false if edges[y].any? do |edge|
      (edge[0] > x_range[0] && edge[0] < x_range[1]) || (edge[1] > x_range[0] && edge[1] < x_range[1])
    end
  end
  next unless valid

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
