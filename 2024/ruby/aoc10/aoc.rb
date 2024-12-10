require "set"

file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

trails = Set.new
numbers = {}
lines.each_with_index do |line, y|
  line.split("").each_with_index do |val, x|
    next if val == "."
    numbers[val.to_i] ||= Set.new
    numbers[val.to_i] << [x,y]
    trails << [[x,y]] if val.to_i == 0
  end
end

def next_options(current, options)
  options.select do |option|
    current[0] + 1 == option[0] && current[1] == option[1] ||
    current[0] - 1 == option[0] && current[1] == option[1] ||
    current[1] + 1 == option[1] && current[0] == option[0] ||
    current[1] - 1 == option[1] && current[0] == option[0]
  end
end

found = true
finished = Set.new
while !trails.empty? do
  found = false
  next_trails = Set.new
  trails.each do |trail|
    if trail.size == 10
      finished << trail
    else
      next_positions = next_options(trail.last, numbers[trail.size])
      next_positions.each do |position|
        new_trail = trail.clone
        new_trail << position
        next_trails << new_trail
      end
    end
  end
  trails = next_trails
end

finished_by_start = {}
finished.each do |fin|
  finished_by_start[fin[0]] ||= []
  finished_by_start[fin[0]] << fin
end

puts "Part1: #{finished_by_start.map{|key, val| val.map(&:last).uniq.size }.sum}"
puts "Part2: #{finished_by_start.map{|key, val| val.size }.sum}"