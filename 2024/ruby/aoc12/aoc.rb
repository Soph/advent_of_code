require "set"

file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

directions = [[0,-1], [1,0], [0,1], [-1,0]]
looked_at = Set.new
plants = {}

lines.each_with_index do |line, y|
  line.split("").each_with_index do |position, x|
    plants[position] ||= Set.new
    plants[position] << [x,y]
  end
end

plots = Set.new
done = Set.new
plants.each do |plant, positions|
  positions.each do |pos|
    next if done.include?(pos)
    plot = Set.new
    plot << pos
    while true do
      new_plot = Set.new
      plot.each do |current|
        directions.each do |dir|
          new_pos = [current[0]+dir[0], current[1]+dir[1]]
          next if done.include?(new_pos)
          next if !plants[plant].include?(new_pos)
          done << new_pos
          new_plot << new_pos
        end
      end
      break if new_plot.size == 0
      plot += new_plot
    end
    plots << plot
  end
end

result = 0
plots.each do |plot|
  sides = 0
  plot.each do |pos|
    directions.each do |dir|
      new_pos = [pos[0]+dir[0], pos[1]+dir[1]]
      sides += 1 unless plot.include?(new_pos)
    end
  end
  result += sides * plot.size
end
puts "Part1: #{result}"

result = 0
plots.each do |plot|
  sides = 0
  plot.each do |pos|
    directions.each do |dir|
      new_pos = [pos[0]+dir[0], pos[1]+dir[1]]
      sides += 1 unless plot.include?(new_pos)
    end
  end
  result += sides * plot.size
end
puts "Part2: #{result}"
