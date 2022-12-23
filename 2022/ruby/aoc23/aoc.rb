require "set"
file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)
@trace = false

Elv = Struct.new(:x, :y) do
  def to_s
    "(#{x},#{y})"
  end

  def move(dx, dy)
    Elv.new(x + dx, y + dy)
  end

  def should_move?(elves)
    Elv.all_directions.each do |direction|
      return true if elves.include?(self.move(*direction))
    end
    return false
  end

  def find_new_direction(elves, start_direction)
    test_direction = start_direction
    4.times do
      puts "Checking: #{test_direction}" if @trace
      found = true
      Elv.direction_options[Elv.directions[test_direction]].each do |direction|
        found = false if elves.include?(self.move(*direction))
      end
      if found
        puts "Found direction: #{test_direction}" if @trace
        return test_direction
      end
      test_direction = (test_direction + 1) % 4
    end
    return nil
  end

  def self.directions
    [:n, :s, :w, :e]
  end

  def self.direction_options
    {
      n: [
        [0, -1],
        [1, -1],
        [-1, -1],
      ],
      s: [
        [0, 1],
        [1, 1],
        [-1, 1],
      ],
      w: [
        [-1, 0],
        [-1, -1],
        [-1, 1],
      ],
      e: [
        [1, 0],
        [1, -1],
        [1, 1],
      ]
    }
  end

  def self.all_directions
    self.direction_options.values.flatten(1).uniq
  end
end

def visualize(elves)
  count_empty = 0
  min = [0, 0, 0, 0]
  elves.each do |elv|
    min[0] = elv.x if elv.x < min[0]
    min[1] = elv.y if elv.y < min[1]
    min[2] = elv.x if elv.x > min[2]
    min[3] = elv.y if elv.y > min[3]
  end
  puts min.inspect
  (min[1]..min[3]).each do |y|
    line = []
    (min[0]..min[2]).each do |x|
      if elves.include?(Elv.new(x, y))
        line << "#"
      else
        count_empty += 1
        line << "."
      end
    end
    puts line.join("")
  end
  puts "----"
  puts "Empty: #{count_empty}"
end

elves = Set.new
i = 0
lines.each_with_index do |line, y|
  line.chars.each_with_index do |point, x|
    i += 1
    elves << Elv.new(x, y) if point == '#'
  end
end
visualize(elves)

last_directions = {}
start_direction = 0
rounds = 0
loop do
  new_elves = Set.new
  
  new_positions = {}
  elves.each do |elv|
    puts "Check Moving: #{elv}" if @trace
    if elv.should_move?(elves)
      direction = elv.find_new_direction(elves, start_direction)
      if direction.nil? # can't move
        new_elves << elv
      else  
        new_elv = elv.move(*Elv.direction_options[Elv.directions[direction]][0])
        new_positions[new_elv] ||= []
        new_positions[new_elv] << elv
      end
    else
      puts "does not need to move" if @trace
      new_elves << elv
    end
  end

  #puts new_positions.inspect
  new_positions.each do |target, sources|
    if sources.size == 1
      puts "Valid Move: #{sources[0]} -> #{target}" if @trace
      new_elves << target
    else
      puts "InValid Move: #{sources} -> #{target}" if @trace
      new_elves += sources
    end
  end
  break if elves == new_elves
  elves = new_elves
  start_direction = (start_direction + 1) % 4
  visualize(elves) if @trace
  rounds += 1
  puts "Round: #{rounds}"
end
visualize(elves)
puts "Rounds: #{rounds+1}"