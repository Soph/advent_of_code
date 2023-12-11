require "Set"

file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

expanded = []
empty_rows = (0..lines[0].size-1).to_a
lines.each do |line|
  expanded << line if line.chars.tally.size == 1
  expanded << line
  line.chars.each_with_index do |point, i|
    empty_rows.delete(i) if point == '#'
  end
end
image = []
galaxies = Set.new
expanded.each_with_index do |line, y|
  points = line.chars
  empty_rows.reverse.each do |i|
    points.insert(i, '.')
  end
  image << points
  points.each_with_index do |point, x|
    galaxies << [x,y] if point == '#'
  end
end
#puts galaxies.inspect

# image.each do |line|
#   puts line.join
# end

#puts galaxies.inspect

paths = {}
sum = 0
galaxies.each do |galaxy_start|
  galaxies.each do |galaxy_end|
    next if galaxy_start == galaxy_end
    key = [galaxy_start, galaxy_end].sort
    next unless paths[key].nil?

    # manhatten distance
    distance = (galaxy_start[0] - galaxy_end[0]).abs + (galaxy_start[1] - galaxy_end[1]).abs
    sum += distance
    paths[key] = distance
  end
end

puts sum
