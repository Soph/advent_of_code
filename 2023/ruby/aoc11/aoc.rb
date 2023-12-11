require "Set"

file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

modifier = 1000000

expanded = []
empty_columns = (0..lines[0].size-1).to_a
empty_rows = []
lines.each_with_index do |line, y|
  if line.chars.tally.size == 1
    empty_rows << y
  end
  line.chars.each_with_index do |point, x|
    empty_columns.delete(x) if point == '#'
  end
end
image = []
galaxies = Set.new
lines.each_with_index do |line, y|
  points = line.chars
  # shift y
  actual_y = y + empty_rows.select{|empty_y| empty_y < y}.size * (modifier-1)
  points.each_with_index do |point, x|
    # shift x
    actual_x = x + empty_columns.select{|empty_x| empty_x < x}.size * (modifier-1)
    galaxies << [actual_x,actual_y] if point == '#'
  end
end

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
