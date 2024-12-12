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

#puts plots.inspect

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
mapping = {
  [0, -1] => [[1,0],[-1,0]],
  [0, 1] => [[1,0],[-1,0]],
  [1, 0] => [[0,-1],[0,1]],
  [-1, 0] => [[0,-1],[0,1]],
}
plots.each do |plot|
  outside = {}

  plot.each do |pos|
    directions.each do |dir|
      new_pos = [pos[0]+dir[0], pos[1]+dir[1]]
      unless plot.include?(new_pos)
        outside[dir] ||= Set.new
        outside[dir] << new_pos
      end
    end
  end
  lines = {
    [0, -1] => Set.new,
    [0, 1] => Set.new,
    [1, 0] => Set.new,
    [-1, 0] => Set.new
  }
  outside.each do |linedir, positions|
    positions.each do |pos|
      found = false
      mapping[linedir].each do |dir|
        new_pos = [pos[0]+dir[0], pos[1]+dir[1]]
        existing_line = lines[linedir].select{ |line| line.include?(new_pos) }.first
        if !existing_line.nil?
          existing_line << pos
          found = true
          break
        end
        #puts existing_line.inspect
      end
      if !found
        new_line = Set.new
        new_line << pos
        lines[linedir] << new_line
      end
    end
  end
  #puts lines.inspect
  # two borders are one off, uneven borders shouldn't be a thing
  borders = lines.map{|k,v| v.uniq.size}.sum / 2 * 2
  #puts "#{lines.map{|k,v| v.uniq.size}.sum} * #{plot.size}"
  result += borders * plot.size
end
puts "Part2: #{result}"
