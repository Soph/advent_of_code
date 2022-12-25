require "set"
file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)
@trace = false

Point = Struct.new(:x, :y) do
  def to_s
    "(#{x},#{y})"
  end
end

Blizzard = Struct.new(:x, :y, :direction) do
  def to_s
    "(#{x},#{y}) #{Blizzard.directions[direction]}"
  end

  def to_blizchar
    Blizzard.directions[direction]
  end

  def self.directions
    ["^", ">", "<", "v"]
  end

  def self.movements
    [
      [0, -1],
      [1, 0],
      [-1, 0],
      [0, 1],
    ]
  end

  def move(mod_x, mod_y)
    movement = Blizzard.movements[direction]
    new_x = (self.x - 1 + movement[0]) % mod_x + 1
    new_y = (self.y - 1 + movement[1]) % mod_y + 1
    #puts "#{self.to_s} + #{movement} -> (#{new_x}, #{new_y}) (#{mod_x}, #{mod_y})"
    Blizzard.new(new_x, new_y, direction)
  end
end

def visualize(blizzards, walls, player)
  puts player.inspect
  count_empty = 0
  min = [0, 0, 0, 0]
  walls.each do |wall|
    min[0] = wall.x if wall.x < min[0]
    min[1] = wall.y if wall.y < min[1]
    min[2] = wall.x if wall.x > min[2]
    min[3] = wall.y if wall.y > min[3]
  end

  (min[1]..min[3]).each do |y|
    line = []
    (min[0]..min[2]).each do |x|
      blizs = blizzards.select{|blizzard| blizzard.x == x && blizzard.y == y}
      if blizs.size > 1
        line << "#{blizs.size}"
      elsif blizs.size == 1
        line << "#{blizs[0].to_blizchar}"
      elsif walls.include?(Point.new(x, y))
        line << "#"
      elsif !player.nil? && player == Point.new(x, y)
        line << "E"
      else
        line << "."
      end
    end
    line << "#"
    puts line.join("")
  end
  puts
end

def move_blizzards(blizzards)
  new_blizzards = Set.new
  mod_y = $playfield[3] - $playfield[1] + 1
  mod_x = $playfield[2] - $playfield[0] + 1
  blizzards.each do |bliz|
    new_bliz = bliz.move(mod_x, mod_y)
    new_blizzards << new_bliz
    #puts "#{bliz.to_s} -> #{new_bliz.to_s}"
  end
  new_blizzards
end

blizzards = Set.new
walls = Set.new
lines.each_with_index do |line, y|
  line.chars.each_with_index do |point, x|
    blizzards << Blizzard.new(x, y, Blizzard.directions.find_index(point)) if Blizzard.directions.include?(point)
    walls << Point.new(x, y) if point == '#'
  end
end

visualize(blizzards, walls, nil)
options = [{
  pos: Point.new(1, 0),
  blizzards: blizzards.clone,
  moves: 0,
  last_wait: false,
  positions: []
}]

$playfield = [walls.map(&:x).min + 1, walls.map(&:y).min + 1, walls.map(&:x).max - 1, walls.map(&:y).max - 1]
start = Point.new(1, 0)
target = Point.new(walls.map(&:x).max - 1, walls.map(&:y).max)

def pass(start, target, blizzards)
  moves = 0
  positions = [start]
    loop do 
    moves += 1
    new_positions = []
    new_blizzards = move_blizzards(blizzards)
    new_blizzard_positions = Set[*new_blizzards.map{|bliz| Point.new(bliz.x, bliz.y)}]
    positions.each do |current|
      [[0, -1],[1, 0],[-1, 0],[0, 1], [0, 0]].each do |diff|
        new_pos = Point.new(current.x + diff[0], current.y + diff[1])
        if new_pos == target
          return [moves, new_blizzards]
        end
        next if new_pos != start && new_pos != target && (new_pos.x < $playfield[0] || new_pos.y < $playfield[1] || new_pos.x > $playfield[2] || new_pos.y > $playfield[3])
        next if new_blizzard_positions.include?(new_pos)

        #puts "Adding: #{new_pos}"
        new_positions << new_pos
      end
      #sleep 1
    end
    positions = new_positions.uniq
    blizzards = new_blizzards
    
    puts "Positions: #{positions.size} move: #{moves}"
    #sleep 1
  end
end

total_moves = 0
moves, pass1 = pass(start, target, blizzards)
puts "Result 1: #{moves}"
total_moves += moves
moves, pass2 = pass(target, start, pass1)
puts "Back: #{moves}"
total_moves += moves
moves, pass3 = pass(start, target, pass2)
puts "Again: #{moves}"
total_moves += moves

puts "Result2: #{total_moves}"